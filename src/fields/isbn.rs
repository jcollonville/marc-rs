//! ISBN — 010 (UNIMARC) and 020 (MARC21).
//! Field is repeatable (e.g. ISBN-10 + ISBN-13, or different bindings).

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// One ISBN occurrence. Raw number in `number`; use `sanitized_number()` for digits + X only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Isbn {
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
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_or_acquisition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_invalid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl Isbn {
    const KNOWN_UNIMARC: [char; 4] = ['a', 'b', 'd', 'z'];
    const KNOWN_MARC21: [char; 4] = ['a', 'q', 'c', 'z'];

    /// Sanitize ISBN: keep only digits and final 'X' (ISBN-10). Removes hyphens and extra text.
    /// Example: "978-2-07-014641-3 (br.)" → "9782070146413".
    pub fn sanitized_number(&self) -> String {
        sanitize_isbn(&self.number)
    }

    /// Parse one 010 (UNIMARC) or 020 (MARC21) field.
    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        let (tag_ok, qualification_code, price_code, known) = match (tag, format) {
            ("010", MarcFormat::Unimarc) => (true, 'b', 'd', &Self::KNOWN_UNIMARC[..]),
            ("020", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                (true, 'q', 'c', &Self::KNOWN_MARC21[..])
            }
            _ => (false, '\0', '\0', &[][..]),
        };
        if !tag_ok {
            return None;
        }
        let number = get_subfield(subfields, 'a').unwrap_or_default();
        Some(Self {
            ind1,
            ind2,
            number,
            qualification: get_subfield(subfields, qualification_code),
            price_or_acquisition: get_subfield(subfields, price_code),
            cancelled_invalid: get_subfield(subfields, 'z'),
            other_subfields: get_remaining_subfields(subfields, known),
        })
    }

    fn to_subfields(&self, format: MarcFormat) -> Vec<(char, String)> {
        let mut out = vec![('a', self.number.clone())];
        match format {
            MarcFormat::Unimarc => {
                push_subfield(&mut out, 'b', &self.qualification);
                push_subfield(&mut out, 'd', &self.price_or_acquisition);
            }
            MarcFormat::Marc21 | MarcFormat::MarcXml => {
                push_subfield(&mut out, 'q', &self.qualification);
                push_subfield(&mut out, 'c', &self.price_or_acquisition);
            }
        }
        push_subfield(&mut out, 'z', &self.cancelled_invalid);
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = match format {
            MarcFormat::Unimarc => "010",
            MarcFormat::Marc21 | MarcFormat::MarcXml => "020",
        };
        to_data_field(tag, self.ind1, self.ind2, self.to_subfields(format))
    }
}

/// Sanitize a raw ISBN string: keep only digits and the letter X (for ISBN-10 check digit).
pub fn sanitize_isbn(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_ascii_digit() || c.eq_ignore_ascii_case(&'X'))
        .map(|c| if c == 'x' { 'X' } else { c })
        .collect()
}
