use crate::format::MarcFormat;

/// Main entry fields (1XX in MARC21, 7XX in UNIMARC for responsibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainEntry {
    /// Personal name (100 in MARC21, 700 in UNIMARC)
    PersonalName,
    /// Corporate name (110 in MARC21, 710 in UNIMARC)
    CorporateName,
    /// Meeting name (111 in MARC21, 711 in UNIMARC)
    MeetingName,
    /// Uniform title (130 in MARC21, 730 in UNIMARC)
    UniformTitle,
}

impl MainEntry {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (MainEntry::PersonalName, MarcFormat::Marc21 | MarcFormat::MarcXml) => "100",
            (MainEntry::PersonalName, MarcFormat::Unimarc) => "700",

            (MainEntry::CorporateName, MarcFormat::Marc21 | MarcFormat::MarcXml) => "110",
            (MainEntry::CorporateName, MarcFormat::Unimarc) => "710",

            (MainEntry::MeetingName, MarcFormat::Marc21 | MarcFormat::MarcXml) => "111",
            (MainEntry::MeetingName, MarcFormat::Unimarc) => "711",

            (MainEntry::UniformTitle, MarcFormat::Marc21 | MarcFormat::MarcXml) => "130",
            (MainEntry::UniformTitle, MarcFormat::Unimarc) => "730",
        }
    }
}
