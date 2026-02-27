use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Series/collection statement data.
/// UNIMARC 225: $a title, $v volume, $x ISSN.
/// MARC21 490: $a title, $v volume, $x ISSN; ind1 = traced (1) or not (0).
/// MARC21 830: $a uniform title, $v volume, $n/$p in other_subfields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesStatementData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub traced: bool,
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

    fn from_subfields(ind1: char, subfields: &[(char, String)]) -> Option<Self> {
        let statement = get_subfield(subfields, 'a')?;
        Some(Self {
            traced: ind1 == '1',
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

/// Series/collection fields.
///
/// **UNIMARC** — Bloc 2XX (description), 4XX (lien):
/// - **Collection** ou **Série/Suite**: 225 ($a titre, $v volume — crucial pour série/suite, $x ISSN — plutôt collection).
/// - Lien: 410 (collection mère), 411 (sous-série) → voir `Linking::MainSeriesEntry` / `SubseriesEntry`.
///
/// **MARC21** — Mention "vue sur le livre" vs indexation:
/// - **Collection**: 490 (mention) + 830 (point d'accès uniforme pour regroupement).
/// - **Série auteur unique** (ex. James Bond): 800 (point d'accès personnel). Plusieurs auteurs → 830.
/// - 810/811: point d'accès corporate/meeting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Series {
    /// Series statement - Personal name (400 MARC21). UNIMARC 410 = lien → Linking.
    SeriesPersonalName(PersonalNameData),
    /// Series statement - Corporate name (410 MARC21 only)
    SeriesCorporateName(CorporateNameData),
    /// Series statement - Meeting name (411)
    SeriesMeetingName(MeetingNameData),
    /// Series added entry - Personal name (800 MARC21). Série auteur unique, point d'accès pour regroupement.
    SeriesAddedEntryPersonalName(PersonalNameData),
    /// Series added entry - Corporate name (810 MARC21 only)
    SeriesAddedEntryCorporateName(CorporateNameData),
    /// Series added entry - Meeting name (811 MARC21 only)
    SeriesAddedEntryMeetingName(MeetingNameData),
    /// Series title (440 MARC21, 225 UNIMARC) — $a, $v, $x
    SeriesTitle(SeriesStatementData),
    /// Series statement / mention de collection (490 MARC21, 225 UNIMARC)
    SeriesStatement(SeriesStatementData),
    /// Added entry - uniform title (830 MARC21 only); regroupement sous forme normalisée
    SeriesUniformTitle(SeriesStatementData),
}

impl Series {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Series::SeriesPersonalName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("400"),
            (Series::SeriesPersonalName(_), MarcFormat::Unimarc) => None,
            (Series::SeriesCorporateName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("410"),
            (Series::SeriesCorporateName(_), MarcFormat::Unimarc) => None,
            (Series::SeriesMeetingName(_), _) => Some("411"),
            (Series::SeriesAddedEntryPersonalName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some("800")
            }
            (Series::SeriesAddedEntryPersonalName(_), MarcFormat::Unimarc) => None,
            (Series::SeriesAddedEntryCorporateName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some("810")
            }
            (Series::SeriesAddedEntryCorporateName(_), MarcFormat::Unimarc) => None,
            (Series::SeriesAddedEntryMeetingName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some("811")
            }
            (Series::SeriesAddedEntryMeetingName(_), MarcFormat::Unimarc) => None,
            (Series::SeriesTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("440"),
            (Series::SeriesTitle(_), MarcFormat::Unimarc) => Some("225"),
            (Series::SeriesStatement(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("490"),
            (Series::SeriesStatement(_), MarcFormat::Unimarc) => Some("225"),
            (Series::SeriesUniformTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("830"),
            (Series::SeriesUniformTitle(_), MarcFormat::Unimarc) => None,
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
                PersonalNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesPersonalName)
            }
            ("410", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                CorporateNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesCorporateName)
            }
            ("411", _) => {
                MeetingNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesMeetingName)
            }
            ("800", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                PersonalNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesAddedEntryPersonalName)
            }
            ("810", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                CorporateNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesAddedEntryCorporateName)
            }
            ("811", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                MeetingNameData::from_subfields(ind1, ind2, subfields, format)
                    .map(Series::SeriesAddedEntryMeetingName)
            }
            ("440", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                SeriesStatementData::from_subfields(ind1, subfields)
                    .map(Series::SeriesTitle)
            }
            ("490", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("225", MarcFormat::Unimarc) => {
                SeriesStatementData::from_subfields(ind1, subfields)
                    .map(Series::SeriesStatement)
            }
            ("830", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                SeriesStatementData::from_subfields(ind1, subfields)
                    .map(Series::SeriesUniformTitle)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        Some(match self {
            Series::SeriesPersonalName(d)
            | Series::SeriesAddedEntryPersonalName(d) => {
                to_data_field(tag, d.name_type.to_ind1(format), ' ', d.to_subfields())
            }
            Series::SeriesCorporateName(d) | Series::SeriesAddedEntryCorporateName(d) => {
                to_data_field(tag, d.name_type.to_ind1(format), ' ', d.to_subfields())
            }
            Series::SeriesMeetingName(d) | Series::SeriesAddedEntryMeetingName(d) => {
                to_data_field(tag, d.name_type.to_ind1(format), ' ', d.to_subfields())
            }
            Series::SeriesTitle(d)
            | Series::SeriesStatement(d)
            | Series::SeriesUniformTitle(d) => {
                let ind1 = if d.traced { '1' } else { '0' };
                to_data_field(tag, ind1, ' ', d.to_subfields())
            }
        })
    }
}
