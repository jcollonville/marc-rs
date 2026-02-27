//! Classification fields — Dewey (082/083 MARC21, 676 UNIMARC).

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Dewey Decimal Classification — 082 (primary) or 083 (additional) in MARC21, 676 in UNIMARC.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeweyClassification {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_additional: bool,
    #[serde(default)]
    pub edition_type: DeweyEditionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_by_lc: Option<bool>,
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
        let edition_type = match format {
            MarcFormat::Marc21 | MarcFormat::MarcXml => DeweyEditionType::from_ind1(ind1),
            MarcFormat::Unimarc => DeweyEditionType::Full,
        };
        let assigned_by_lc = match (format, ind2) {
            (MarcFormat::Marc21 | MarcFormat::MarcXml, '0') => Some(true),
            (MarcFormat::Marc21 | MarcFormat::MarcXml, '4') => Some(false),
            _ => None,
        };
        let edition = get_subfield(subfields, edition_code);
        let other_subfields = get_remaining_subfields(subfields, known);
        Some(Self {
            is_additional: tag == "083",
            edition_type,
            assigned_by_lc,
            numbers,
            item_number,
            edition,
            other_subfields,
        })
    }

    fn to_subfields(&self, format: MarcFormat) -> Vec<(char, String)> {
        let mut out = Vec::new();
        for n in &self.numbers {
            out.push(('a', n.clone()));
        }
        push_subfield(&mut out, 'b', &self.item_number);
        match format {
            MarcFormat::Unimarc => push_subfield(&mut out, 'v', &self.edition),
            MarcFormat::Marc21 | MarcFormat::MarcXml => push_subfield(&mut out, '2', &self.edition),
        }
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = match format {
            MarcFormat::Unimarc => "676",
            MarcFormat::Marc21 | MarcFormat::MarcXml => {
                if self.is_additional { "083" } else { "082" }
            }
        };
        let ind1 = match format {
            MarcFormat::Marc21 | MarcFormat::MarcXml => self.edition_type.to_ind1(),
            MarcFormat::Unimarc => ' ',
        };
        let ind2 = match (format, self.assigned_by_lc) {
            (MarcFormat::Marc21 | MarcFormat::MarcXml, Some(true)) => '0',
            (MarcFormat::Marc21 | MarcFormat::MarcXml, Some(false)) => '4',
            _ => ' ',
        };
        to_data_field(tag, ind1, ind2, self.to_subfields(format))
    }
}
