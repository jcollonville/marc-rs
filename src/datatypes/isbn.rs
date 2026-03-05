use serde::{Deserialize, Serialize};

use crate::datatypes::{data_field_from_desc, find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map, push_subfield_by_names};
use crate::formats::TagDescriptor;
use crate::record::DataField;

/// One ISBN occurrence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Isbn {
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
    pub fn sanitized_number(&self) -> String {
        sanitize_isbn(&self.number)
    }

    /// Format-agnostic parse via SubfieldMapping.
    pub fn from_subfields_with_map(subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        let number = get_subfield_by_names(subfields, desc.subfield_map, &["value"])?;
        let known = known_codes_from_map(desc.subfield_map);
        Some(Self {
            number,
            qualification: get_subfield_by_names(subfields, desc.subfield_map, &["qualification"]),
            price_or_acquisition: get_subfield_by_names(subfields, desc.subfield_map, &["price_or_acquisition"]),
            cancelled_invalid: get_subfield_by_names(subfields, desc.subfield_map, &["canceled_invalid"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    /// Format-agnostic serialization via TagDescriptor.
    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let mut out = Vec::new();
        if let Some(code) = find_code_for_name(desc.subfield_map, "value") {
            out.push((code, self.number.clone()));
        } else {
            out.push(('a', self.number.clone()));
        }
        push_subfield_by_names(&mut out, desc.subfield_map, &["qualification"], &self.qualification);
        push_subfield_by_names(&mut out, desc.subfield_map, &["price_or_acquisition"], &self.price_or_acquisition);
        push_subfield_by_names(&mut out, desc.subfield_map, &["canceled_invalid"], &self.cancelled_invalid);
        out.extend(self.other_subfields.clone());
        data_field_from_desc(desc, ' ', ' ', out)
    }
}

pub fn sanitize_isbn(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_ascii_digit() || c.eq_ignore_ascii_case(&'X'))
        .map(|c| if c == 'x' { 'X' } else { c })
        .collect()
}
