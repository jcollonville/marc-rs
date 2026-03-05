use serde::{Deserialize, Serialize};

use crate::datatypes::{
    data_field_from_desc, find_code_for_name, get_remaining_subfields, get_subfield_by_names,
    known_codes_from_map, push_subfield_by_names,
};
use crate::formats::TagDescriptor;
use crate::record::DataField;

/// Dewey edition type (MARC21 082/083 ind1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeweyEditionType {
    Full,
    Abridged,
    Other,
}

impl DeweyEditionType {
    pub fn from_ind1(ind1: char) -> Self {
        match ind1 {
            '0' => Self::Full,
            '1' => Self::Abridged,
            _ => Self::Other,
        }
    }

    pub fn to_ind1(&self) -> char {
        match self {
            Self::Full => '0',
            Self::Abridged => '1',
            Self::Other => '7',
        }
    }
}

impl Default for DeweyEditionType {
    fn default() -> Self {
        Self::Full
    }
}

/// Dewey Decimal Classification.
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
    pub fn first_number(&self) -> Option<&str> {
        self.numbers.first().map(String::as_str)
    }

    pub fn from_subfields_with_map(ind1: char, ind2: char, subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        let number_code = find_code_for_name(desc.subfield_map, "number").unwrap_or('a');
        let numbers: Vec<String> = subfields.iter().filter(|(c, _)| *c == number_code).map(|(_, v)| v.clone()).collect();
        if numbers.is_empty() {
            return None;
        }
        let known = known_codes_from_map(desc.subfield_map);
        Some(Self {
            is_additional: desc.field == "additional_dewey_classification",
            edition_type: DeweyEditionType::from_ind1(ind1),
            assigned_by_lc: match ind2 { '0' => Some(true), '4' => Some(false), _ => None },
            numbers,
            item_number: get_subfield_by_names(subfields, desc.subfield_map, &["item_number"]),
            edition: get_subfield_by_names(subfields, desc.subfield_map, &["edition"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let number_code = find_code_for_name(desc.subfield_map, "number").unwrap_or('a');
        let mut out: Vec<(char, String)> = self.numbers.iter().map(|n| (number_code, n.clone())).collect();
        push_subfield_by_names(&mut out, desc.subfield_map, &["item_number"], &self.item_number);
        push_subfield_by_names(&mut out, desc.subfield_map, &["edition"], &self.edition);
        out.extend(self.other_subfields.clone());
        let ind1 = self.edition_type.to_ind1();
        let ind2 = match self.assigned_by_lc { Some(true) => '0', Some(false) => '4', None => ' ' };
        data_field_from_desc(desc, ind1, ind2, out)
    }
}
