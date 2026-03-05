use serde::{Deserialize, Serialize};

use crate::formats::SubfieldMapping;

use super::{
    find_code_for_name, get_remaining_subfields, get_subfield_by_names, ind_to_nonfiling_chars,
    is_zero, known_codes_from_map, push_subfield, push_subfield_by_names,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformTitleData {
    #[serde(default, skip_serializing_if = "is_zero")]
    pub nonfiling_chars: u8,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_work: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_of_part: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl UniformTitleData {
    pub fn from_subfields_with_map(
        ind1: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
    ) -> Option<Self> {
        let title = get_subfield_by_names(subfields, map, &["title"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            nonfiling_chars: ind_to_nonfiling_chars(ind1),
            title,
            date: get_subfield_by_names(subfields, map, &["date"]),
            date_of_work: get_subfield_by_names(subfields, map, &["date_of_work"]),
            language: get_subfield_by_names(subfields, map, &["language"]),
            number: get_subfield_by_names(subfields, map, &["number"]),
            name_of_part: get_subfield_by_names(subfields, map, &["name_of_part"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let title_code = find_code_for_name(map, "title").unwrap_or('a');
        let mut out = vec![(title_code, self.title.clone())];
        push_subfield_by_names(&mut out, map, &["date"], &self.date);
        push_subfield_by_names(&mut out, map, &["date_of_work"], &self.date_of_work);
        push_subfield_by_names(&mut out, map, &["language"], &self.language);
        push_subfield_by_names(&mut out, map, &["number"], &self.number);
        push_subfield_by_names(&mut out, map, &["name_of_part"], &self.name_of_part);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.title.clone())];
        push_subfield(&mut out, 'd', &self.date);
        push_subfield(&mut out, 'f', &self.date_of_work);
        push_subfield(&mut out, 'l', &self.language);
        push_subfield(&mut out, 'n', &self.number);
        push_subfield(&mut out, 'p', &self.name_of_part);
        out.extend(self.other_subfields.clone());
        out
    }
}
