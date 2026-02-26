//! Language code fields — 041 (MARC21). UNIMARC 101 is in Physical::AssociatedLanguage.

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Language codes — 041 (MARC21). $a repeatable (main language codes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageData {
    pub ind1: char,
    pub ind2: char,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codes: Vec<String>,
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
        let codes: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'a').map(|(_, v)| v.clone()).collect();
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
        for c in &self.codes {
            out.push(('a', c.clone()));
        }
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, _format: MarcFormat) -> DataField {
        to_data_field("041", self.ind1, self.ind2, self.to_subfields())
    }
}
