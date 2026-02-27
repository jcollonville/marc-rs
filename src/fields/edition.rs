use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditionStatementData {
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl EditionStatementData {
    const KNOWN_CODES: [char; 2] = ['a', 'b'];

    fn from_subfields(subfields: &[(char, String)]) -> Option<Self> {
        let edition = get_subfield(subfields, 'a')?;
        Some(Self {
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

/// Publication/imprint: place, publisher, date.
/// MARC21: 260 or 264 ($a place, $b publisher, $c date). UNIMARC: 210 ($a place, $c publisher, $d date).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rda: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<PublicationFunction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub places: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publishers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dates: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_places: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_dates: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl PublicationData {
    /// First place, or None.
    pub fn place(&self) -> Option<&str> {
        self.places.first().map(String::as_str)
    }
    /// First publisher, or None.
    pub fn publisher(&self) -> Option<&str> {
        self.publishers.first().map(String::as_str)
    }
    /// First date, or None.
    pub fn date(&self) -> Option<&str> {
        self.dates.first().map(String::as_str)
    }

    fn from_subfields_260_264(ind2: char, subfields: &[(char, String)]) -> Self {
        const KNOWN: [char; 3] = ['a', 'b', 'c'];
        let places: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'a').map(|(_, v)| v.clone()).collect();
        let publishers: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'b').map(|(_, v)| v.clone()).collect();
        let dates: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'c').map(|(_, v)| v.clone()).collect();
        let other_subfields = get_remaining_subfields(subfields, &KNOWN);
        Self {
            is_rda: false,
            function: PublicationFunction::from_ind2(ind2),
            places,
            publishers,
            dates,
            manufacturing_places: Vec::new(),
            manufacturing_dates: Vec::new(),
            other_subfields,
        }
    }

    fn from_subfields_210(subfields: &[(char, String)]) -> Self {
        const KNOWN: [char; 5] = ['a', 'c', 'd', 'e', 'g'];
        let places: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'a').map(|(_, v)| v.clone()).collect();
        let publishers: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'c').map(|(_, v)| v.clone()).collect();
        let dates: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'd').map(|(_, v)| v.clone()).collect();
        let manufacturing_places: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'e').map(|(_, v)| v.clone()).collect();
        let manufacturing_dates: Vec<String> = subfields.iter().filter(|(c, _)| *c == 'g').map(|(_, v)| v.clone()).collect();
        let other_subfields = get_remaining_subfields(subfields, &KNOWN);
        Self {
            is_rda: false,
            function: None,
            places,
            publishers,
            dates,
            manufacturing_places,
            manufacturing_dates,
            other_subfields,
        }
    }

    fn to_subfields(&self, format: MarcFormat) -> Vec<(char, String)> {
        let mut out = Vec::new();
        match format {
            MarcFormat::Unimarc => {
                for p in &self.places {
                    out.push(('a', p.clone()));
                }
                for p in &self.publishers {
                    out.push(('c', p.clone()));
                }
                for d in &self.dates {
                    out.push(('d', d.clone()));
                }
                for p in &self.manufacturing_places {
                    out.push(('e', p.clone()));
                }
                for d in &self.manufacturing_dates {
                    out.push(('g', d.clone()));
                }
            }
            MarcFormat::Marc21 | MarcFormat::MarcXml => {
                if let Some(p) = self.places.first() {
                    out.push(('a', p.clone()));
                }
                if let Some(p) = self.publishers.first() {
                    out.push(('b', p.clone()));
                }
                if let Some(d) = self.dates.first() {
                    out.push(('c', d.clone()));
                }
            }
        }
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Edition fields (25X in MARC21, 2XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Edition {
    /// Edition statement (250 in MARC21, 205 in UNIMARC)
    EditionStatement(EditionStatementData),
    /// Publication/distribution: place, publisher, date (260/264 MARC21, 210 UNIMARC)
    Publication(PublicationData),
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
    pub fn tag(&self, format: MarcFormat) -> Option<&str> {
        match (self, format) {
            (Edition::EditionStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("250"),
            (Edition::EditionStatement(_), MarcFormat::Unimarc) => Some("205"),
            (Edition::Publication(_), MarcFormat::Unimarc) => Some("210"),
            (Edition::Publication(d), MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                if d.is_rda { Some("264") } else { Some("260") }
            }
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
                EditionStatementData::from_subfields(subfields)
                    .map(Edition::EditionStatement)
            }
            ("260", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some(Edition::Publication(PublicationData::from_subfields_260_264(ind2, subfields)))
            }
            ("264", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                let mut d = PublicationData::from_subfields_260_264(ind2, subfields);
                d.is_rda = true;
                Some(Edition::Publication(d))
            }
            ("210", MarcFormat::Unimarc) => {
                Some(Edition::Publication(PublicationData::from_subfields_210(subfields)))
            }
            ("254", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields, format)
                    .map(Edition::MusicalPresentationStatement)
            }
            ("255", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("206", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields, format)
                    .map(Edition::CartographicMathematicalData)
            }
            ("256", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("336", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields, format)
                    .map(Edition::ComputerFileCharacteristics)
            }
            ("257", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields, format)
                    .map(Edition::CountryOfProducingEntity)
            }
            ("258", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields, format)
                    .map(Edition::PhilatelicIssueData)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        let df = match self {
            Edition::EditionStatement(d) => to_data_field(tag, ' ', ' ', d.to_subfields()),
            Edition::Publication(d) => {
                let ind2 = d.function.as_ref().map(|f| f.to_ind2()).unwrap_or(' ');
                to_data_field(tag, ' ', ind2, d.to_subfields(format))
            }
            Edition::MusicalPresentationStatement(d)
            | Edition::CartographicMathematicalData(d)
            | Edition::ComputerFileCharacteristics(d)
            | Edition::CountryOfProducingEntity(d)
            | Edition::PhilatelicIssueData(d) => {
                to_data_field(tag, ' ', ' ', d.to_subfields())
            }
        };
        Some(df)
    }
}
