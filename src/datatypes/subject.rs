use serde::{Deserialize, Serialize};

use crate::formats::SubfieldMapping;

use super::{
    find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map,
    push_subfield, push_subfield_by_names,
};

/// Subject heading thesaurus / system (MARC21 6XX ind2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectThesaurus {
    Lcsh,
    LcChildrens,
    Mesh,
    NationalBibliography,
    NotSpecified,
    Canadian,
    Repertoire,
    SourceSpecified,
}

impl SubjectThesaurus {
    pub fn from_ind2(ind2: char) -> Self {
        match ind2 {
            '0' => Self::Lcsh,
            '1' => Self::LcChildrens,
            '2' => Self::Mesh,
            '3' => Self::NationalBibliography,
            '5' => Self::Canadian,
            '6' => Self::Repertoire,
            '7' => Self::SourceSpecified,
            _ => Self::NotSpecified,
        }
    }

    pub fn to_ind2(&self) -> char {
        match self {
            Self::Lcsh => '0',
            Self::LcChildrens => '1',
            Self::Mesh => '2',
            Self::NationalBibliography => '3',
            Self::NotSpecified => '4',
            Self::Canadian => '5',
            Self::Repertoire => '6',
            Self::SourceSpecified => '7',
        }
    }
}

impl Default for SubjectThesaurus {
    fn default() -> Self {
        Self::NotSpecified
    }
}

/// Generic data struct for subject access fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectData {
    #[serde(default)]
    pub thesaurus: SubjectThesaurus,
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
    pub fn from_subfields_with_map(
        ind2: char,
        subfields: &[(char, String)],
        map: &[SubfieldMapping],
    ) -> Option<Self> {
        let term = get_subfield_by_names(subfields, map, &["term"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            thesaurus: SubjectThesaurus::from_ind2(ind2),
            term,
            name_subdivision: get_subfield_by_names(subfields, map, &["name_subdivision"]),
            form_subdivision: get_subfield_by_names(subfields, map, &["form_subdivision"]),
            general_subdivision: get_subfield_by_names(subfields, map, &["general_subdivision", "general_subdivision_2"]),
            chronological_subdivision: get_subfield_by_names(subfields, map, &["chronological_subdivision"]),
            geographic_subdivision: get_subfield_by_names(subfields, map, &["geographic_subdivision"]),
            source: get_subfield_by_names(subfields, map, &["source"]),
            authority_number: get_subfield_by_names(subfields, map, &["authority_number"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let term_code = find_code_for_name(map, "term").unwrap_or('a');
        let mut out = vec![(term_code, self.term.clone())];
        push_subfield_by_names(&mut out, map, &["name_subdivision"], &self.name_subdivision);
        push_subfield_by_names(&mut out, map, &["form_subdivision"], &self.form_subdivision);
        push_subfield_by_names(&mut out, map, &["general_subdivision"], &self.general_subdivision);
        push_subfield_by_names(&mut out, map, &["chronological_subdivision"], &self.chronological_subdivision);
        push_subfield_by_names(&mut out, map, &["geographic_subdivision"], &self.geographic_subdivision);
        push_subfield_by_names(&mut out, map, &["source"], &self.source);
        push_subfield_by_names(&mut out, map, &["authority_number"], &self.authority_number);
        out.extend(self.other_subfields.clone());
        out
    }

    /// Fallback serializer for tagged entries that already store their raw tag.
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
