use serde::Deserialize;
use std::collections::HashMap;

/// Root of a dictionary file (marc21.json / unimarc.json).
/// Contains the full configuration for a MARC format: leader, field blocks, and translation tables.
#[derive(Debug, Deserialize)]
pub struct CatalogConfig {
    pub name: String,
    #[serde(default)]
    pub leader: Vec<LeaderPositionDef>,
    #[serde(default)]
    pub directory_map: Option<String>,
    #[serde(default)]
    pub encoding_indicator: Option<EncodingIndicatorDef>,
    /// Named reusable translation tables (e.g. "languages", "audience").
    #[serde(default)]
    pub rules: HashMap<String, SharedRulesTable>,
    pub blocks: Vec<BlockDef>,
}

/// Extracts one or more bytes from the ISO2709 leader (24 fixed bytes) and maps them
/// to a semantic field of the Record model via `target` (e.g. "record_type").
#[derive(Debug, Deserialize)]
pub struct LeaderPositionDef {
    pub position: usize,
    #[serde(default = "default_one")]
    pub length: usize,
    pub target: String,
    pub rules: Option<RulesRef>,
    pub default_raw: Option<String>,
    /// Optional Rust regex: translated leader fragment must match before mapping.
    #[serde(default)]
    pub pattern: Option<String>,
}

fn default_one() -> usize {
    1
}

/// Indicates where to find the character encoding in the record.
/// MARC21: byte 9 of the leader. UNIMARC: subfield $a of field 100, positions 26-28.
#[derive(Debug, Deserialize)]
pub struct EncodingIndicatorDef {
    pub leader_position: Option<usize>,
    pub tag: Option<String>,
    pub subfield: Option<String>,
    pub slice: Option<SliceDef>,
    pub rules: Vec<EncodingRule>,
    #[serde(default)]
    pub default_raw: Option<String>,
}

/// Associates a raw value (e.g. "50" in UNIMARC) with an encoding name (e.g. "utf8").
#[derive(Debug, Clone, Deserialize)]
pub struct EncodingRule {
    pub encoding: String,
    pub raw: String,
}

/// Named translation table: a list of raw→value rules with an optional default.
/// Referenced by name from fields to avoid duplication (e.g. the same language table
/// used by field 101 in both MARC21 and UNIMARC).
#[derive(Debug, Deserialize)]
pub struct SharedRulesTable {
    #[serde(default)]
    pub default: Option<String>,
    pub entries: Vec<TranslationRule>,
}

/// Group of fields belonging to the same bibliographic block (0XX, 1XX, 2XX…).
#[derive(Debug, Deserialize)]
pub struct BlockDef {
    pub id: String,
    pub label: Option<String>,
    pub fields: Vec<FieldDef>,
}

/// Descriptor for a MARC field (3-digit tag).
/// Can be a control field (target only) or a data field (subfields).
#[derive(Debug, Deserialize)]
pub struct FieldDef {
    pub tag: String,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    #[serde(default)]
    pub indicators: Option<[String; 2]>,
    /// Default values injected into the Record if this field is absent from the raw record.
    #[serde(default)]
    pub defaults: Option<HashMap<String, String>>,
    /// If false, this field is skipped during serialization (Record → binary).
    #[serde(default)]
    pub reverse: Option<bool>,
    pub target: Option<String>,
    pub slice: Option<SliceDef>,
    pub rules: Option<RulesRef>,
    pub subfields: Option<Vec<SubfieldBinding>>,
    pub length: Option<usize>,
    pub mandatory: Option<bool>,
    /// Optional Rust regex for control field value (after slice/trim/translate).
    #[serde(default)]
    pub pattern: Option<String>,
}

/// Maps a MARC subfield (e.g. $a, $b) to a dotted path in the Record model
/// (e.g. "description.title.main"). Supports substring extraction (slice) and
/// value translation (rules).
#[derive(Debug, Deserialize)]
pub struct SubfieldBinding {
    pub code: String,
    pub target: String,
    pub slice: Option<SliceDef>,
    pub trim: Option<String>,
    pub rules: Option<RulesRef>,
    pub default: Option<String>,
    pub length: Option<usize>,
    /// Optional Rust regex: translated value must match before mapping to the record path.
    #[serde(default)]
    pub pattern: Option<String>,
}

/// Reference to a translation table: either the name of a shared table (e.g. "languages"),
/// or an inline list of rules directly in the field.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RulesRef {
    Ref(String),
    Inline(Vec<TranslationRule>),
}

/// Extracts a positional substring from a fixed-length subfield
/// (e.g. UNIMARC 100$a: offset 0, length 8 → date entered on file).
#[derive(Debug, Clone, Deserialize)]
pub struct SliceDef {
    pub offset: usize,
    pub length: usize,
}

/// A translation rule: raw MARC value → semantic name (e.g. "fre" → "french").
#[derive(Debug, Clone, Deserialize)]
pub struct TranslationRule {
    pub raw: String,
    pub value: String,
}
