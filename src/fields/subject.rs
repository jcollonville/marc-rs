use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Subject access fields (6XX in MARC21, 6XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Subject {
    SubjectPersonalName(SubjectData),
    SubjectCorporateName(SubjectData),
    SubjectMeetingName(SubjectData),
    SubjectUniformTitle(SubjectData),
    SubjectTopicalTerm(SubjectData),
    SubjectGeographicName(SubjectData),
    IndexTermUncontrolled(SubjectData),
    SubjectFacetedTopicalTerms(SubjectData),
    IndexTermGenreForm(SubjectData),
    IndexTermOccupation(SubjectData),
    IndexTermFunction(SubjectData),
    IndexTermCurriculumObjective(SubjectData),
    SubjectHierarchicalPlaceName(SubjectData),
    SubjectTypeOfEntityUnspecified(SubjectData),
}

impl Subject {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Subject::SubjectPersonalName(_), _) => Some("600"),
            (Subject::SubjectCorporateName(_), _) => Some("610"),
            (Subject::SubjectMeetingName(_), _) => Some("611"),
            (Subject::SubjectUniformTitle(_), _) => Some("630"),
            (Subject::SubjectTopicalTerm(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("650"),
            (Subject::SubjectTopicalTerm(_), MarcFormat::Unimarc) => Some("606"),
            (Subject::SubjectGeographicName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("651"),
            (Subject::SubjectGeographicName(_), MarcFormat::Unimarc) => Some("607"),
            (Subject::IndexTermUncontrolled(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("653"),
            (Subject::IndexTermUncontrolled(_), MarcFormat::Unimarc) => Some("610"),
            (Subject::SubjectFacetedTopicalTerms(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("654"),
            (Subject::SubjectFacetedTopicalTerms(_), MarcFormat::Unimarc) => Some("606"),
            (Subject::IndexTermGenreForm(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("655"),
            (Subject::IndexTermGenreForm(_), MarcFormat::Unimarc) => Some("608"),
            (Subject::IndexTermOccupation(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("656"),
            (Subject::IndexTermOccupation(_), MarcFormat::Unimarc) => Some("608"),
            (Subject::IndexTermFunction(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("657"),
            (Subject::IndexTermFunction(_), MarcFormat::Unimarc) => Some("608"),
            (Subject::IndexTermCurriculumObjective(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("658"),
            (Subject::IndexTermCurriculumObjective(_), MarcFormat::Unimarc) => None,
            (Subject::SubjectHierarchicalPlaceName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("662"),
            (Subject::SubjectHierarchicalPlaceName(_), MarcFormat::Unimarc) => Some("607"),
            (Subject::SubjectTypeOfEntityUnspecified(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("688"),
            (Subject::SubjectTypeOfEntityUnspecified(_), MarcFormat::Unimarc) => None,
        }
    }

    fn data(&self) -> &SubjectData {
        match self {
            Subject::SubjectPersonalName(d)
            | Subject::SubjectCorporateName(d)
            | Subject::SubjectMeetingName(d)
            | Subject::SubjectUniformTitle(d)
            | Subject::SubjectTopicalTerm(d)
            | Subject::SubjectGeographicName(d)
            | Subject::IndexTermUncontrolled(d)
            | Subject::SubjectFacetedTopicalTerms(d)
            | Subject::IndexTermGenreForm(d)
            | Subject::IndexTermOccupation(d)
            | Subject::IndexTermFunction(d)
            | Subject::IndexTermCurriculumObjective(d)
            | Subject::SubjectHierarchicalPlaceName(d)
            | Subject::SubjectTypeOfEntityUnspecified(d) => d,
        }
    }

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        let d = SubjectData::from_subfields(ind1, ind2, subfields)?;
        let subj = match (tag, format) {
            ("600", _) => Subject::SubjectPersonalName(d),
            ("610", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::SubjectCorporateName(d),
            ("611", _) => Subject::SubjectMeetingName(d),
            ("630", _) => Subject::SubjectUniformTitle(d),
            ("650", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("606", MarcFormat::Unimarc) => {
                Subject::SubjectTopicalTerm(d)
            }
            ("651", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("607", MarcFormat::Unimarc) => {
                Subject::SubjectGeographicName(d)
            }
            ("653", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::IndexTermUncontrolled(d),
            ("654", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::SubjectFacetedTopicalTerms(d),
            ("655", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("608", MarcFormat::Unimarc) => {
                Subject::IndexTermGenreForm(d)
            }
            ("656", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::IndexTermOccupation(d),
            ("657", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::IndexTermFunction(d),
            ("658", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::IndexTermCurriculumObjective(d),
            ("662", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::SubjectHierarchicalPlaceName(d),
            ("688", MarcFormat::Marc21 | MarcFormat::MarcXml) => Subject::SubjectTypeOfEntityUnspecified(d),
            _ => return None,
        };
        Some(subj)
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        let d = self.data();
        Some(to_data_field(tag, d.ind1, d.ind2, d.to_subfields()))
    }
}
