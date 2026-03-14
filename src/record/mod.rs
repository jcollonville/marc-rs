use marc_rs_derive::MarcPaths;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod types;
pub use types::*;

use crate::Encoding;

// ── Path resolution types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    VecPush,
    VecStructCreator,
    VecStructField,
    OptionInit,
    OptionSet,
}

pub trait MarcPaths: Sized {
    const IS_LEAF: bool;
    fn from_marc_str(s: &str) -> Self;
    fn to_marc_str(&self) -> String;
    fn marc_set(&mut self, path: &str, value: &str) -> bool;
    fn marc_get_option(&self, path: &str) -> Option<String>;
    fn marc_get_vec(&self, path: &str) -> Option<Vec<String>>;
    fn marc_path_kind(path: &str) -> Option<PathKind>;
    fn marc_has_path(path: &str) -> bool;
    fn marc_is_vec_leaf(path: &str) -> bool;
    fn marc_creator_field() -> &'static str;
}

// ── FromRuleValue: serde bridge for enum ↔ string conversion ────────────────

pub trait FromRuleValue: Sized + DeserializeOwned + Serialize {
    fn from_rule_value(s: &str) -> Self;
    fn to_rule_value(&self) -> String;
}

macro_rules! impl_from_rule_value {
    ($type:ty, $other:path) => {
        impl FromRuleValue for $type {
            fn from_rule_value(s: &str) -> Self {
                serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_else(|_| $other(s.to_string()))
            }
            fn to_rule_value(&self) -> String {
                match serde_json::to_value(self).ok() {
                    Some(serde_json::Value::String(s)) => s,
                    _ => match self {
                        $other(s) => s.clone(),
                        _ => unreachable!(),
                    },
                }
            }
        }
    };
}

impl_from_rule_value!(Language, Language::Other);
impl_from_rule_value!(Country, Country::Other);
impl_from_rule_value!(TargetAudience, TargetAudience::Other);
impl_from_rule_value!(ClassificationScheme, ClassificationScheme::Other);
impl_from_rule_value!(SubjectType, SubjectType::Other);
impl_from_rule_value!(NoteType, NoteType::Other);
impl_from_rule_value!(LinkType, LinkType::Other);

macro_rules! impl_from_rule_value_char {
    ($type:ty, $other:path) => {
        impl FromRuleValue for $type {
            fn from_rule_value(s: &str) -> Self {
                serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_else(|_| $other(s.chars().next().unwrap_or(' ')))
            }
            fn to_rule_value(&self) -> String {
                match serde_json::to_value(self).ok() {
                    Some(serde_json::Value::String(s)) => s,
                    _ => match self {
                        $other(c) => c.to_string(),
                        _ => unreachable!(),
                    },
                }
            }
        }
    };
}

impl_from_rule_value_char!(RecordStatus, RecordStatus::Other);
impl_from_rule_value_char!(RecordType, RecordType::Other);
impl_from_rule_value_char!(BibliographicLevel, BibliographicLevel::Other);

// ── MarcPaths leaf implementations for value enums ──────────────────────────

macro_rules! impl_marc_leaf {
    ($ty:ty) => {
        impl MarcPaths for $ty {
            const IS_LEAF: bool = true;
            fn from_marc_str(s: &str) -> Self {
                <$ty as FromRuleValue>::from_rule_value(s)
            }
            fn to_marc_str(&self) -> String {
                <$ty as FromRuleValue>::to_rule_value(self)
            }
            fn marc_set(&mut self, _: &str, _: &str) -> bool {
                false
            }
            fn marc_get_option(&self, _: &str) -> Option<String> {
                None
            }
            fn marc_get_vec(&self, _: &str) -> Option<Vec<String>> {
                None
            }
            fn marc_path_kind(_: &str) -> Option<PathKind> {
                None
            }
            fn marc_has_path(_: &str) -> bool {
                false
            }
            fn marc_is_vec_leaf(_: &str) -> bool {
                false
            }
            fn marc_creator_field() -> &'static str {
                ""
            }
        }
    };
}

impl_marc_leaf!(Language);
impl_marc_leaf!(Country);
impl_marc_leaf!(TargetAudience);
impl_marc_leaf!(ClassificationScheme);
impl_marc_leaf!(SubjectType);
impl_marc_leaf!(NoteType);
impl_marc_leaf!(LinkType);

/// High-level semantic representation of a MARC bibliographic record,
/// organized following the standard block numbering (0XX-9XX).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[marc(skip)]
    pub leader: Leader,
    #[marc(skip)]
    #[serde(skip)]
    pub encoding: Option<Encoding>,
    /// 0XX - Identification
    #[serde(default)]
    pub identification: Identification,
    /// 1XX - Coded information
    #[serde(default)]
    pub coded: Coded,
    /// 2XX - Descriptive information
    #[serde(default)]
    pub description: Description,
    /// 3XX - Notes
    #[serde(default)]
    pub notes: Notes,
    /// 4XX - Links to other bibliographic records
    #[serde(default)]
    pub links: Links,
    /// 5XX - Associated titles
    #[serde(default)]
    pub associated_titles: AssociatedTitles,
    /// 6XX - Subject indexing
    #[serde(default)]
    pub indexing: Indexing,
    /// 7XX - Responsibility
    #[serde(default)]
    pub responsibility: Responsibility,
    /// 8XX - International data
    #[serde(default)]
    pub international: International,
    /// 9XX - National and local data
    #[serde(default)]
    pub local: Local,
}

impl Record {
    pub fn authors(&self) -> impl Iterator<Item = &Agent> {
        self.responsibility.main_entry.iter().chain(self.responsibility.added_entries.iter())
    }

    pub fn languages(&self) -> &[Language] {
        &self.coded.languages
    }

    pub fn titles(&self) -> Vec<&Title> {
        let mut out = Vec::new();
        if let Some(t) = &self.description.title {
            out.push(t);
        }
        if let Some(t) = &self.associated_titles.uniform_title {
            out.push(t);
        }
        out
    }

    pub fn audience(&self) -> Option<&TargetAudience> {
        self.coded.target_audience.as_ref()
    }

    pub fn isbn(&self) -> &[Isbn] {
        &self.identification.isbn
    }

    pub fn specimens(&self) -> &[Specimen] {
        &self.local.specimens
    }
}

/// 0XX - Identification block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_version_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isbn: Vec<Isbn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issn: Vec<Issn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_bibliography_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_library_record_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal_deposit_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lccn: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub system_control_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patent_numbers: Vec<PatentNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technical_report_numbers: Vec<TechnicalReportNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publisher_numbers: Vec<PublisherNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codens: Vec<Coden>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub original_study_numbers: Vec<OriginalStudyNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub government_document_numbers: Vec<GovernmentDocumentNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_numbers: Vec<ReportNumber>,
}

/// 1XX - Coded information block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Coded {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<Language>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub original_languages: Vec<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_dates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<TargetAudience>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geographic_area_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_period_codes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_entered_on_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government_publication: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cataloging_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_character_set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_of_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_publication_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cataloging_source_code: Option<String>,
}

/// 2XX - Descriptive information block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publication: Vec<Publication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_description: Option<PhysicalDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<SeriesStatement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub varying_titles: Vec<VaryingTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

/// 3XX - Notes block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Note>,
}

/// 4XX - Links to other bibliographic records
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<LinkedRecord>,
}

/// 5XX - Associated titles block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedTitles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_title: Option<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub former_titles: Vec<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variant_titles: Vec<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviated_title: Option<String>,
}

/// 6XX - Subject indexing block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Indexing {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<Classification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uncontrolled_terms: Vec<String>,
}

/// 7XX - Responsibility block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Responsibility {
    #[marc(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry: Option<Agent>,
    #[marc(skip)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entries: Vec<Agent>,
}

/// 8XX - International data block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct International {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cataloging_sources: Vec<CatalogingSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location_call_numbers: Vec<LocationCallNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub electronic_locations: Vec<ElectronicLocation>,
    /// MARC21 850 - Holding institution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding_institutions: Vec<String>,
}

/// 9XX - National and local data block
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimens: Vec<Specimen>,
}
