use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CatalogConfig {
    pub name: String,
    #[serde(default)]
    pub leader: Vec<LeaderPositionDef>,
    #[serde(default)]
    pub directory_map: Option<String>,
    #[serde(default)]
    pub encoding_indicator: Option<EncodingIndicatorDef>,
    #[serde(default)]
    pub rules: HashMap<String, SharedRulesTable>,
    pub blocks: Vec<BlockDef>,
}

#[derive(Debug, Deserialize)]
pub struct LeaderPositionDef {
    pub position: usize,
    #[serde(default = "default_one")]
    pub length: usize,
    pub target: String,
    pub rules: Option<RulesRef>,
    pub default_raw: Option<String>,
}

fn default_one() -> usize {
    1
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct EncodingRule {
    pub encoding: String,
    pub raw: String,
}

#[derive(Debug, Deserialize)]
pub struct SharedRulesTable {
    #[serde(default)]
    pub default: Option<String>,
    pub entries: Vec<TranslationRule>,
}

#[derive(Debug, Deserialize)]
pub struct BlockDef {
    pub id: String,
    pub label: Option<String>,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Deserialize)]
pub struct FieldDef {
    pub tag: String,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    #[serde(default)]
    pub indicators: Option<[String; 2]>,
    #[serde(default)]
    pub defaults: Option<HashMap<String, String>>,
    #[serde(default)]
    pub reverse: Option<bool>,
    pub target: Option<String>,
    pub slice: Option<SliceDef>,
    pub rules: Option<RulesRef>,
    pub subfields: Option<Vec<SubfieldBinding>>,
    pub length: Option<usize>,
    pub mandatory: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SubfieldBinding {
    pub code: String,
    pub target: String,
    pub slice: Option<SliceDef>,
    pub trim: Option<String>,
    pub rules: Option<RulesRef>,
    pub default: Option<String>,
    pub length: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RulesRef {
    Ref(String),
    Inline(Vec<TranslationRule>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SliceDef {
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranslationRule {
    pub raw: String,
    pub value: String,
}
