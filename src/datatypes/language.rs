use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::datatypes::{data_field_from_desc, find_code_for_name, get_remaining_subfields, known_codes_from_map};
use crate::formats::TagDescriptor;
use crate::record::DataField;

/// Language enumeration for common MARC / ISO 639-2 language codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageCode {
    Arabic,
    Chinese,
    Dutch,
    English,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Latin,
    Portuguese,
    Russian,
    Spanish,
    Other(String),
}

impl LanguageCode {
    pub fn from_code(code: &str) -> Self {
        match code.to_ascii_lowercase().as_str() {
            "ara" => LanguageCode::Arabic,
            "chi" | "zho" => LanguageCode::Chinese,
            "dut" | "nld" => LanguageCode::Dutch,
            "eng" => LanguageCode::English,
            "fre" | "fra" => LanguageCode::French,
            "ger" | "deu" => LanguageCode::German,
            "ita" => LanguageCode::Italian,
            "jpn" => LanguageCode::Japanese,
            "kor" => LanguageCode::Korean,
            "lat" => LanguageCode::Latin,
            "por" => LanguageCode::Portuguese,
            "rus" => LanguageCode::Russian,
            "spa" => LanguageCode::Spanish,
            _ => LanguageCode::Other(code.to_string()),
        }
    }

    pub fn as_code(&self) -> &str {
        match self {
            LanguageCode::Arabic => "ara",
            LanguageCode::Chinese => "chi",
            LanguageCode::Dutch => "dut",
            LanguageCode::English => "eng",
            LanguageCode::French => "fre",
            LanguageCode::German => "ger",
            LanguageCode::Italian => "ita",
            LanguageCode::Japanese => "jpn",
            LanguageCode::Korean => "kor",
            LanguageCode::Latin => "lat",
            LanguageCode::Portuguese => "por",
            LanguageCode::Russian => "rus",
            LanguageCode::Spanish => "spa",
            LanguageCode::Other(ref s) => s.as_str(),
        }
    }
}

impl Serialize for LanguageCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_code())
    }
}

impl<'de> Deserialize<'de> for LanguageCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(LanguageCode::from_code(&s))
    }
}

/// Language codes — 041 (MARC21), 101 (UNIMARC).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_translation: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<LanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl LanguageData {
    pub fn from_subfields_with_map(ind1: char, subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        let lang_code = find_code_for_name(desc.subfield_map, "language_code").unwrap_or('a');
        let known = known_codes_from_map(desc.subfield_map);
        let codes: Vec<LanguageCode> = subfields.iter()
            .filter(|(c, _)| *c == lang_code)
            .map(|(_, v)| LanguageCode::from_code(v))
            .collect();
        if codes.is_empty() {
            return None;
        }
        let is_translation = match ind1 {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
        };
        Some(Self {
            is_translation,
            codes,
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let lang_code = find_code_for_name(desc.subfield_map, "language_code").unwrap_or('a');
        let mut out: Vec<(char, String)> = self.codes.iter()
            .map(|l| (lang_code, l.as_code().to_string()))
            .collect();
        out.extend(self.other_subfields.clone());
        let ind1 = match self.is_translation {
            Some(false) => '0',
            Some(true) => '1',
            None => ' ',
        };
        data_field_from_desc(desc, ind1, ' ', out)
    }

    pub fn push_language(&mut self, lang: LanguageCode) {
        self.codes.push(lang);
    }
}
