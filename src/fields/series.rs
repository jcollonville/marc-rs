use crate::format::MarcFormat;

/// Series statement fields (4XX in MARC21, 4XX in UNIMARC with different structure)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Series {
    /// Series statement/Added entry - Personal name (400 in MARC21, 410 in UNIMARC)
    SeriesPersonalName,
    /// Series statement/Added entry - Corporate name (410 in MARC21, 410 in UNIMARC)
    SeriesCorporateName,
    /// Series statement/Added entry - Meeting name (411 in MARC21, 411 in UNIMARC)
    SeriesMeetingName,
    /// Series statement/Added entry - Title (440 in MARC21, 225 in UNIMARC)
    SeriesTitle,
    /// Series statement (490 in MARC21, 225 in UNIMARC)
    SeriesStatement,
}

impl Series {
    /// Get the tag as string for the given format
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (Series::SeriesPersonalName, MarcFormat::Marc21 | MarcFormat::MarcXml) => "400",
            (Series::SeriesPersonalName, MarcFormat::Unimarc) => "410",
            
            (Series::SeriesCorporateName, _) => "410",
            
            (Series::SeriesMeetingName, _) => "411",
            
            (Series::SeriesTitle, MarcFormat::Marc21 | MarcFormat::MarcXml) => "440",
            (Series::SeriesTitle, MarcFormat::Unimarc) => "225",
            
            (Series::SeriesStatement, MarcFormat::Marc21 | MarcFormat::MarcXml) => "490",
            (Series::SeriesStatement, MarcFormat::Unimarc) => "225",
        }
    }
}
