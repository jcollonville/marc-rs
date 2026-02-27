use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Main entry fields (1XX in MARC21, 7XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MainEntry {
    /// Personal name (100 in MARC21, 700 in UNIMARC)
    PersonalName(PersonalNameData),
    /// Corporate name (110 in MARC21, 710 in UNIMARC)
    CorporateName(CorporateNameData),
    /// Meeting name (111 in MARC21, 711 in UNIMARC)
    MeetingName(MeetingNameData),
    /// Uniform title (130 in MARC21, 730 in UNIMARC)
    UniformTitle(UniformTitleData),
}

impl MainEntry {
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        match (self, format) {
            (MainEntry::PersonalName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "100",
            (MainEntry::PersonalName(_), MarcFormat::Unimarc) => "700",
            (MainEntry::CorporateName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "110",
            (MainEntry::CorporateName(_), MarcFormat::Unimarc) => "710",
            (MainEntry::MeetingName(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "111",
            (MainEntry::MeetingName(_), MarcFormat::Unimarc) => "711",
            (MainEntry::UniformTitle(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => "130",
            (MainEntry::UniformTitle(_), MarcFormat::Unimarc) => "730",
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
            ("100", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("700", MarcFormat::Unimarc) => {
                PersonalNameData::from_subfields(ind1, ind2, subfields)
                    .map(MainEntry::PersonalName)
            }
            ("110", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("710", MarcFormat::Unimarc) => {
                CorporateNameData::from_subfields(ind1, ind2, subfields)
                    .map(MainEntry::CorporateName)
            }
            ("111", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("711", MarcFormat::Unimarc) => {
                MeetingNameData::from_subfields(ind1, ind2, subfields)
                    .map(MainEntry::MeetingName)
            }
            ("130", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("730", MarcFormat::Unimarc) => {
                UniformTitleData::from_subfields(ind1, ind2, subfields)
                    .map(MainEntry::UniformTitle)
            }
            _ => None,
        }
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = self.tag(format);
        match self {
            MainEntry::PersonalName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            MainEntry::CorporateName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            MainEntry::MeetingName(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
            MainEntry::UniformTitle(d) => to_data_field(tag, d.ind1, d.ind2, d.to_subfields()),
        }
    }
}
