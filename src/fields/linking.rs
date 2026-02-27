use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Linking entry fields (76X-78X in MARC21, 4XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Linking {
    MainSeriesEntry(LinkingData),
    SubseriesEntry(LinkingData),
    OriginalLanguageEntry(LinkingData),
    TranslationEntry(LinkingData),
    SupplementSpecialIssueEntry(LinkingData),
    SupplementParentEntry(LinkingData),
    HostItemEntry(LinkingData),
    ConstituentUnitEntry(LinkingData),
    OtherEditionEntry(LinkingData),
    AdditionalPhysicalFormEntry(LinkingData),
    IssuedWithEntry(LinkingData),
    PrecedingEntry(LinkingData),
    SucceedingEntry(LinkingData),
    DataSourceEntry(LinkingData),
    OtherRelationshipEntry(LinkingData),
}

impl Linking {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Linking::MainSeriesEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("760"),
            (Linking::MainSeriesEntry(_), MarcFormat::Unimarc) => Some("410"),
            (Linking::SubseriesEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("762"),
            (Linking::SubseriesEntry(_), MarcFormat::Unimarc) => Some("411"),
            (Linking::OriginalLanguageEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("765"),
            (Linking::OriginalLanguageEntry(_), MarcFormat::Unimarc) => Some("454"),
            (Linking::TranslationEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("767"),
            (Linking::TranslationEntry(_), MarcFormat::Unimarc) => Some("454"),
            (Linking::SupplementSpecialIssueEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("770"),
            (Linking::SupplementSpecialIssueEntry(_), MarcFormat::Unimarc) => Some("488"),
            (Linking::SupplementParentEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("772"),
            (Linking::SupplementParentEntry(_), MarcFormat::Unimarc) => Some("488"),
            (Linking::HostItemEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("773"),
            (Linking::HostItemEntry(_), MarcFormat::Unimarc) => Some("461"),
            (Linking::ConstituentUnitEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("774"),
            (Linking::ConstituentUnitEntry(_), MarcFormat::Unimarc) => Some("462"),
            (Linking::OtherEditionEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("775"),
            (Linking::OtherEditionEntry(_), MarcFormat::Unimarc) => Some("453"),
            (Linking::AdditionalPhysicalFormEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("776"),
            (Linking::AdditionalPhysicalFormEntry(_), MarcFormat::Unimarc) => Some("452"),
            (Linking::IssuedWithEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("777"),
            (Linking::IssuedWithEntry(_), MarcFormat::Unimarc) => Some("488"),
            (Linking::PrecedingEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("780"),
            (Linking::PrecedingEntry(_), MarcFormat::Unimarc) => Some("430"),
            (Linking::SucceedingEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("785"),
            (Linking::SucceedingEntry(_), MarcFormat::Unimarc) => Some("431"),
            (Linking::DataSourceEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("786"),
            (Linking::DataSourceEntry(_), MarcFormat::Unimarc) => None,
            (Linking::OtherRelationshipEntry(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("787"),
            (Linking::OtherRelationshipEntry(_), MarcFormat::Unimarc) => Some("488"),
        }
    }

    fn data(&self) -> &LinkingData {
        match self {
            Linking::MainSeriesEntry(d)
            | Linking::SubseriesEntry(d)
            | Linking::OriginalLanguageEntry(d)
            | Linking::TranslationEntry(d)
            | Linking::SupplementSpecialIssueEntry(d)
            | Linking::SupplementParentEntry(d)
            | Linking::HostItemEntry(d)
            | Linking::ConstituentUnitEntry(d)
            | Linking::OtherEditionEntry(d)
            | Linking::AdditionalPhysicalFormEntry(d)
            | Linking::IssuedWithEntry(d)
            | Linking::PrecedingEntry(d)
            | Linking::SucceedingEntry(d)
            | Linking::DataSourceEntry(d)
            | Linking::OtherRelationshipEntry(d) => d,
        }
    }

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        let d = LinkingData::from_subfields(ind1, ind2, subfields, format);
        let link = match (tag, format) {
            ("760", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::MainSeriesEntry(d),
            ("762", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::SubseriesEntry(d),
            ("765", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("454", MarcFormat::Unimarc) => {
                Linking::OriginalLanguageEntry(d)
            }
            ("767", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::TranslationEntry(d),
            ("770", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::SupplementSpecialIssueEntry(d),
            ("772", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::SupplementParentEntry(d),
            ("773", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("461", MarcFormat::Unimarc) => {
                Linking::HostItemEntry(d)
            }
            ("774", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("462", MarcFormat::Unimarc) => {
                Linking::ConstituentUnitEntry(d)
            }
            ("775", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("453", MarcFormat::Unimarc) => {
                Linking::OtherEditionEntry(d)
            }
            ("776", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("452", MarcFormat::Unimarc) => {
                Linking::AdditionalPhysicalFormEntry(d)
            }
            ("777", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("488", MarcFormat::Unimarc) => {
                Linking::IssuedWithEntry(d)
            }
            ("780", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("430", MarcFormat::Unimarc) => {
                Linking::PrecedingEntry(d)
            }
            ("785", MarcFormat::Marc21 | MarcFormat::MarcXml) | ("431", MarcFormat::Unimarc) => {
                Linking::SucceedingEntry(d)
            }
            ("786", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::DataSourceEntry(d),
            ("787", MarcFormat::Marc21 | MarcFormat::MarcXml) => Linking::OtherRelationshipEntry(d),
            _ => return None,
        };
        Some(link)
    }

    pub fn to_raw(&self, format: MarcFormat) -> Option<DataField> {
        let tag = self.tag(format)?;
        let d = self.data();
        Some(to_data_field(tag, d.display_note_ind1(), ' ', d.to_subfields()))
    }
}
