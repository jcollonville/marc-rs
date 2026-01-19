use crate::format::MarcFormat;

/// Title and title-related fields (20X-24X)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Title {
    /// Title statement (200 in UNIMARC, 245 in MARC21)
    TitleStatement,
    /// Varying form of title (246 in MARC21, 517 in UNIMARC)
    VaryingFormOfTitle,
    /// Former title (247 in MARC21, 520 in UNIMARC)
    FormerTitle,
    /// Parallel title (246 in MARC21 variant, 510 in UNIMARC)
    ParallelTitle,
    /// Other title information (246 in MARC21, 517 in UNIMARC)
    OtherTitleInformation,
}

impl Title {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (Title::TitleStatement, MarcFormat::Marc21 | MarcFormat::MarcXml) => "245", // XML follows MARC21 structure
            (Title::TitleStatement, MarcFormat::Unimarc) => "200",

            (Title::VaryingFormOfTitle, MarcFormat::Marc21 | MarcFormat::MarcXml) => "246",
            (Title::VaryingFormOfTitle, MarcFormat::Unimarc) => "517",

            (Title::FormerTitle, MarcFormat::Marc21 | MarcFormat::MarcXml) => "247",
            (Title::FormerTitle, MarcFormat::Unimarc) => "520",

            (Title::ParallelTitle, MarcFormat::Marc21 | MarcFormat::MarcXml) => "246", // Used with specific indicators
            (Title::ParallelTitle, MarcFormat::Unimarc) => "510",

            (Title::OtherTitleInformation, MarcFormat::Marc21 | MarcFormat::MarcXml) => "246", // Used with specific indicators
            (Title::OtherTitleInformation, MarcFormat::Unimarc) => "517",
        }
    }
}
