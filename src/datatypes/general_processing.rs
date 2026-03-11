//! General processing data (format-agnostic).
//! For UNIMARC field 100, subfield $a is a fixed-length coded string; each format
//! (UNIMARC, MARC21) parses it via `FormatDescriptor::parse_general_processing_data`.

use serde::{Deserialize, Serialize};

/// Parsed general processing data. Fields are filled by the format when parsing
/// (e.g. UNIMARC 100 $a positions). Round-trip uses `raw`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeneralProcessingData {
    /// Raw string (for round-trip).
    #[serde(rename = "raw")]
    pub raw: String,

    /// Positions 0-7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_date: Option<String>,
    /// Position 8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_publication_date: Option<char>,
    /// Positions 9-12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_1: Option<String>,
    /// Positions 13-16
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_2: Option<String>,
    /// Positions 17-19
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_type: Option<String>,
    /// Position 20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official_publication_type: Option<char>,
    /// Position 21
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_record: Option<char>,
    /// Positions 22-24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cataloging_language: Option<String>,
    /// Position 25
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<char>,
    /// Positions 26-29
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    /// Positions 30-33
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_character_set: Option<String>,
    /// Positions 34-35
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_script: Option<String>,
}

impl GeneralProcessingData {
    /// Raw string for serialization (round-trip).
    pub fn to_raw_string(&self) -> String {
        self.raw.clone()
    }
}
