use serde::{Deserialize, Serialize};

use crate::datatypes::{
    data_field_from_desc, default_true, find_code_for_name, get_remaining_subfields,
    get_subfield_by_names, ind_to_nonfiling_chars, is_true, is_zero, known_codes_from_map,
    nonfiling_chars_to_ind, push_subfield_by_names,
};
use crate::formats::{SubfieldMapping, TagDescriptor};
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleStatementData {
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub title_added_entry: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub nonfiling_chars: u8,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_title_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_responsibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_responsibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_part: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_of_part: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl TitleStatementData {
    pub fn from_subfields_with_map(ind1: char, ind2: char, subfields: &[(char, String)], map: &[SubfieldMapping]) -> Option<Self> {
        let title = get_subfield_by_names(subfields, map, &["title"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            title_added_entry: ind1 != '0',
            nonfiling_chars: ind_to_nonfiling_chars(ind2),
            title,
            remainder: get_subfield_by_names(subfields, map, &["remainder"]),
            responsibility: get_subfield_by_names(subfields, map, &["responsibility"]),
            other_title_info: get_subfield_by_names(subfields, map, &["other_title_info", "other_title_information", "other_title_info_2"]),
            first_responsibility: get_subfield_by_names(subfields, map, &["first_responsibility"]),
            other_responsibility: get_subfield_by_names(subfields, map, &["other_responsibility"]),
            medium: get_subfield_by_names(subfields, map, &["medium"]),
            number_of_part: get_subfield_by_names(subfields, map, &["number_of_part"]),
            name_of_part: get_subfield_by_names(subfields, map, &["name_of_part"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let title_code = find_code_for_name(map, "title").unwrap_or('a');
        let mut out = vec![(title_code, self.title.clone())];
        push_subfield_by_names(&mut out, map, &["remainder"], &self.remainder);
        push_subfield_by_names(&mut out, map, &["responsibility"], &self.responsibility);
        push_subfield_by_names(&mut out, map, &["other_title_info", "other_title_information"], &self.other_title_info);
        push_subfield_by_names(&mut out, map, &["first_responsibility"], &self.first_responsibility);
        push_subfield_by_names(&mut out, map, &["other_responsibility"], &self.other_responsibility);
        push_subfield_by_names(&mut out, map, &["medium"], &self.medium);
        push_subfield_by_names(&mut out, map, &["number_of_part"], &self.number_of_part);
        push_subfield_by_names(&mut out, map, &["name_of_part"], &self.name_of_part);
        out.extend(self.other_subfields.clone());
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleData {
    #[serde(default, skip_serializing_if = "is_zero")]
    pub nonfiling_chars: u8,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl TitleData {
    pub fn from_subfields_with_map(ind2: char, subfields: &[(char, String)], map: &[SubfieldMapping]) -> Option<Self> {
        let title = get_subfield_by_names(subfields, map, &["title"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            nonfiling_chars: ind_to_nonfiling_chars(ind2),
            title,
            remainder: get_subfield_by_names(subfields, map, &["remainder"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let title_code = find_code_for_name(map, "title").unwrap_or('a');
        let mut out = vec![(title_code, self.title.clone())];
        push_subfield_by_names(&mut out, map, &["remainder"], &self.remainder);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Title and title-related fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Title {
    TitleStatement(TitleStatementData),
    VaryingFormOfTitle(TitleData),
    FormerTitle(TitleData),
    ParallelTitle(TitleData),
    OtherTitleInformation(TitleData),
}

impl Title {
    /// Format-agnostic parse: uses the TagDescriptor's subfield_map.
    pub fn from_desc(
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        desc: &TagDescriptor,
    ) -> Option<Self> {
        match desc.field {
            "title_statement" | "title_statement_obsolete" => {
                TitleStatementData::from_subfields_with_map(ind1, ind2, subfields, desc.subfield_map)
                    .map(Title::TitleStatement)
            }
            _ => TitleStatementData::from_subfields_with_map(ind1, ind2, subfields, desc.subfield_map)
                .map(Title::TitleStatement),
        }
    }

    /// Format-agnostic serialization: uses TagDescriptor for tag and subfield codes.
    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        match self {
            Title::TitleStatement(d) => {
                let ind1 = if d.title_added_entry { '1' } else { '0' };
                let ind2 = nonfiling_chars_to_ind(d.nonfiling_chars);
                data_field_from_desc(desc, ind1, ind2, d.to_subfields_with_map(desc.subfield_map))
            }
            Title::VaryingFormOfTitle(d) => {
                data_field_from_desc(desc, '1', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields_with_map(desc.subfield_map))
            }
            Title::FormerTitle(d) => {
                data_field_from_desc(desc, '0', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields_with_map(desc.subfield_map))
            }
            Title::ParallelTitle(d) | Title::OtherTitleInformation(d) => {
                data_field_from_desc(desc, '1', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields_with_map(desc.subfield_map))
            }
        }
    }
}
