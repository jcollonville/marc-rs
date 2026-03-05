use serde::{Deserialize, Serialize};

use crate::formats::SubfieldMapping;

use super::{
    default_true, get_remaining_subfields, get_subfield_by_names, is_true, known_codes_from_map,
    push_subfield, push_subfield_by_names,
};

/// Generic data struct for linking entry fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkingData {
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub display_note: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_control_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl LinkingData {
    pub fn from_subfields_with_map(
        ind1: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
    ) -> Self {
        let known = known_codes_from_map(map);
        Self {
            display_note: ind1 != '1',
            title: get_subfield_by_names(subfields, map, &["title"]),
            record_control_number: get_subfield_by_names(subfields, map, &["record_control_number"]),
            issn: get_subfield_by_names(subfields, map, &["issn"]),
            isbn: get_subfield_by_names(subfields, map, &["isbn"]),
            volume: get_subfield_by_names(subfields, map, &["volume"]),
            link_identifier: get_subfield_by_names(subfields, map, &["link_identifier"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        }
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let mut out = Vec::new();
        push_subfield_by_names(&mut out, map, &["title"], &self.title);
        push_subfield_by_names(&mut out, map, &["volume"], &self.volume);
        push_subfield_by_names(&mut out, map, &["record_control_number"], &self.record_control_number);
        push_subfield_by_names(&mut out, map, &["issn"], &self.issn);
        push_subfield_by_names(&mut out, map, &["isbn"], &self.isbn);
        push_subfield_by_names(&mut out, map, &["link_identifier"], &self.link_identifier);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = Vec::new();
        push_subfield(&mut out, 't', &self.title);
        push_subfield(&mut out, 'v', &self.volume);
        push_subfield(&mut out, 'w', &self.record_control_number);
        push_subfield(&mut out, 'x', &self.issn);
        push_subfield(&mut out, 'z', &self.isbn);
        push_subfield(&mut out, '1', &self.link_identifier);
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn display_note_ind1(&self) -> char {
        if self.display_note { '0' } else { '1' }
    }
}
