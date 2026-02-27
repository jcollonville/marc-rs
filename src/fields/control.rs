use serde::{Deserialize, Serialize};

use crate::format::MarcFormat;
use crate::record::ControlField;

/// Typed control fields (001-009)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Control {
    /// 001 - Control number
    ControlNumber(String),
    /// 003 - Control number identifier
    ControlNumberIdentifier(String),
    /// 005 - Date and time of latest transaction
    DateAndTimeOfLatestTransaction(String),
    /// 006 - Fixed-length data elements (MARC21 only)
    FixedLengthDataElementsAdditional(String),
    /// 007 - Physical description fixed field
    PhysicalDescriptionFixedField(String),
    /// 008 - Fixed-length data elements (MARC21)
    FixedLengthDataElements(String),
    /// 009 - Local control number (UNIMARC)
    LocalControlNumber(String),
}

impl Control {
    pub fn tag(&self, format: MarcFormat) -> Option<&'static str> {
        match (self, format) {
            (Control::ControlNumber(_), _) => Some("001"),
            (Control::ControlNumberIdentifier(_), _) => Some("003"),
            (Control::DateAndTimeOfLatestTransaction(_), _) => Some("005"),
            (Control::FixedLengthDataElementsAdditional(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("006"),
            (Control::FixedLengthDataElementsAdditional(_), MarcFormat::Unimarc) => None,
            (Control::PhysicalDescriptionFixedField(_), _) => Some("007"),
            (Control::FixedLengthDataElements(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => Some("008"),
            (Control::FixedLengthDataElements(_), MarcFormat::Unimarc) => None,
            (Control::LocalControlNumber(_), MarcFormat::Unimarc) => Some("009"),
            (Control::LocalControlNumber(_), MarcFormat::Marc21 | MarcFormat::MarcXml) => None,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Control::ControlNumber(v)
            | Control::ControlNumberIdentifier(v)
            | Control::DateAndTimeOfLatestTransaction(v)
            | Control::FixedLengthDataElementsAdditional(v)
            | Control::PhysicalDescriptionFixedField(v)
            | Control::FixedLengthDataElements(v)
            | Control::LocalControlNumber(v) => v,
        }
    }

    /// Parse a control field (tag < "010") into a typed variant.
    pub fn try_parse(tag: &str, value: &str, format: MarcFormat) -> Option<Self> {
        match (tag, format) {
            ("001", _) => Some(Control::ControlNumber(value.to_string())),
            ("003", _) => Some(Control::ControlNumberIdentifier(value.to_string())),
            ("005", _) => Some(Control::DateAndTimeOfLatestTransaction(value.to_string())),
            ("006", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some(Control::FixedLengthDataElementsAdditional(value.to_string()))
            }
            ("007", _) => Some(Control::PhysicalDescriptionFixedField(value.to_string())),
            ("008", MarcFormat::Marc21 | MarcFormat::MarcXml) => {
                Some(Control::FixedLengthDataElements(value.to_string()))
            }
            ("009", MarcFormat::Unimarc) => Some(Control::LocalControlNumber(value.to_string())),
            _ => None,
        }
    }

    /// Convert back to raw control field for writing.
    pub fn to_raw(&self, format: MarcFormat) -> Option<ControlField> {
        self.tag(format).map(|t| ControlField {
            tag: t.to_string(),
            value: self.value().to_string(),
        })
    }
}
