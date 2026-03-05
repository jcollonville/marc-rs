use serde::{Deserialize, Serialize};

use crate::formats::{FormatDescriptor, SubfieldMapping};

use super::{
    find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map,
    push_subfield, push_subfield_by_names,
};

/// Type of organization (corporate / meeting) name entry (MARC21 1XX/7XX ind1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationNameType {
    InvertedName,
    JurisdictionName,
    DirectOrder,
}

impl Default for OrganizationNameType {
    fn default() -> Self {
        Self::DirectOrder
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateNameData {
    #[serde(default)]
    pub name_type: OrganizationNameType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl CorporateNameData {
    pub fn from_subfields_with_map(
        ind1: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
        descriptor: &dyn FormatDescriptor,
    ) -> Option<Self> {
        let name = get_subfield_by_names(subfields, map, &["name"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            name_type: descriptor.organization_name_type_from_ind1(ind1),
            name,
            subordinate_unit: get_subfield_by_names(subfields, map, &["subordinate_unit"]),
            location: get_subfield_by_names(subfields, map, &["location"]),
            date: get_subfield_by_names(subfields, map, &["date"]),
            relator_term: get_subfield_by_names(subfields, map, &["relator_term"]),
            relator_code: get_subfield_by_names(subfields, map, &["relator_code"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let name_code = find_code_for_name(map, "name").unwrap_or('a');
        let mut out = vec![(name_code, self.name.clone())];
        push_subfield_by_names(&mut out, map, &["subordinate_unit"], &self.subordinate_unit);
        push_subfield_by_names(&mut out, map, &["location"], &self.location);
        push_subfield_by_names(&mut out, map, &["date"], &self.date);
        push_subfield_by_names(&mut out, map, &["relator_term"], &self.relator_term);
        push_subfield_by_names(&mut out, map, &["relator_code"], &self.relator_code);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.name.clone())];
        push_subfield(&mut out, 'b', &self.subordinate_unit);
        push_subfield(&mut out, 'c', &self.location);
        push_subfield(&mut out, 'd', &self.date);
        push_subfield(&mut out, 'e', &self.relator_term);
        push_subfield(&mut out, '4', &self.relator_code);
        out.extend(self.other_subfields.clone());
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingNameData {
    #[serde(default)]
    pub name_type: OrganizationNameType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl MeetingNameData {
    pub fn from_subfields_with_map(
        ind1: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
        descriptor: &dyn FormatDescriptor,
    ) -> Option<Self> {
        let name = get_subfield_by_names(subfields, map, &["name"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            name_type: descriptor.organization_name_type_from_ind1(ind1),
            name,
            location: get_subfield_by_names(subfields, map, &["location"]),
            date: get_subfield_by_names(subfields, map, &["date"]),
            subordinate_unit: get_subfield_by_names(subfields, map, &["subordinate_unit"]),
            number: get_subfield_by_names(subfields, map, &["number"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let name_code = find_code_for_name(map, "name").unwrap_or('a');
        let mut out = vec![(name_code, self.name.clone())];
        push_subfield_by_names(&mut out, map, &["location"], &self.location);
        push_subfield_by_names(&mut out, map, &["date"], &self.date);
        push_subfield_by_names(&mut out, map, &["subordinate_unit"], &self.subordinate_unit);
        push_subfield_by_names(&mut out, map, &["number"], &self.number);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.name.clone())];
        push_subfield(&mut out, 'c', &self.location);
        push_subfield(&mut out, 'd', &self.date);
        push_subfield(&mut out, 'e', &self.subordinate_unit);
        push_subfield(&mut out, 'n', &self.number);
        out.extend(self.other_subfields.clone());
        out
    }
}
