use serde::{Deserialize, Serialize};

use crate::datatypes::{data_field_from_desc, find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map, push_subfield_by_names};
use crate::formats::TagDescriptor;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesStatementData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub traced: bool,
    pub statement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subseries: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl SeriesStatementData {
    pub fn from_subfields_with_map(ind1: char, subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        let statement = get_subfield_by_names(subfields, desc.subfield_map, &["statement"])?;
        let known = known_codes_from_map(desc.subfield_map);
        Some(Self {
            traced: ind1 == '1',
            statement,
            volume: get_subfield_by_names(subfields, desc.subfield_map, &["volume"]),
            issn: get_subfield_by_names(subfields, desc.subfield_map, &["issn"]),
            subseries: get_subfield_by_names(subfields, desc.subfield_map, &["subseries"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let code = find_code_for_name(desc.subfield_map, "statement").unwrap_or('a');
        let mut out = vec![(code, self.statement.clone())];
        push_subfield_by_names(&mut out, desc.subfield_map, &["subseries"], &self.subseries);
        push_subfield_by_names(&mut out, desc.subfield_map, &["volume"], &self.volume);
        push_subfield_by_names(&mut out, desc.subfield_map, &["issn"], &self.issn);
        out.extend(self.other_subfields.clone());
        let ind1 = if self.traced { '1' } else { '0' };
        data_field_from_desc(desc, ind1, ' ', out)
    }
}
