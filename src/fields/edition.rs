use crate::format::MarcFormat;

/// Edition fields (25X in MARC21, 2XX in UNIMARC)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edition {
    /// Edition statement (250 in MARC21, 205 in UNIMARC)
    EditionStatement,
    /// Musical presentation statement (254 in MARC21, not in UNIMARC)
    MusicalPresentationStatement,
    /// Cartographic mathematical data (255 in MARC21, 206 in UNIMARC)
    CartographicMathematicalData,
    /// Computer file characteristics (256 in MARC21, 336 in UNIMARC)
    ComputerFileCharacteristics,
    /// Country of producing entity (257 in MARC21, not in UNIMARC)
    CountryOfProducingEntity,
    /// Philatelic issue data (258 in MARC21, not in UNIMARC)
    PhilatelicIssueData,
}

impl Edition {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Edition::EditionStatement, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("250"),
            (Edition::EditionStatement, MarcFormat::Unimarc) => Some("205"),
            
            (Edition::MusicalPresentationStatement, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("254"),
            (Edition::MusicalPresentationStatement, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Edition::CartographicMathematicalData, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("255"),
            (Edition::CartographicMathematicalData, MarcFormat::Unimarc) => Some("206"),
            
            (Edition::ComputerFileCharacteristics, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("256"),
            (Edition::ComputerFileCharacteristics, MarcFormat::Unimarc) => Some("336"),
            
            (Edition::CountryOfProducingEntity, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("257"),
            (Edition::CountryOfProducingEntity, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Edition::PhilatelicIssueData, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("258"),
            (Edition::PhilatelicIssueData, MarcFormat::Unimarc) => None, // Not in UNIMARC
        }
    }
}
