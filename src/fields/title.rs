use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleStatementData {
    #[serde(
        default = "crate::fields::common::default_true",
        skip_serializing_if = "crate::fields::common::is_true"
    )]
    pub title_added_entry: bool,
    #[serde(default, skip_serializing_if = "crate::fields::common::is_zero")]
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
    const KNOWN_CODES: [char; 9] = ['a', 'b', 'c', 'e', 'f', 'g', 'h', 'n', 'p'];

    fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let title = get_subfield(subfields, 'a')?;
        Some(Self {
            title_added_entry: ind1 != '0',
            nonfiling_chars: ind_to_nonfiling_chars(ind2),
            title,
            remainder: get_subfield(subfields, 'b'),
            responsibility: get_subfield(subfields, 'c'),
            other_title_info: get_subfield(subfields, 'e'),
            first_responsibility: get_subfield(subfields, 'f'),
            other_responsibility: get_subfield(subfields, 'g'),
            medium: get_subfield(subfields, 'h'),
            number_of_part: get_subfield(subfields, 'n'),
            name_of_part: get_subfield(subfields, 'p'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.title.clone())];
        push_subfield(&mut out, 'b', &self.remainder);
        push_subfield(&mut out, 'c', &self.responsibility);
        push_subfield(&mut out, 'e', &self.other_title_info);
        push_subfield(&mut out, 'f', &self.first_responsibility);
        push_subfield(&mut out, 'g', &self.other_responsibility);
        push_subfield(&mut out, 'h', &self.medium);
        push_subfield(&mut out, 'n', &self.number_of_part);
        push_subfield(&mut out, 'p', &self.name_of_part);
        out.extend(self.other_subfields.clone());
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleData {
    #[serde(default, skip_serializing_if = "crate::fields::common::is_zero")]
    pub nonfiling_chars: u8,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl TitleData {
    const KNOWN_CODES: [char; 2] = ['a', 'b'];

    fn from_subfields(ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let title = get_subfield(subfields, 'a')?;
        Some(Self {
            nonfiling_chars: ind_to_nonfiling_chars(ind2),
            title,
            remainder: get_subfield(subfields, 'b'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.title.clone())];
        push_subfield(&mut out, 'b', &self.remainder);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Title and title-related fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Title {
    /// Title statement (245 in MARC21, 200 in UNIMARC)
    TitleStatement(TitleStatementData),
    /// Varying form of title (246 in MARC21, 517 in UNIMARC)
    VaryingFormOfTitle(TitleData),
    /// Former title (247 in MARC21, 520 in UNIMARC)
    FormerTitle(TitleData),
    /// Parallel title (246 in MARC21 variant, 510 in UNIMARC)
    ParallelTitle(TitleData),
    /// Other title information (246 in MARC21, 517 in UNIMARC)
    OtherTitleInformation(TitleData),
}

impl Title {
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (Title::TitleStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "245",
            (Title::TitleStatement(_), MarcFormat::Unimarc) => "200",
            (Title::VaryingFormOfTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "246",
            (Title::VaryingFormOfTitle(_), MarcFormat::Unimarc) => "517",
            (Title::FormerTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "247",
            (Title::FormerTitle(_), MarcFormat::Unimarc) => "520",
            (Title::ParallelTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "246",
            (Title::ParallelTitle(_), MarcFormat::Unimarc) => "510",
            (Title::OtherTitleInformation(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "246",
            (Title::OtherTitleInformation(_), MarcFormat::Unimarc) => "517",
        }
    }

    /// Parse a data field into a typed Title variant.
    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        match (tag, format) {
            ("245", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("200", MarcFormat::Unimarc) => {
                TitleStatementData::from_subfields(ind1, ind2, subfields)
                    .map(Title::TitleStatement)
            }
            ("246", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                TitleData::from_subfields(ind2, subfields)
                    .map(Title::VaryingFormOfTitle)
            }
            ("247", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                TitleData::from_subfields(ind2, subfields).map(Title::FormerTitle)
            }
            ("510", MarcFormat::Unimarc) => {
                TitleData::from_subfields(ind2, subfields).map(Title::ParallelTitle)
            }
            ("517", MarcFormat::Unimarc) => {
                TitleData::from_subfields(ind2, subfields)
                    .map(Title::VaryingFormOfTitle)
            }
            ("520", MarcFormat::Unimarc) => {
                TitleData::from_subfields(ind2, subfields).map(Title::FormerTitle)
            }
            _ => None,
        }
    }

    /// Convert back to raw DataField for writing.
    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = self.tag(format);
        match self {
            Title::TitleStatement(d) => {
                let ind1 = if d.title_added_entry { '1' } else { '0' };
                let ind2 = nonfiling_chars_to_ind(d.nonfiling_chars);
                to_data_field(tag, ind1, ind2, d.to_subfields())
            }
            Title::VaryingFormOfTitle(d) => {
                to_data_field(tag, '1', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields())
            }
            Title::FormerTitle(d) => {
                to_data_field(tag, '0', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields())
            }
            Title::ParallelTitle(d) => {
                to_data_field(tag, '1', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields())
            }
            Title::OtherTitleInformation(d) => {
                to_data_field(tag, '1', nonfiling_chars_to_ind(d.nonfiling_chars), d.to_subfields())
            }
        }
    }
}
