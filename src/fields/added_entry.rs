use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Added entry fields (70X-75X in MARC21)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddedEntry {
    /// 700 - Personal name
    PersonalName(PersonalNameData),
    /// 710 - Corporate name
    CorporateName(CorporateNameData),
    /// 711 - Meeting name
    MeetingName(MeetingNameData),
    /// 720 - Uncontrolled name
    UncontrolledName(NoteData),
    /// 730 - Uniform title
    UniformTitle(UniformTitleData),
    /// 740 - Uncontrolled related/analytical title
    UncontrolledRelatedAnalyticalTitle(NoteData),
    /// 751 - Geographic name
    GeographicName(NoteData),
    /// 752 - Hierarchical place name
    HierarchicalPlaceName(NoteData),
    /// 753 - System details access to computer files
    SystemDetailsAccessToComputerFiles(NoteData),
    /// 754 - Taxonomic identification
    TaxonomicIdentification(NoteData),
    /// 755 - Physical characteristics
    PhysicalCharacteristics(NoteData),
}

impl AddedEntry {
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (AddedEntry::PersonalName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "700",
            (AddedEntry::PersonalName(_), MarcFormat::Unimarc) => "701",
            (AddedEntry::CorporateName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "710",
            (AddedEntry::CorporateName(_), MarcFormat::Unimarc) => "712",
            (AddedEntry::MeetingName(_), _) => "711",
            (AddedEntry::UncontrolledName(_), _) => "720",
            (AddedEntry::UniformTitle(_), _) => "730",
            (AddedEntry::UncontrolledRelatedAnalyticalTitle(_), _) => "740",
            (AddedEntry::GeographicName(_), _) => "751",
            (AddedEntry::HierarchicalPlaceName(_), _) => "752",
            (AddedEntry::SystemDetailsAccessToComputerFiles(_), _) => "753",
            (AddedEntry::TaxonomicIdentification(_), _) => "754",
            (AddedEntry::PhysicalCharacteristics(_), _) => "755",
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
            ("700", MarcFormat::Marc21 | MarcFormat::MarcXml)
            | ("701", MarcFormat::Unimarc)
            | ("702", MarcFormat::Unimarc) => {
                PersonalNameData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::PersonalName)
            }
            ("710", MarcFormat::Marc21 | MarcFormat::MarcXml)
            | ("712", MarcFormat::Unimarc) => {
                CorporateNameData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::CorporateName)
            }
            ("711", _) => {
                MeetingNameData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::MeetingName)
            }
            ("720", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::UncontrolledName)
            }
            ("730", _) => {
                UniformTitleData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::UniformTitle)
            }
            ("740", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::UncontrolledRelatedAnalyticalTitle)
            }
            ("751", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::GeographicName)
            }
            ("752", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::HierarchicalPlaceName)
            }
            ("753", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::SystemDetailsAccessToComputerFiles)
            }
            ("754", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::TaxonomicIdentification)
            }
            ("755", _) => {
                NoteData::from_subfields(ind1, ind2, subfields)
                    .map(AddedEntry::PhysicalCharacteristics)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = self.tag(format);
        match self {
            AddedEntry::PersonalName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            AddedEntry::CorporateName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            AddedEntry::MeetingName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            AddedEntry::UniformTitle(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            AddedEntry::UncontrolledName(d)
            | AddedEntry::UncontrolledRelatedAnalyticalTitle(d)
            | AddedEntry::GeographicName(d)
            | AddedEntry::HierarchicalPlaceName(d)
            | AddedEntry::SystemDetailsAccessToComputerFiles(d)
            | AddedEntry::TaxonomicIdentification(d)
            | AddedEntry::PhysicalCharacteristics(d) => {
                to_data_field(tag, d.ind1, d.ind2, d.to_subfields())
            }
        }
    }
}
