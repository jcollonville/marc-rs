use crate::format::MarcFormat;

/// Physical description fields (3XX in MARC21, 2XX in UNIMARC)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physical {
    /// Physical description (300 in MARC21, 215 in UNIMARC)
    PhysicalDescription,
    /// 306 - Playing time
    PlayingTime,
    /// 307 - Hours, etc.
    Hours,
    /// 310 - Current publication frequency
    CurrentPublicationFrequency,
    /// 321 - Former publication frequency
    FormerPublicationFrequency,
    /// 340 - Physical medium
    PhysicalMedium,
    /// 342 - Geospatial reference data
    GeospatialReferenceData,
    /// 343 - Planar coordinate data
    PlanarCoordinateData,
    /// 351 - Organization and arrangement of materials
    OrganizationAndArrangement,
    /// 352 - Digital graphic representation
    DigitalGraphicRepresentation,
    /// 355 - Security classification control
    SecurityClassificationControl,
    /// 357 - Originator dissemination control
    OriginatorDisseminationControl,
    /// 362 - Dates of publication and/or sequential designation
    DatesOfPublication,
    /// 363 - Normalized date and sequential designation
    NormalizedDate,
    /// 365 - Trade price
    TradePrice,
    /// 366 - Trade availability information
    TradeAvailabilityInformation,
    /// 370 - Associated place
    AssociatedPlace,
    /// 377 - Associated language
    AssociatedLanguage,
    /// 380 - Form of work
    FormOfWork,
    /// 381 - Other distinguishing characteristics
    OtherDistinguishingCharacteristics,
    /// 382 - Medium of performance
    MediumOfPerformance,
    /// 383 - Numeric designation of musical work
    NumericDesignationOfMusicalWork,
    /// 384 - Key
    Key,
    /// 385 - Audience characteristics
    AudienceCharacteristics,
    /// 386 - Creator/Contributor characteristics
    CreatorContributorCharacteristics,
    /// 388 - Time period of creation
    TimePeriodOfCreation,
}

impl Physical {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Physical::PhysicalDescription, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("300"),
            (Physical::PhysicalDescription, MarcFormat::Unimarc) => Some("215"),
            
            (Physical::PlayingTime, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("306"),
            (Physical::PlayingTime, MarcFormat::Unimarc) => Some("215"), // Part of physical description
            
            (Physical::Hours, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("307"),
            (Physical::Hours, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::CurrentPublicationFrequency, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("310"),
            (Physical::CurrentPublicationFrequency, MarcFormat::Unimarc) => Some("326"),
            
            (Physical::FormerPublicationFrequency, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("321"),
            (Physical::FormerPublicationFrequency, MarcFormat::Unimarc) => Some("326"),
            
            (Physical::PhysicalMedium, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("340"),
            (Physical::PhysicalMedium, MarcFormat::Unimarc) => Some("215"), // Part of physical description
            
            (Physical::GeospatialReferenceData, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("342"),
            (Physical::GeospatialReferenceData, MarcFormat::Unimarc) => Some("206"), // Cartographic data
            
            (Physical::PlanarCoordinateData, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("343"),
            (Physical::PlanarCoordinateData, MarcFormat::Unimarc) => Some("206"), // Cartographic data
            
            (Physical::OrganizationAndArrangement, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("351"),
            (Physical::OrganizationAndArrangement, MarcFormat::Unimarc) => Some("327"),
            
            (Physical::DigitalGraphicRepresentation, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("352"),
            (Physical::DigitalGraphicRepresentation, MarcFormat::Unimarc) => Some("336"),
            
            (Physical::SecurityClassificationControl, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("355"),
            (Physical::SecurityClassificationControl, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::OriginatorDisseminationControl, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("357"),
            (Physical::OriginatorDisseminationControl, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::DatesOfPublication, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("362"),
            (Physical::DatesOfPublication, MarcFormat::Unimarc) => Some("210"), // Publication/distribution
            
            (Physical::NormalizedDate, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("363"),
            (Physical::NormalizedDate, MarcFormat::Unimarc) => Some("210"), // Publication/distribution
            
            (Physical::TradePrice, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("365"),
            (Physical::TradePrice, MarcFormat::Unimarc) => Some("010"), // ISBN/price
            
            (Physical::TradeAvailabilityInformation, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("366"),
            (Physical::TradeAvailabilityInformation, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::AssociatedPlace, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("370"),
            (Physical::AssociatedPlace, MarcFormat::Unimarc) => Some("620"), // Place access
            
            (Physical::AssociatedLanguage, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("377"),
            (Physical::AssociatedLanguage, MarcFormat::Unimarc) => Some("101"), // Language
            
            (Physical::FormOfWork, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("380"),
            (Physical::FormOfWork, MarcFormat::Unimarc) => Some("608"), // Form/genre
            
            (Physical::OtherDistinguishingCharacteristics, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("381"),
            (Physical::OtherDistinguishingCharacteristics, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::MediumOfPerformance, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("382"),
            (Physical::MediumOfPerformance, MarcFormat::Unimarc) => Some("128"), // Musical form
            
            (Physical::NumericDesignationOfMusicalWork, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("383"),
            (Physical::NumericDesignationOfMusicalWork, MarcFormat::Unimarc) => Some("125"), // Musical key
            
            (Physical::Key, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("384"),
            (Physical::Key, MarcFormat::Unimarc) => Some("125"), // Musical key
            
            (Physical::AudienceCharacteristics, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("385"),
            (Physical::AudienceCharacteristics, MarcFormat::Unimarc) => Some("330"), // Audience
            
            (Physical::CreatorContributorCharacteristics, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("386"),
            (Physical::CreatorContributorCharacteristics, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Physical::TimePeriodOfCreation, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("388"),
            (Physical::TimePeriodOfCreation, MarcFormat::Unimarc) => Some("660"), // Geographic/time
        }
    }
}
