use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalDescriptionData {
    pub ind1: char,
    pub ind2: char,
    pub extent: String,
    pub other_physical_details: Option<String>,
    pub dimensions: Option<String>,
    pub accompanying_material: Option<String>,
    pub other_subfields: Vec<(char, String)>,
}

impl PhysicalDescriptionData {
    const KNOWN_CODES: [char; 4] = ['a', 'b', 'c', 'e'];

    fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let extent = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            extent,
            other_physical_details: get_subfield(subfields, 'b'),
            dimensions: get_subfield(subfields, 'c'),
            accompanying_material: get_subfield(subfields, 'e'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.extent.clone())];
        push_subfield(&mut out, 'b', &self.other_physical_details);
        push_subfield(&mut out, 'c', &self.dimensions);
        push_subfield(&mut out, 'e', &self.accompanying_material);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Physical description fields (3XX in MARC21, 2XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Physical {
    PhysicalDescription(PhysicalDescriptionData),
    PlayingTime(NoteData),
    Hours(NoteData),
    CurrentPublicationFrequency(NoteData),
    FormerPublicationFrequency(NoteData),
    PhysicalMedium(NoteData),
    GeospatialReferenceData(NoteData),
    PlanarCoordinateData(NoteData),
    OrganizationAndArrangement(NoteData),
    DigitalGraphicRepresentation(NoteData),
    SecurityClassificationControl(NoteData),
    OriginatorDisseminationControl(NoteData),
    DatesOfPublication(NoteData),
    NormalizedDate(NoteData),
    TradePrice(NoteData),
    TradeAvailabilityInformation(NoteData),
    AssociatedPlace(NoteData),
    AssociatedLanguage(NoteData),
    FormOfWork(NoteData),
    OtherDistinguishingCharacteristics(NoteData),
    MediumOfPerformance(NoteData),
    NumericDesignationOfMusicalWork(NoteData),
    Key(NoteData),
    AudienceCharacteristics(NoteData),
    CreatorContributorCharacteristics(NoteData),
    TimePeriodOfCreation(NoteData),
}

impl Physical {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Physical::PhysicalDescription(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("300"),
            (Physical::PhysicalDescription(_), MarcFormat::Unimarc) => Some("215"),
            (Physical::PlayingTime(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("306"),
            (Physical::PlayingTime(_), MarcFormat::Unimarc) => Some("215"),
            (Physical::Hours(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("307"),
            (Physical::Hours(_), MarcFormat::Unimarc) => None,
            (Physical::CurrentPublicationFrequency(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("310"),
            (Physical::CurrentPublicationFrequency(_), MarcFormat::Unimarc) => Some("326"),
            (Physical::FormerPublicationFrequency(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("321"),
            (Physical::FormerPublicationFrequency(_), MarcFormat::Unimarc) => Some("326"),
            (Physical::PhysicalMedium(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("340"),
            (Physical::PhysicalMedium(_), MarcFormat::Unimarc) => Some("215"),
            (Physical::GeospatialReferenceData(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("342"),
            (Physical::GeospatialReferenceData(_), MarcFormat::Unimarc) => Some("206"),
            (Physical::PlanarCoordinateData(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("343"),
            (Physical::PlanarCoordinateData(_), MarcFormat::Unimarc) => Some("206"),
            (Physical::OrganizationAndArrangement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("351"),
            (Physical::OrganizationAndArrangement(_), MarcFormat::Unimarc) => Some("327"),
            (Physical::DigitalGraphicRepresentation(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("352"),
            (Physical::DigitalGraphicRepresentation(_), MarcFormat::Unimarc) => Some("336"),
            (Physical::SecurityClassificationControl(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("355"),
            (Physical::SecurityClassificationControl(_), MarcFormat::Unimarc) => None,
            (Physical::OriginatorDisseminationControl(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("357"),
            (Physical::OriginatorDisseminationControl(_), MarcFormat::Unimarc) => None,
            (Physical::DatesOfPublication(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("362"),
            (Physical::DatesOfPublication(_), MarcFormat::Unimarc) => Some("210"),
            (Physical::NormalizedDate(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("363"),
            (Physical::NormalizedDate(_), MarcFormat::Unimarc) => Some("210"),
            (Physical::TradePrice(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("365"),
            (Physical::TradePrice(_), MarcFormat::Unimarc) => Some("010"),
            (Physical::TradeAvailabilityInformation(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("366"),
            (Physical::TradeAvailabilityInformation(_), MarcFormat::Unimarc) => None,
            (Physical::AssociatedPlace(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("370"),
            (Physical::AssociatedPlace(_), MarcFormat::Unimarc) => Some("620"),
            (Physical::AssociatedLanguage(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("377"),
            (Physical::AssociatedLanguage(_), MarcFormat::Unimarc) => Some("101"),
            (Physical::FormOfWork(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("380"),
            (Physical::FormOfWork(_), MarcFormat::Unimarc) => Some("608"),
            (Physical::OtherDistinguishingCharacteristics(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("381"),
            (Physical::OtherDistinguishingCharacteristics(_), MarcFormat::Unimarc) => None,
            (Physical::MediumOfPerformance(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("382"),
            (Physical::MediumOfPerformance(_), MarcFormat::Unimarc) => Some("128"),
            (Physical::NumericDesignationOfMusicalWork(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("383"),
            (Physical::NumericDesignationOfMusicalWork(_), MarcFormat::Unimarc) => Some("125"),
            (Physical::Key(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("384"),
            (Physical::Key(_), MarcFormat::Unimarc) => Some("125"),
            (Physical::AudienceCharacteristics(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("385"),
            (Physical::AudienceCharacteristics(_), MarcFormat::Unimarc) => Some("330"),
            (Physical::CreatorContributorCharacteristics(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("386"),
            (Physical::CreatorContributorCharacteristics(_), MarcFormat::Unimarc) => None,
            (Physical::TimePeriodOfCreation(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("388"),
            (Physical::TimePeriodOfCreation(_), MarcFormat::Unimarc) => Some("660"),
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
            ("300", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("215", MarcFormat::Unimarc) => {
                PhysicalDescriptionData::from_subfields(ind1, ind2, subfields)
                    .map(Physical::PhysicalDescription)
            }
            ("306", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::PlayingTime)
            }
            ("307", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::Hours)
            }
            ("310", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("326", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::CurrentPublicationFrequency)
            }
            ("321", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::FormerPublicationFrequency)
            }
            ("340", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::PhysicalMedium)
            }
            ("342", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::GeospatialReferenceData)
            }
            ("343", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::PlanarCoordinateData)
            }
            ("351", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("327", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::OrganizationAndArrangement)
            }
            ("352", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::DigitalGraphicRepresentation)
            }
            ("355", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::SecurityClassificationControl)
            }
            ("357", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::OriginatorDisseminationControl)
            }
            ("362", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("210", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::DatesOfPublication)
            }
            ("363", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::NormalizedDate)
            }
            ("365", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::TradePrice)
            }
            ("366", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::TradeAvailabilityInformation)
            }
            ("370", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("620", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::AssociatedPlace)
            }
            ("377", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("101", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::AssociatedLanguage)
            }
            ("380", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("608", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::FormOfWork)
            }
            ("381", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::OtherDistinguishingCharacteristics)
            }
            ("382", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("128", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::MediumOfPerformance)
            }
            ("383", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("125", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::NumericDesignationOfMusicalWork)
            }
            ("384", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::Key)
            }
            ("385", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("330", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::AudienceCharacteristics)
            }
            ("386", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::CreatorContributorCharacteristics)
            }
            ("388", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("660", MarcFormat::Unimarc) => {
                NoteData::from_subfields(ind1, ind2, subfields).map(Physical::TimePeriodOfCreation)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        let df = match self {
            Physical::PhysicalDescription(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            Physical::PlayingTime(d)
            | Physical::Hours(d)
            | Physical::CurrentPublicationFrequency(d)
            | Physical::FormerPublicationFrequency(d)
            | Physical::PhysicalMedium(d)
            | Physical::GeospatialReferenceData(d)
            | Physical::PlanarCoordinateData(d)
            | Physical::OrganizationAndArrangement(d)
            | Physical::DigitalGraphicRepresentation(d)
            | Physical::SecurityClassificationControl(d)
            | Physical::OriginatorDisseminationControl(d)
            | Physical::DatesOfPublication(d)
            | Physical::NormalizedDate(d)
            | Physical::TradePrice(d)
            | Physical::TradeAvailabilityInformation(d)
            | Physical::AssociatedPlace(d)
            | Physical::AssociatedLanguage(d)
            | Physical::FormOfWork(d)
            | Physical::OtherDistinguishingCharacteristics(d)
            | Physical::MediumOfPerformance(d)
            | Physical::NumericDesignationOfMusicalWork(d)
            | Physical::Key(d)
            | Physical::AudienceCharacteristics(d)
            | Physical::CreatorContributorCharacteristics(d)
            | Physical::TimePeriodOfCreation(d) => {
                to_data_field(tag, d.ind1, d.ind2, d.to_subfields())
            }
        };
        Some(df)
    }
}
