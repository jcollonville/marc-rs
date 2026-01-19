use crate::format::MarcFormat;

/// Linking entry fields (76X-78X in MARC21, 4XX in UNIMARC)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Linking {
    /// 760 - Main series entry
    MainSeriesEntry,
    /// 762 - Subseries entry
    SubseriesEntry,
    /// 765 - Original language entry
    OriginalLanguageEntry,
    /// 767 - Translation entry
    TranslationEntry,
    /// 770 - Supplement/special issue entry
    SupplementSpecialIssueEntry,
    /// 772 - Supplement parent entry
    SupplementParentEntry,
    /// 773 - Host item entry
    HostItemEntry,
    /// 774 - Constituent unit entry
    ConstituentUnitEntry,
    /// 775 - Other edition entry
    OtherEditionEntry,
    /// 776 - Additional physical form entry
    AdditionalPhysicalFormEntry,
    /// 777 - Issued with entry
    IssuedWithEntry,
    /// 780 - Preceding entry
    PrecedingEntry,
    /// 785 - Succeeding entry
    SucceedingEntry,
    /// 786 - Data source entry
    DataSourceEntry,
    /// 787 - Other relationship entry
    OtherRelationshipEntry,
}

impl Linking {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            // In UNIMARC, linking entries are in the 4XX block
            (Linking::MainSeriesEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("760"),
            (Linking::MainSeriesEntry, MarcFormat::Unimarc) => Some("410"), // Series
            
            (Linking::SubseriesEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("762"),
            (Linking::SubseriesEntry, MarcFormat::Unimarc) => Some("411"), // Subseries
            
            (Linking::OriginalLanguageEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("765"),
            (Linking::OriginalLanguageEntry, MarcFormat::Unimarc) => Some("454"), // Translation
            
            (Linking::TranslationEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("767"),
            (Linking::TranslationEntry, MarcFormat::Unimarc) => Some("454"), // Translation
            
            (Linking::SupplementSpecialIssueEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("770"),
            (Linking::SupplementSpecialIssueEntry, MarcFormat::Unimarc) => Some("488"), // Other related title
            
            (Linking::SupplementParentEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("772"),
            (Linking::SupplementParentEntry, MarcFormat::Unimarc) => Some("488"), // Other related title
            
            (Linking::HostItemEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("773"),
            (Linking::HostItemEntry, MarcFormat::Unimarc) => Some("461"), // Set level
            
            (Linking::ConstituentUnitEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("774"),
            (Linking::ConstituentUnitEntry, MarcFormat::Unimarc) => Some("462"), // Subset level
            
            (Linking::OtherEditionEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("775"),
            (Linking::OtherEditionEntry, MarcFormat::Unimarc) => Some("453"), // Other edition
            
            (Linking::AdditionalPhysicalFormEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("776"),
            (Linking::AdditionalPhysicalFormEntry, MarcFormat::Unimarc) => Some("452"), // Other edition
            
            (Linking::IssuedWithEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("777"),
            (Linking::IssuedWithEntry, MarcFormat::Unimarc) => Some("488"), // Other related title
            
            (Linking::PrecedingEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("780"),
            (Linking::PrecedingEntry, MarcFormat::Unimarc) => Some("430"), // Continuation of
            
            (Linking::SucceedingEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("785"),
            (Linking::SucceedingEntry, MarcFormat::Unimarc) => Some("431"), // Continuation
            
            (Linking::DataSourceEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("786"),
            (Linking::DataSourceEntry, MarcFormat::Unimarc) => None, // Not in UNIMARC
            
            (Linking::OtherRelationshipEntry, MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("787"),
            (Linking::OtherRelationshipEntry, MarcFormat::Unimarc) => Some("488"), // Other related title
        }
    }
}
