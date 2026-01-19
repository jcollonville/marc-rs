use crate::format::MarcFormat;

/// Added entry fields (70X-75X in MARC21, 7XX in UNIMARC for responsibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddedEntry {
    /// 700 - Added entry - Personal name
    PersonalName,
    /// 710 - Added entry - Corporate name
    CorporateName,
    /// 711 - Added entry - Meeting name
    MeetingName,
    /// 720 - Added entry - Uncontrolled name
    UncontrolledName,
    /// 730 - Added entry - Uniform title
    UniformTitle,
    /// 740 - Added entry - Uncontrolled related/analytical title
    UncontrolledRelatedAnalyticalTitle,
    /// 751 - Added entry - Geographic name
    GeographicName,
    /// 752 - Added entry - Hierarchical place name
    HierarchicalPlaceName,
    /// 753 - System details access to computer files
    SystemDetailsAccessToComputerFiles,
    /// 754 - Added entry - Taxonomic identification
    TaxonomicIdentification,
    /// 755 - Added entry - Physical characteristics
    PhysicalCharacteristics,
}

impl AddedEntry {
    /// Get the tag as string for the given format
    pub fn tag(&self, _format: MarcFormat) -> &'static str {
        // In UNIMARC, 7XX is the responsibility block, similar to MARC21's 7XX
        // Most tags are the same, but usage may differ
        match self {
            AddedEntry::PersonalName => "700",
            AddedEntry::CorporateName => "710",
            AddedEntry::MeetingName => "711",
            AddedEntry::UncontrolledName => "720",
            AddedEntry::UniformTitle => "730",
            AddedEntry::UncontrolledRelatedAnalyticalTitle => "740",
            AddedEntry::GeographicName => "751",
            AddedEntry::HierarchicalPlaceName => "752",
            AddedEntry::SystemDetailsAccessToComputerFiles => "753",
            AddedEntry::TaxonomicIdentification => "754",
            AddedEntry::PhysicalCharacteristics => "755",
        }
    }
}
