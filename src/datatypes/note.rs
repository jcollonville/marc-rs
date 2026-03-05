use serde::{Deserialize, Serialize};

use crate::formats::SubfieldMapping;

use super::{find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map};

/// Generic data struct for notes and simple fields with mainly $a text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteData {
    pub text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl NoteData {
    pub fn from_subfields_with_map(
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
    ) -> Option<Self> {
        let text = get_subfield_by_names(subfields, map, &["text"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            text,
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let text_code = find_code_for_name(map, "text").unwrap_or('a');
        let mut out = vec![(text_code, self.text.clone())];
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.text.clone())];
        out.extend(self.other_subfields.clone());
        out
    }
}
