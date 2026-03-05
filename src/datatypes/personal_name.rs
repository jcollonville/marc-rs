use serde::{Deserialize, Serialize};

use crate::formats::{FormatDescriptor, SubfieldMapping};

use super::{
    find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map,
    push_subfield, push_subfield_by_names,
};

/// Type of personal name entry element (MARC21 1XX/7XX ind1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonalNameType {
    Forename,
    Surname,
    FamilyName,
}

impl Default for PersonalNameType {
    fn default() -> Self {
        Self::Surname
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalNameData {
    #[serde(default)]
    pub name_type: PersonalNameType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuller_form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates_of_work: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl PersonalNameData {
    pub fn from_subfields_with_map(
        ind1: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
        descriptor: &dyn FormatDescriptor,
    ) -> Option<Self> {
        let name = get_subfield_by_names(subfields, map, &["name", "surname"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            name_type: descriptor.personal_name_type_from_ind1(ind1),
            name,
            numeration: get_subfield_by_names(subfields, map, &["numeration", "forenames", "roman_numerals"]),
            titles: get_subfield_by_names(subfields, map, &["titles", "additions_to_name"]),
            dates: get_subfield_by_names(subfields, map, &["dates"]),
            relator_term: get_subfield_by_names(subfields, map, &["relator_term"]),
            fuller_form: get_subfield_by_names(subfields, map, &["fuller_form"]),
            relator_code: get_subfield_by_names(subfields, map, &["relator_code"]),
            authority_number: get_subfield_by_names(subfields, map, &["authority_number"]),
            dates_of_work: get_subfield_by_names(subfields, map, &["dates_of_work"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let name_code = find_code_for_name(map, "name")
            .or_else(|| find_code_for_name(map, "surname"))
            .unwrap_or('a');
        let mut out = vec![(name_code, self.name.clone())];
        push_subfield_by_names(&mut out, map, &["numeration", "forenames"], &self.numeration);
        push_subfield_by_names(&mut out, map, &["titles", "additions_to_name"], &self.titles);
        push_subfield_by_names(&mut out, map, &["dates"], &self.dates);
        push_subfield_by_names(&mut out, map, &["relator_term"], &self.relator_term);
        push_subfield_by_names(&mut out, map, &["dates_of_work"], &self.dates_of_work);
        push_subfield_by_names(&mut out, map, &["fuller_form"], &self.fuller_form);
        push_subfield_by_names(&mut out, map, &["authority_number"], &self.authority_number);
        push_subfield_by_names(&mut out, map, &["relator_code"], &self.relator_code);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.name.clone())];
        push_subfield(&mut out, 'b', &self.numeration);
        push_subfield(&mut out, 'c', &self.titles);
        push_subfield(&mut out, 'd', &self.dates);
        push_subfield(&mut out, 'e', &self.relator_term);
        push_subfield(&mut out, 'f', &self.dates_of_work);
        push_subfield(&mut out, 'q', &self.fuller_form);
        push_subfield(&mut out, '3', &self.authority_number);
        push_subfield(&mut out, '4', &self.relator_code);
        out.extend(self.other_subfields.clone());
        out
    }
}
