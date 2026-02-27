use serde::{Deserialize, Serialize};

use crate::record::{ControlField, DataField, Subfield};

pub fn get_subfield(subfields: &[(char, String)], code: char) -> Option<String> {
    subfields
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, v)| v.clone())
}

/// Returns subfields not consumed by named fields.
/// Skips the first occurrence of each known code (captured by get_subfield),
/// keeps duplicates and unknown codes.
pub fn get_remaining_subfields(
    subfields: &[(char, String)],
    known: &[char],
) -> Vec<(char, String)> {
    let mut seen: Vec<bool> = known.iter().map(|_| false).collect();
    let mut result = Vec::new();

    for (code, value) in subfields {
        if let Some(pos) = known.iter().position(|k| k == code) {
            if !seen[pos] {
                seen[pos] = true;
            } else {
                result.push((*code, value.clone()));
            }
        } else {
            result.push((*code, value.clone()));
        }
    }

    result
}

/// Default indicator value used when none is provided.
pub fn default_indicator() -> char {
    ' '
}

/// Helper for Serde: indicators that are the default (blank) are not serialized.
pub fn is_default_indicator(c: &char) -> bool {
    *c == ' ' || *c == '\0'
}

pub fn push_subfield(out: &mut Vec<(char, String)>, code: char, value: &Option<String>) {
    if let Some(ref v) = value {
        out.push((code, v.clone()));
    }
}

pub fn subfields_to_raw(subfields: &[(char, String)]) -> Vec<Subfield> {
    subfields
        .iter()
        .map(|(c, v)| Subfield {
            code: *c,
            value: v.clone(),
        })
        .collect()
}

pub fn to_control_field(tag: &str, value: &str) -> ControlField {
    ControlField {
        tag: tag.to_string(),
        value: value.to_string(),
    }
}

pub fn to_data_field(
    tag: &str,
    ind1: char,
    ind2: char,
    subfields: Vec<(char, String)>,
) -> DataField {
    DataField {
        tag: tag.to_string(),
        ind1,
        ind2,
        subfields: subfields_to_raw(&subfields),
    }
}

// Shared data structs for MainEntry and AddedEntry

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalNameData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
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
    const KNOWN_CODES: [char; 9] = ['a', 'b', 'c', 'd', 'e', 'f', 'q', '3', '4'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let name = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            name,
            numeration: get_subfield(subfields, 'b'),
            titles: get_subfield(subfields, 'c'),
            dates: get_subfield(subfields, 'd'),
            relator_term: get_subfield(subfields, 'e'),
            fuller_form: get_subfield(subfields, 'q'),
            relator_code: get_subfield(subfields, '4'),
            authority_number: get_subfield(subfields, '3'),
            dates_of_work: get_subfield(subfields, 'f'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateNameData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
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
    const KNOWN_CODES: [char; 6] = ['a', 'b', 'c', 'd', 'e', '4'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let name = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            name,
            subordinate_unit: get_subfield(subfields, 'b'),
            location: get_subfield(subfields, 'c'),
            date: get_subfield(subfields, 'd'),
            relator_term: get_subfield(subfields, 'e'),
            relator_code: get_subfield(subfields, '4'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

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
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
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
    const KNOWN_CODES: [char; 5] = ['a', 'c', 'd', 'e', 'n'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let name = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            name,
            location: get_subfield(subfields, 'c'),
            date: get_subfield(subfields, 'd'),
            subordinate_unit: get_subfield(subfields, 'e'),
            number: get_subfield(subfields, 'n'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformTitleData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
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
    const KNOWN_CODES: [char; 6] = ['a', 'd', 'f', 'l', 'n', 'p'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let title = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            title,
            date: get_subfield(subfields, 'd'),
            date_of_work: get_subfield(subfields, 'f'),
            language: get_subfield(subfields, 'l'),
            number: get_subfield(subfields, 'n'),
            name_of_part: get_subfield(subfields, 'p'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

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

/// Generic data struct for notes and simple fields with mainly $a text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
    pub text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl NoteData {
    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let text = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            text,
            other_subfields: get_remaining_subfields(subfields, &['a']),
        })
    }

    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.text.clone())];
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Generic data struct for subject fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
    pub term: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_subdivision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_subdivision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_subdivision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chronological_subdivision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic_subdivision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_number: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl SubjectData {
    const KNOWN_CODES: [char; 8] = ['a', 'b', 'v', 'x', 'y', 'z', '2', '3'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let term = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            term,
            name_subdivision: get_subfield(subfields, 'b'),
            form_subdivision: get_subfield(subfields, 'v'),
            general_subdivision: get_subfield(subfields, 'x'),
            chronological_subdivision: get_subfield(subfields, 'y'),
            geographic_subdivision: get_subfield(subfields, 'z'),
            source: get_subfield(subfields, '2'),
            authority_number: get_subfield(subfields, '3'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.term.clone())];
        push_subfield(&mut out, 'b', &self.name_subdivision);
        push_subfield(&mut out, 'v', &self.form_subdivision);
        push_subfield(&mut out, 'x', &self.general_subdivision);
        push_subfield(&mut out, 'y', &self.chronological_subdivision);
        push_subfield(&mut out, 'z', &self.geographic_subdivision);
        push_subfield(&mut out, '2', &self.source);
        push_subfield(&mut out, '3', &self.authority_number);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Generic data struct for linking entry fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkingData {
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl LinkingData {
    const KNOWN_CODES: [char; 5] = ['t', 'v', 'w', 'x', 'z'];

    pub fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Self {
        Self {
            ind1,
            ind2,
            title: get_subfield(subfields, 't'),
            record_control_number: get_subfield(subfields, 'w'),
            issn: get_subfield(subfields, 'x'),
            isbn: get_subfield(subfields, 'z'),
            volume: get_subfield(subfields, 'v'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        }
    }

    pub fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = Vec::new();
        push_subfield(&mut out, 't', &self.title);
        push_subfield(&mut out, 'v', &self.volume);
        push_subfield(&mut out, 'w', &self.record_control_number);
        push_subfield(&mut out, 'x', &self.issn);
        push_subfield(&mut out, 'z', &self.isbn);
        out.extend(self.other_subfields.clone());
        out
    }
}
