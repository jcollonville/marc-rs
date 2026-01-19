use crate::format::MarcFormat;

/// Control fields (00X) - Generally similar across formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// Control number (001 in both)
    ControlNumber,
    /// Control number identifier (003 in both)
    ControlNumberIdentifier,
    /// Date and time of latest transaction (005 in both)
    DateAndTimeOfLatestTransaction,
    /// Fixed-length data elements - Additional material characteristics (006 in MARC21, not in UNIMARC)
    FixedLengthDataElementsAdditional,
    /// Physical description fixed field (007 in both)
    PhysicalDescriptionFixedField,
    /// Fixed-length data elements (008 in MARC21, 100 in UNIMARC)
    FixedLengthDataElements,
    /// Local control number (009 in UNIMARC, not standard in MARC21)
    LocalControlNumber,
}

impl Control {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Control::ControlNumber, _) => Some("001"),
            (Control::ControlNumberIdentifier, _) => Some("003"),
            (Control::DateAndTimeOfLatestTransaction, _) => Some("005"),

            (Control::FixedLengthDataElementsAdditional, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("006"),
            (Control::FixedLengthDataElementsAdditional, MarcFormat::Unimarc) => None, // Not in UNIMARC

            (Control::PhysicalDescriptionFixedField, _) => Some("007"),

            (Control::FixedLengthDataElements, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("008"),
            (Control::FixedLengthDataElements, MarcFormat::Unimarc) => Some("100"), // Coded data block

            (Control::LocalControlNumber, MarcFormat::Marc21 | MarcFormat::MarcXml) => None, // Not standard
            (Control::LocalControlNumber, MarcFormat::Unimarc) => Some("009"),
        }
    }
}
