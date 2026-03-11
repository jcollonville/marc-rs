use std::collections::HashMap;

use crate::datatypes::{AudienceCode, GeneralProcessingData, OrganizationNameType, PersonalNameType};
use crate::formats::{BlockId, FieldType, FormatDescriptor, SubfieldMapping, TagDescriptor};
use crate::record::{ControlField, DataField};

/// JSON schema for a single subfield definition.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonSubfield {
    pub code: char,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default = "default_subfield_type", rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub repeatable: bool,
    /// For coded fixed-length subfields: named slices within the raw value.
    #[serde(default)]
    pub slices: Vec<HashMap<String, JsonSlice>>,
}

fn default_subfield_type() -> String {
    "Raw".to_string()
}

/// JSON schema for a coded slice definition on fixed-length fields.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonSlice {
    pub range: [usize; 2],
    #[serde(rename = "target_type")]
    pub target_type: String,
    #[serde(default)]
    pub map: Option<HashMap<String, serde_json::Value>>,
}

/// JSON schema for a field mapping (one logical field in a block).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonFieldMapping {
    pub tag: String,
    #[serde(default)]
    pub is_control: bool,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub subfields: Vec<JsonSubfield>,
    #[serde(default)]
    pub note: Option<String>,
}

/// JSON schema for a format dictionary.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonDictionary {
    pub dialect: String,
    #[serde(default)]
    pub default_encoding: Option<String>,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
    #[serde(default)]
    pub indicator_maps: Option<serde_json::Value>,
    #[serde(default)]
    pub value_maps: Option<serde_json::Value>,
    /// mappings[block_name][field_name] = JsonFieldMapping
    pub mappings: HashMap<String, HashMap<String, JsonFieldMapping>>,
}

/// Flattened slice definition: (name, range, target_type, optional map).
#[derive(Debug, Clone)]
pub struct CodedSlice {
    pub name: String,
    pub range: [usize; 2],
    pub target_type: String,
    pub map: Option<HashMap<String, serde_json::Value>>,
}

/// In-memory representation of a JSON-driven format.
pub struct DictionaryFormat {
    name: String,
    descriptors: Vec<TagDescriptor>,
    by_tag: HashMap<String, usize>,
    by_block_field: HashMap<(BlockId, String), usize>,
    by_type: HashMap<FieldType, Vec<usize>>,
    /// Coded field slices keyed by field_name (e.g. "general_processing_data").
    coded_slices: HashMap<String, Vec<CodedSlice>>,
}

impl DictionaryFormat {
    pub fn from_json(name: &str, json: &str) -> serde_json::Result<Self> {
        let cfg: JsonDictionary = serde_json::from_str(json)?;

        let mut descriptors = Vec::new();
        let mut by_tag = HashMap::new();
        let mut by_block_field = HashMap::new();
        let mut by_type: HashMap<FieldType, Vec<usize>> = HashMap::new();
        let mut coded_slices: HashMap<String, Vec<CodedSlice>> = HashMap::new();

        for (block_name, fields) in cfg.mappings {
            let block = block_id_from_name(&block_name);
            for (field_name, field_cfg) in fields {
                let field_type = field_type_from_name(field_cfg.field_type.as_deref());

                let mut subfield_map: Vec<SubfieldMapping> = Vec::new();
                for sf in &field_cfg.subfields {
                    if !sf.slices.is_empty() {
                        // Coded subfield: collect slice definitions, add a "value" mapping
                        subfield_map.push(SubfieldMapping::Raw(sf.code, "value".to_string(), false));
                        let slices: Vec<CodedSlice> = sf.slices.iter()
                            .flat_map(|entry| entry.iter())
                            .map(|(slice_name, slice_def)| CodedSlice {
                                name: slice_name.clone(),
                                range: slice_def.range,
                                target_type: slice_def.target_type.clone(),
                                map: slice_def.map.clone(),
                            })
                            .collect();
                        coded_slices.insert(field_name.clone(), slices);
                    } else if let Some(ref sf_name) = sf.name {
                        subfield_map.push(subfield_mapping_from_json(sf.code, sf_name, &sf.kind, sf.repeatable));
                    }
                }

                let desc = TagDescriptor {
                    tag: field_cfg.tag.clone(),
                    block,
                    field: field_name.clone(),
                    field_type,
                    is_control: field_cfg.is_control,
                    subfield_map,
                };

                let idx = descriptors.len();
                by_tag.insert(desc.tag.clone(), idx);
                by_block_field.insert((block, desc.field.clone()), idx);
                by_type.entry(desc.field_type).or_default().push(idx);

                descriptors.push(desc);
            }
        }

        Ok(Self {
            name: name.to_string(),
            descriptors,
            by_tag,
            by_block_field,
            by_type,
            coded_slices,
        })
    }

    /// Extract slices from a raw coded string using the dictionary definition.
    pub fn extract_general_processing_data(&self, raw: &str) -> GeneralProcessingData {
        let slices = match self.coded_slices.get("general_processing_data") {
            Some(s) => s,
            None => return GeneralProcessingData { raw: raw.to_string(), ..Default::default() },
        };

        let b = raw.as_bytes();
        let mut gpd = GeneralProcessingData { raw: raw.to_string(), ..Default::default() };

        for slice in slices {
            let [start, end] = slice.range;
            if end > b.len() { continue; }
            let val = String::from_utf8_lossy(&b[start..end]).to_string();

            match (slice.name.as_str(), slice.target_type.as_str()) {
                ("entry_date", "String") => gpd.entry_date = non_empty_trimmed(&val),
                ("type_of_publication_date", "char") => gpd.type_of_publication_date = first_non_space_char(&val),
                ("date_1", "String") => gpd.date_1 = non_empty_trimmed(&val),
                ("date_2", "String") => gpd.date_2 = non_empty_trimmed(&val),
                ("audience_type", "String") => gpd.audience_type = non_empty_trimmed(&val),
                ("official_publication_type", "char") => gpd.official_publication_type = first_non_space_char(&val),
                ("modified_record", "char") => gpd.modified_record = first_non_space_char(&val),
                ("cataloging_language", "String") => gpd.cataloging_language = non_empty_trimmed(&val),
                ("transliteration", "char") => gpd.transliteration = first_non_space_char(&val),
                ("character_set", "String") => gpd.character_set = non_empty_trimmed(&val),
                ("additional_character_set", "String") => gpd.additional_character_set = non_empty_trimmed(&val),
                ("title_script", "String") => gpd.title_script = non_empty_trimmed(&val),
                _ => {}
            }
        }
        gpd
    }
}

fn block_id_from_name(name: &str) -> BlockId {
    match name {
        "identification" => BlockId::Identification,
        "coded_information" => BlockId::CodedInformation,
        "description" => BlockId::Description,
        "notes" => BlockId::Notes,
        "links" => BlockId::Links,
        "associated_titles" => BlockId::AssociatedTitles,
        "subject_analysis" => BlockId::SubjectAnalysis,
        "intellectual_responsibility" => BlockId::IntellectualResponsibility,
        "international_use" => BlockId::InternationalUse,
        "local_use" => BlockId::LocalUse,
        _ => BlockId::LocalUse,
    }
}

fn field_type_from_name(name: Option<&str>) -> FieldType {
    match name {
        Some("SimpleString") | Some("string") | Some("String") => FieldType::SimpleString,
        Some("StringList") => FieldType::StringList,
        Some("Isbn") => FieldType::Isbn,
        Some("TitleStatement") => FieldType::TitleStatement,
        Some("PhysicalDescription") => FieldType::PhysicalDescription,
        Some("EditionStatement") => FieldType::EditionStatement,
        Some("Publication") => FieldType::Publication,
        Some("SeriesStatement") => FieldType::SeriesStatement,
        Some("PersonalName") => FieldType::PersonalName,
        Some("CorporateName") => FieldType::CorporateName,
        Some("MeetingName") => FieldType::MeetingName,
        Some("UniformTitle") => FieldType::UniformTitle,
        Some("DeweyClassification") => FieldType::DeweyClassification,
        Some("Language") => FieldType::Language,
        Some("LinkingEntry") => FieldType::LinkingEntry,
        Some("SubjectEntry") => FieldType::SubjectEntry,
        Some("NoteEntry") => FieldType::NoteEntry,
        Some("Specimen") => FieldType::Specimen,
        // CodedField or anything unknown falls back to GenericDataField.
        _ => FieldType::GenericDataField,
    }
}

fn subfield_mapping_from_json(code: char, name: &str, kind: &str, repeatable: bool) -> SubfieldMapping {
    match kind {
        "Language" => SubfieldMapping::Language(code, name.to_string(), repeatable),
        "RelatorCode" => SubfieldMapping::RelatorCode(code, name.to_string(), repeatable),
        "CountryCode" => SubfieldMapping::CountryCode(code, name.to_string(), repeatable),
        "Date" => SubfieldMapping::Date(code, name.to_string(), repeatable),
        "Uri" => SubfieldMapping::Uri(code, name.to_string(), repeatable),
        "Identifier" => SubfieldMapping::Identifier(code, name.to_string(), repeatable),
        _ => SubfieldMapping::Raw(code, name.to_string(), repeatable),
    }
}

fn non_empty_trimmed(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() { None } else { Some(t.to_string()) }
}

fn first_non_space_char(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    if c == ' ' { None } else { Some(c) }
}

impl FormatDescriptor for DictionaryFormat {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag_descriptor(&self, tag: &str) -> Option<&TagDescriptor> {
        self.by_tag.get(tag).and_then(|idx| self.descriptors.get(*idx))
    }

    fn field_type_to_tag(&self, field_type: FieldType) -> Vec<&TagDescriptor> {
        self.by_type
            .get(&field_type)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .filter_map(|idx| self.descriptors.get(*idx))
            .collect()
    }

    fn field_to_tag(&self, block: BlockId, field: &str) -> Option<&TagDescriptor> {
        self.by_block_field
            .get(&(block, field.to_string()))
            .and_then(|idx| self.descriptors.get(*idx))
    }

    fn all_descriptors(&self) -> &[TagDescriptor] {
        &self.descriptors
    }

    fn personal_name_type_from_ind1(&self, _ind1: char) -> PersonalNameType {
        PersonalNameType::Surname
    }

    fn personal_name_type_to_ind1(&self, _name_type: &PersonalNameType) -> char {
        ' '
    }

    fn organization_name_type_from_ind1(&self, _ind1: char) -> OrganizationNameType {
        OrganizationNameType::DirectOrder
    }

    fn organization_name_type_to_ind1(&self, _name_type: &OrganizationNameType) -> char {
        ' '
    }

    fn audience_code_from_raw(&self, raw: char) -> AudienceCode {
        AudienceCode::from_unimarc_code(raw)
    }

    fn parse_general_processing_data(&self, raw: &str) -> GeneralProcessingData {
        self.extract_general_processing_data(raw)
    }

    fn audience_from_general_processing_data(&self, raw: &str) -> Option<AudienceCode> {
        // Default implementation: try to extract audience from fixed pos 17, like UNIMARC 100$a.
        let c = raw.as_bytes().get(17).copied().map(|b| b as char)?;
        if c == ' ' {
            return None;
        }
        Some(AudienceCode::from_unimarc_code(c))
    }

    fn decode_control(
        &self,
        _tag: &str,
        _value: &str,
    ) -> Option<ControlField> {
        None
    }

    fn decode_data(
        &self,
        _tag: &str,
        _ind1: char,
        _ind2: char,
        _subfields: &[(char, String)],
    ) -> Option<DataField> {
        None
    }
}

