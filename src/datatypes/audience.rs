use serde::{Deserialize, Serialize};

/// Target audience code extracted from fixed-length coded data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudienceCode {
    /// UNIMARC 100/17: a (jeunesse); MARC21 008/22: j (juvenile).
    Juvenile,
    /// UNIMARC 100/17: b; MARC21 008/22: a.
    Preschool,
    /// UNIMARC 100/17: c; MARC21 008/22: b.
    Primary,
    /// MARC21 008/22: c.
    ElementaryAndJuniorHigh,
    /// UNIMARC 100/17: d; MARC21 008/22: d.
    Adolescent,
    /// UNIMARC 100/17: e.
    Adult,
    /// MARC21 008/22: g.
    General,
    /// UNIMARC 100/17: k.
    Specialist,
    /// UNIMARC 100/17: m.
    AdultPrintDisabled,
    /// UNIMARC 100/17: u.
    Unknown,
    Other(char),
}

