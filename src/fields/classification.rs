//! Classification fields — Dewey (082/083 MARC21, 676 UNIMARC).

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Dewey Decimal Classification — 082 (primary) or 083 (additional) in MARC21, 676 in UNIMARC.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeweyClassification {
    pub tag: String,
    pub ind1: char,
    pub ind2: char,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl DeweyClassification {
    /// First classification number, if any.
    pub fn first_number(&self) -> Option<&str> {
        self.numbers.first().map(String::as_str)
    }

    const KNOWN_082_083: [char; 3] = ['a', 'b', '2'];
    const KNOWN_676: [char; 2] = ['a', 'v'];

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        let (tag_ok, edition_code) = match (tag, format) {
            ("082", MarcFormat::Marc21 | MarcFormat::MarcXml) => (true, '2'),
            ("083", MarcFormat::Marc21 | MarcFormat::MarcXml) => (true, '2'),
            ("676", MarcFormat::Unimarc) => (true, 'v'),
            _ => (false, '\0'),
        };
        if !tag_ok {
            return None;
        }
        let numbers: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'a').map(|(_, v)| v.clone()).collect();
        if numbers.is_empty() {
            return None;
        }
        let (item_number, known) = if tag == "676" {
            (None, &Self::KNOWN_676[..])
        } else {
            (get_subfield(subfields, 'b'), &Self::KNOWN_082_083[..])
        };
        let edition = get_subfield(subfields, edition_code);
        let other_subfields = get_remaining_subfields(subfields, known);
        Some(Self {
            tag: tag.to_string(),
            ind1,
            ind2,
            numbers,
            item_number,
            edition,
            other_subfields,
        })
    }

    fn to_subfields(&self, _format: MarcFormat) -> Vec<(char, String)> {
        let mut out = Vec::new();
        for n in &self.numbers {
            out.push(('a', n.clone()));
        }
        push_subfield(&mut out, 'b', &self.item_number);
        if self.tag == "676" {
            push_subfield(&mut out, 'v', &self.edition);
        } else {
            push_subfield(&mut out, '2', &self.edition);
        }
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        to_data_field(&self.tag, self.ind1, self.ind2, self.to_subfields(format))
    }
}
