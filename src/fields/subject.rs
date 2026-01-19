use crate::format::MarcFormat;

/// Subject access fields (6XX in MARC21, 6XX in UNIMARC with different structure)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    /// 600 - Subject added entry - Personal name
    SubjectPersonalName,
    /// 610 - Subject added entry - Corporate name
    SubjectCorporateName,
    /// 611 - Subject added entry - Meeting name
    SubjectMeetingName,
    /// 630 - Subject added entry - Uniform title
    SubjectUniformTitle,
    /// 650 - Subject added entry - Topical term
    SubjectTopicalTerm,
    /// 651 - Subject added entry - Geographic name
    SubjectGeographicName,
    /// 653 - Index term - Uncontrolled
    IndexTermUncontrolled,
    /// 654 - Subject added entry - Faceted topical terms
    SubjectFacetedTopicalTerms,
    /// 655 - Index term - Genre/form
    IndexTermGenreForm,
    /// 656 - Index term - Occupation
    IndexTermOccupation,
    /// 657 - Index term - Function
    IndexTermFunction,
    /// 658 - Index term - Curriculum objective
    IndexTermCurriculumObjective,
    /// 662 - Subject added entry - Hierarchical place name
    SubjectHierarchicalPlaceName,
    /// 688 - Subject added entry - Type of entity unspecified
    SubjectTypeOfEntityUnspecified,
}

impl Subject {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Subject::SubjectPersonalName, _) => Some("600"),
            (Subject::SubjectCorporateName, _) => Some("610"),
            (Subject::SubjectMeetingName, _) => Some("611"),
            (Subject::SubjectUniformTitle, _) => Some("630"),
            
            (Subject::SubjectTopicalTerm, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("650"),
            (Subject::SubjectTopicalTerm, MarcFormat::Unimarc) => Some("606"), // Topical name
            
            (Subject::SubjectGeographicName, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("651"),
            (Subject::SubjectGeographicName, MarcFormat::Unimarc) => Some("607"), // Geographic name
            
            (Subject::IndexTermUncontrolled, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("653"),
            (Subject::IndexTermUncontrolled, MarcFormat::Unimarc) => Some("610"), // Uncontrolled subject
            
            (Subject::SubjectFacetedTopicalTerms, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("654"),
            (Subject::SubjectFacetedTopicalTerms, MarcFormat::Unimarc) => Some("606"), // Topical name
            
            (Subject::IndexTermGenreForm, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("655"),
            (Subject::IndexTermGenreForm, MarcFormat::Unimarc) => Some("608"), // Form/genre
            
            (Subject::IndexTermOccupation, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("656"),
            (Subject::IndexTermOccupation, MarcFormat::Unimarc) => Some("608"), // Form/genre
            
            (Subject::IndexTermFunction, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("657"),
            (Subject::IndexTermFunction, MarcFormat::Unimarc) => Some("608"), // Form/genre
            
            (Subject::IndexTermCurriculumObjective, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("658"),
            (Subject::IndexTermCurriculumObjective, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Subject::SubjectHierarchicalPlaceName, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("662"),
            (Subject::SubjectHierarchicalPlaceName, MarcFormat::Unimarc) => Some("607"), // Geographic name
            
            (Subject::SubjectTypeOfEntityUnspecified, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("688"),
            (Subject::SubjectTypeOfEntityUnspecified, MarcFormat::Unimarc) => None, // Not in UNIMARC
        }
    }
}
