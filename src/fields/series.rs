use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesStatementData {
    pub ind1: char,
    pub ind2: char,
    pub statement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subseries: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl SeriesStatementData {
    const KNOWN_CODES: [char; 4] = ['a', 'i', 'v', 'x'];

    fn from_subfields(ind1: char, ind2: char, subfields: &[(char, String)]) -> Option<Self> {
        let statement = get_subfield(subfields, 'a')?;
        Some(Self {
            ind1,
            ind2,
            statement,
            volume: get_subfield(subfields, 'v'),
            issn: get_subfield(subfields, 'x'),
            subseries: get_subfield(subfields, 'i'),
            other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
        })
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = vec![('a', self.statement.clone())];
        push_subfield(&mut out, 'i', &self.subseries);
        push_subfield(&mut out, 'v', &self.volume);
        push_subfield(&mut out, 'x', &self.issn);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Series statement fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Series {
    /// Series/Added entry - Personal name (400 in MARC21, 410 in UNIMARC)
    SeriesPersonalName(PersonalNameData),
    /// Series/Added entry - Corporate name (410 in MARC21)
    SeriesCorporateName(CorporateNameData),
    /// Series/Added entry - Meeting name (411 in MARC21)
    SeriesMeetingName(MeetingNameData),
    /// Series title (440 in MARC21, 225 in UNIMARC)
    SeriesTitle(SeriesStatementData),
    /// Series statement (490 in MARC21, 225 in UNIMARC)
    SeriesStatement(SeriesStatementData),
}

impl Series {
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (Series::SeriesPersonalName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "400",
            (Series::SeriesPersonalName(_), MarcFormat::Unimarc) => "410",
            (Series::SeriesCorporateName(_), _) => "410",
            (Series::SeriesMeetingName(_), _) => "411",
            (Series::SeriesTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "440",
            (Series::SeriesTitle(_), MarcFormat::Unimarc) => "225",
            (Series::SeriesStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "490",
            (Series::SeriesStatement(_), MarcFormat::Unimarc) => "225",
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
            ("400", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                PersonalNameData::from_subfields(ind1, ind2, subfields)
                    .map(Series::SeriesPersonalName)
            }
            ("410", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                CorporateNameData::from_subfields(ind1, ind2, subfields)
                    .map(Series::SeriesCorporateName)
            }
            ("411", _) => {
                MeetingNameData::from_subfields(ind1, ind2, subfields)
                    .map(Series::SeriesMeetingName)
            }
            ("440", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                SeriesStatementData::from_subfields(ind1, ind2, subfields)
                    .map(Series::SeriesTitle)
            }
            ("490", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("225", MarcFormat::Unimarc) => {
                SeriesStatementData::from_subfields(ind1, ind2, subfields)
                    .map(Series::SeriesStatement)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = self.tag(format);
        match self {
            Series::SeriesPersonalName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            Series::SeriesCorporateName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            Series::SeriesMeetingName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            Series::SeriesTitle(d) | Series::SeriesStatement(d) => {
                to_data_field(tag, d.ind1, d.ind2, d.to_subfields())
            }
        }
    }
}
