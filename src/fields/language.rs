//! Language code fields — 041 (MARC21). UNIMARC 101 is in Physical::AssociatedLanguage.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Language enumeration for common MARC / ISO 639-2 language codes.
///
/// Serialized as the 3-letter code (e.g. `"fre"`, `"eng"`).
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
    /// Any other 3-letter MARC/ISO code not covered above.
    Other(String),
}

impl LanguageCode {
    /// Create a `LanguageCode` from a 3-letter MARC / ISO 639-2 code.
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

    /// Preferred 3-letter code for MARC output.
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

/// Language codes — 041 (MARC21). $a repeatable (main language codes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<LanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl LanguageData {
    const KNOWN_CODES: [char; 8] = ['a', 'b', 'd', 'e', 'f', 'g', 'h', 'j'];

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        match (tag, format) {
            ("041", MarcFormat::Marc21 | MarcFormat::MarcXml) => {}
            _ => return None,
        }
        let codes: Vec<LanguageCode> = subfields
            .iter()
            .filter(|(c, _)| *c == 'a')
            .map(|(_, v)| LanguageCode::from_code(v))
            .collect();
        let other_subfields = get_remaining_subfields(subfields, &Self::KNOWN_CODES);
        Some(Self {
            ind1,
            ind2,
            codes,
            other_subfields,
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = Vec::new();
        for lang in &self.codes {
            out.push(('a', lang.as_code().to_string()));
        }
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, _format: MarcFormat) -> DataField {
        to_data_field("041", self.ind1, self.ind2, self.to_subfields())
    }

    pub fn push_language(&mut self, lang: LanguageCode) {
        self.codes.push(lang);
    }
}
