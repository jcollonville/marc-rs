use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditionStatementData {
    pub ind1: char,
    pub ind2: char,
    pub edition: String,
    pub remainder: Option<String>,
    pub other_subfields: Vec<(char, String)>,
}

impl EditionStatementData {
    const KNOWN_CODES: [char; 2] = ['a', 'b'];

    fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let edition = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            edition,
            remainder: get_subfield(subfields, 'b'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.edition.clone())];
        push_subfield(&mut out, 'b', &self.remainder);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Edition fields (25X in MARC21, 2XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Edition {
    /// Edition statement (250 in MARC21, 205 in UNIMARC)
    EditionStatement(EditionStatementData),
    /// Musical presentation statement (254 in MARC21)
    MusicalPresentationStatement(NoteData),
    /// Cartographic mathematical data (255 in MARC21, 206 in UNIMARC)
    CartographicMathematicalData(NoteData),
    /// Computer file characteristics (256 in MARC21, 336 in UNIMARC)
    ComputerFileCharacteristics(NoteData),
    /// Country of producing entity (257 in MARC21)
    CountryOfProducingEntity(NoteData),
    /// Philatelic issue data (258 in MARC21)
    PhilatelicIssueData(NoteData),
}

impl Edition {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Edition::EditionStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("250"),
            (Edition::EditionStatement(_), MarcFormat::Unimarc) => Some("205"),
            (Edition::MusicalPresentationStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("254"),
            (Edition::MusicalPresentationStatement(_), MarcFormat::Unimarc) => None,
            (Edition::CartographicMathematicalData(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("255"),
            (Edition::CartographicMathematicalData(_), MarcFormat::Unimarc) => Some("206"),
            (Edition::ComputerFileCharacteristics(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("256"),
            (Edition::ComputerFileCharacteristics(_), MarcFormat::Unimarc) => Some("336"),
            (Edition::CountryOfProducingEntity(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("257"),
            (Edition::CountryOfProducingEntity(_), MarcFormat::Unimarc) => None,
            (Edition::PhilatelicIssueData(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("258"),
            (Edition::PhilatelicIssueData(_), MarcFormat::Unimarc) => None,
        }
    }

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        match (tag, format) {
            ("250", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("205", MarcFormat::Unimarc) => {
                EditionStatementData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::EditionStatement)
            }
            ("254", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::MusicalPresentationStatement)
            }
            ("255", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("206", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::CartographicMathematicalData)
            }
            ("256", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("336", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::ComputerFileCharacteristics)
            }
            ("257", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::CountryOfProducingEntity)
            }
            ("258", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(Edition::PhilatelicIssueData)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        let df = match self {
            Edition::EditionStatement(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            Edition::MusicalPresentationStatement(d)
            | Edition::CartographicMathematicalData(d)
            | Edition::ComputerFileCharacteristics(d)
            | Edition::CountryOfProducingEntity(d)
            | Edition::PhilatelicIssueData(d) => {
                to_data_field(tag, d.ind1, d.ind2, d.to_subfields())
            }
        };
        Some(df)
    }
}
