use std::collections::HashMap;

use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::format::build_iso2709;
use crate::format::config::*;
use crate::format::LeaderLayout;
use crate::format::ReverseFieldData;
use crate::raw::OwnedRawRecord;
use crate::raw::{RawField, RawRecord};
use crate::record::*;

// ---------------------------------------------------------------------------
// ConfigError
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum ConfigError {
    UnknownPath(String),
    UnknownRulesRef(String),
    InvalidDefaults(String),
    DuplicateTag(String),
    Json(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::UnknownPath(p) => write!(f, "unknown target path: {}", p),
            ConfigError::UnknownRulesRef(r) => write!(f, "unknown rules reference: {}", r),
            ConfigError::InvalidDefaults(d) => write!(f, "invalid defaults key: {}", d),
            ConfigError::DuplicateTag(t) => write!(f, "duplicate tag: {}", t),
            ConfigError::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

// ---------------------------------------------------------------------------
// Path resolver: known paths -> typed getter/setter closures
// ---------------------------------------------------------------------------

type Setter = Box<dyn Fn(&mut Record, &str) + Send + Sync>;
type VecGetter = Box<dyn Fn(&Record) -> Vec<String> + Send + Sync>;
type OptionGetter = Box<dyn Fn(&Record) -> Option<String> + Send + Sync>;

struct ResolvedPath {
    kind: PathKind,
    setter: Setter,
    vec_getter: Option<VecGetter>,
    option_getter: Option<OptionGetter>,
}

fn resolve_path(path: &str) -> Result<ResolvedPath, ConfigError> {
    // Agent paths are handled manually (enum-based dispatch)
    if path.starts_with("responsibility.main_entry.") || path.starts_with("responsibility.added_entries.") {
        return resolve_agent_path(path);
    }

    let kind = Record::marc_path_kind(path).ok_or_else(|| ConfigError::UnknownPath(path.to_string()))?;

    let ps = path.to_string();
    let pv = path.to_string();
    let po = path.to_string();

    let setter: Setter = Box::new(move |rec, v| {
        rec.marc_set(&ps, v);
    });

    let vec_getter: Option<VecGetter> = match kind {
        PathKind::VecPush | PathKind::VecStructCreator | PathKind::VecStructField => Some(Box::new(move |rec| rec.marc_get_vec(&pv).unwrap_or_default())),
        _ => None,
    };

    let option_getter: Option<OptionGetter> = match kind {
        PathKind::OptionSet | PathKind::OptionInit => Some(Box::new(move |rec| rec.marc_get_option(&po))),
        _ => None,
    };

    Ok(ResolvedPath {
        kind,
        setter,
        vec_getter,
        option_getter,
    })
}

fn resolve_agent_path(path: &str) -> Result<ResolvedPath, ConfigError> {
    match path {
        // -- main_entry getters --
        "responsibility.main_entry.name" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => Some(p.name.clone()),
                Some(Agent::CorporateBody(c)) => Some(c.name.clone()),
                Some(Agent::Meeting(m)) => Some(m.name.clone()),
                None => None,
            })),
        }),
        "responsibility.main_entry.forename" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.forename.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.dates" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.dates.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.numeration" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.numeration.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.titles_associated" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.titles_associated.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.fuller_form" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.fuller_form.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.relator" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::Person(p)) => p.relator.as_ref().map(|r| r.to_marc_str()),
                _ => None,
            })),
        }),
        "responsibility.main_entry.subordinate_unit" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::CorporateBody(c)) => c.subordinate_unit.clone(),
                Some(Agent::Meeting(m)) => m.subordinate_unit.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.location" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::CorporateBody(c)) => c.location.clone(),
                Some(Agent::Meeting(m)) => m.location.clone(),
                _ => None,
            })),
        }),
        "responsibility.main_entry.date" => Ok(ResolvedPath {
            kind: PathKind::OptionInit,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: None,
            option_getter: Some(Box::new(|rec| match &rec.responsibility.main_entry {
                Some(Agent::CorporateBody(c)) => c.date.clone(),
                Some(Agent::Meeting(m)) => m.date.clone(),
                _ => None,
            })),
        }),
        // -- added_entries getters --
        "responsibility.added_entries.name" => Ok(ResolvedPath {
            kind: PathKind::VecStructCreator,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .map(|a| match a {
                        Agent::Person(p) => p.name.clone(),
                        Agent::CorporateBody(c) => c.name.clone(),
                        Agent::Meeting(m) => m.name.clone(),
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.forename" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.forename.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.dates" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.dates.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.numeration" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.numeration.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.titles_associated" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.titles_associated.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.fuller_form" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.fuller_form.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.relator" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::Person(p) => p.relator.as_ref().map(|r| r.to_marc_str()),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.subordinate_unit" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::CorporateBody(c) => c.subordinate_unit.clone(),
                        Agent::Meeting(m) => m.subordinate_unit.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.location" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::CorporateBody(c) => c.location.clone(),
                        Agent::Meeting(m) => m.location.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        "responsibility.added_entries.date" => Ok(ResolvedPath {
            kind: PathKind::VecStructField,
            setter: Box::new(|_rec, _v| {}),
            vec_getter: Some(Box::new(|rec| {
                rec.responsibility
                    .added_entries
                    .iter()
                    .filter_map(|a| match a {
                        Agent::CorporateBody(c) => c.date.clone(),
                        Agent::Meeting(m) => m.date.clone(),
                        _ => None,
                    })
                    .collect()
            })),
            option_getter: None,
        }),
        _ => Err(ConfigError::UnknownPath(path.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Compiled config: pre-resolved paths for runtime efficiency
// ---------------------------------------------------------------------------

struct CompiledBinding {
    code: u8,
    target_path: String,
    resolved: ResolvedPath,
    slice: Option<SliceDef>,
    length: Option<usize>,
    trim_chars: Option<Vec<char>>,
    rules_entries: Option<Vec<TranslationRule>>,
    rules_default: Option<String>,
}

struct CompiledField {
    tag: [u8; 3],
    field_type: Option<String>,
    indicators: [u8; 2],
    defaults: Option<HashMap<String, String>>,
    reverse_enabled: bool,
    length: Option<usize>,
    mandatory: bool,
    control_target: Option<CompiledControlTarget>,
    bindings: Vec<CompiledBinding>,
}

struct CompiledControlTarget {
    resolved: ResolvedPath,
    slice: Option<SliceDef>,
    length: Option<usize>,
    rules_entries: Option<Vec<TranslationRule>>,
    rules_default: Option<String>,
}

struct CompiledLeaderPosition {
    position: usize,
    length: usize,
    target: String,
    rules_entries: Option<Vec<TranslationRule>>,
    rules_default: Option<String>,
    default_raw: Option<String>,
}

struct CompiledEncodingIndicator {
    leader_position: Option<usize>,
    tag: Option<[u8; 3]>,
    subfield: Option<u8>,
    slice: Option<SliceDef>,
    rules: Vec<EncodingRule>,
    default_raw: Option<String>,
}

pub struct CompiledConfig {
    leader_positions: Vec<CompiledLeaderPosition>,
    directory_map: [u8; 4],
    encoding_indicator: Option<CompiledEncodingIndicator>,
    fields: Vec<CompiledField>,
}

// ---------------------------------------------------------------------------
// Rules resolution helpers
// ---------------------------------------------------------------------------

fn resolve_rules(rules_ref: &Option<RulesRef>, shared: &HashMap<String, SharedRulesTable>, binding_default: &Option<String>) -> Result<(Option<Vec<TranslationRule>>, Option<String>), ConfigError> {
    match rules_ref {
        None => Ok((None, None)),
        Some(RulesRef::Inline(entries)) => Ok((Some(entries.clone()), binding_default.clone())),
        Some(RulesRef::Ref(name)) => {
            let table = shared.get(name).ok_or_else(|| ConfigError::UnknownRulesRef(name.clone()))?;
            let default = binding_default.clone().or_else(|| table.default.clone());
            Ok((Some(table.entries.clone()), default))
        }
    }
}

fn forward_translate(raw: &str, entries: &Option<Vec<TranslationRule>>, default: &Option<String>) -> String {
    match entries {
        None => raw.to_string(),
        Some(entries) => {
            for rule in entries {
                if rule.raw.trim().eq_ignore_ascii_case(raw) {
                    return rule.value.clone();
                }
            }
            default.clone().unwrap_or_else(|| raw.to_string())
        }
    }
}

fn reverse_translate(value: &str, entries: &Option<Vec<TranslationRule>>) -> String {
    match entries {
        None => value.to_string(),
        Some(entries) => {
            for rule in entries {
                if rule.value.eq_ignore_ascii_case(value) {
                    return rule.raw.clone();
                }
            }
            value.to_string()
        }
    }
}

fn parse_tag(s: &str) -> [u8; 3] {
    let bytes = s.as_bytes();
    let mut tag = [b' '; 3];
    let len = bytes.len().min(3);
    tag[..len].copy_from_slice(&bytes[..len]);
    tag
}

fn parse_indicators(ind: &Option<[String; 2]>) -> [u8; 2] {
    match ind {
        Some(arr) => {
            let i1 = arr[0].as_bytes().first().copied().unwrap_or(b' ');
            let i2 = arr[1].as_bytes().first().copied().unwrap_or(b' ');
            [i1, i2]
        }
        None => [b' ', b' '],
    }
}

// ---------------------------------------------------------------------------
// Compile config
// ---------------------------------------------------------------------------

impl CatalogConfig {
    pub fn compile(&self) -> Result<CompiledConfig, ConfigError> {
        let mut fields = Vec::new();
        for block in &self.blocks {
            for field_def in &block.fields {
                let tag = parse_tag(&field_def.tag);
                let indicators = parse_indicators(&field_def.indicators);
                let reverse_enabled = field_def.reverse.unwrap_or(true);
                let is_control = field_def.field_type.as_deref() == Some("control");

                let control_target = if is_control {
                    if let Some(target) = &field_def.target {
                        let resolved = resolve_path(target)?;
                        let (rules_entries, rules_default) = resolve_rules(&field_def.rules, &self.rules, &None)?;
                        Some(CompiledControlTarget {
                            resolved,
                            slice: field_def.slice.clone(),
                            length: field_def.length,
                            rules_entries,
                            rules_default,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };

                let mut bindings = Vec::new();
                if let Some(subs) = &field_def.subfields {
                    for sub in subs {
                        let resolved = resolve_path(&sub.target)?;
                        let (rules_entries, rules_default) = resolve_rules(&sub.rules, &self.rules, &sub.default)?;
                        let trim_chars = sub.trim.as_ref().map(|t| t.chars().collect());
                        let code = sub.code.as_bytes().first().copied().unwrap_or(b' ');
                        bindings.push(CompiledBinding {
                            code,
                            target_path: sub.target.clone(),
                            resolved,
                            slice: sub.slice.clone(),
                            length: sub.length,
                            trim_chars,
                            rules_entries,
                            rules_default,
                        });
                    }
                }

                fields.push(CompiledField {
                    tag,
                    field_type: field_def.field_type.clone(),
                    indicators,
                    defaults: field_def.defaults.clone(),
                    reverse_enabled,
                    length: field_def.length,
                    mandatory: field_def.mandatory.unwrap_or(false),
                    control_target,
                    bindings,
                });
            }
        }
        let mut leader_positions = Vec::new();
        for lp in &self.leader {
            let (rules_entries, rules_default) = resolve_rules(&lp.rules, &self.rules, &None)?;
            leader_positions.push(CompiledLeaderPosition {
                position: lp.position,
                length: lp.length,
                target: lp.target.clone(),
                rules_entries,
                rules_default,
                default_raw: lp.default_raw.clone(),
            });
        }

        let mut directory_map = *b"4500";
        if let Some(dm) = &self.directory_map {
            let bytes = dm.as_bytes();
            let len = bytes.len().min(4);
            directory_map[..len].copy_from_slice(&bytes[..len]);
            for i in len..4 {
                directory_map[i] = b' ';
            }
        }

        let encoding_indicator = self.encoding_indicator.as_ref().map(|ei| CompiledEncodingIndicator {
            leader_position: ei.leader_position,
            tag: ei.tag.as_ref().map(|t| parse_tag(t)),
            subfield: ei.subfield.as_ref().and_then(|s| s.as_bytes().first().copied()),
            slice: ei.slice.clone(),
            rules: ei.rules.clone(),
            default_raw: ei.default_raw.clone(),
        });

        Ok(CompiledConfig {
            leader_positions,
            directory_map,
            encoding_indicator,
            fields,
        })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        let mut seen_tags = HashMap::new();
        for block in &self.blocks {
            for field_def in &block.fields {
                // Check for duplicate tags (warning-level, not fatal)
                let count = seen_tags.entry(field_def.tag.clone()).or_insert(0u32);
                *count += 1;

                let is_control = field_def.field_type.as_deref() == Some("control");

                if is_control {
                    if let Some(target) = &field_def.target {
                        resolve_path(target)?;
                    }
                    if let Some(RulesRef::Ref(name)) = &field_def.rules {
                        if !self.rules.contains_key(name) {
                            return Err(ConfigError::UnknownRulesRef(name.clone()));
                        }
                    }
                }

                if let Some(subs) = &field_def.subfields {
                    for sub in subs {
                        resolve_path(&sub.target)?;
                        if let Some(RulesRef::Ref(name)) = &sub.rules {
                            if !self.rules.contains_key(name) {
                                return Err(ConfigError::UnknownRulesRef(name.clone()));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Leader field accessors
// ---------------------------------------------------------------------------

fn set_leader_field(leader: &mut Leader, target: &str, value: &str) {
    match target {
        "status" => leader.status = RecordStatus::from_rule_value(value),
        "record_type" => leader.record_type = RecordType::from_rule_value(value),
        "bibliographic_level" => leader.bibliographic_level = BibliographicLevel::from_rule_value(value),
        "encoding_level" => leader.encoding_level = Some(value.to_string()),
        "descriptive_cataloging_form" => leader.descriptive_cataloging_form = Some(value.to_string()),
        other => {
            leader.extra.insert(other.to_string(), value.to_string());
        }
    }
}

fn get_leader_field(leader: &Leader, target: &str) -> Option<String> {
    match target {
        "status" => Some(leader.status.to_rule_value()),
        "record_type" => Some(leader.record_type.to_rule_value()),
        "bibliographic_level" => Some(leader.bibliographic_level.to_rule_value()),
        "encoding_level" => leader.encoding_level.clone(),
        "descriptive_cataloging_form" => leader.descriptive_cataloging_form.clone(),
        other => leader.extra.get(other).cloned(),
    }
}

// ---------------------------------------------------------------------------
// Forward mapping: raw -> Record
// ---------------------------------------------------------------------------

impl CompiledConfig {
    pub fn to_record(&self, encoding: &Encoding, raw: &RawRecord<'_>) -> Result<Record, MarcError> {
        let mut rec = Record {
            leader: Leader::default(),
            encoding: Some(encoding.clone()),
            identification: Identification::default(),
            coded: Coded::default(),
            description: Description::default(),
            notes: Notes::default(),
            links: Links::default(),
            associated_titles: AssociatedTitles::default(),
            indexing: Indexing::default(),
            responsibility: Responsibility::default(),
            international: International::default(),
            local: Local::default(),
        };

        if let Ok(leader_bytes) = raw.leader() {
            for lp in &self.leader_positions {
                let end = (lp.position + lp.length).min(leader_bytes.len());
                if lp.position >= end {
                    continue;
                }
                let raw_str = std::str::from_utf8(&leader_bytes[lp.position..end]).unwrap_or("").trim();
                if raw_str.is_empty() {
                    continue;
                }
                let translated = forward_translate(raw_str, &lp.rules_entries, &lp.rules_default);
                set_leader_field(&mut rec.leader, &lp.target, &translated);
            }
        }

        for raw_field in raw.fields()? {
            let field_tag = match raw_field {
                RawField::Control { tag, .. } | RawField::Data { tag, .. } => tag,
            };
            for compiled in &self.fields {
                if compiled.tag != field_tag {
                    continue;
                }
                match &raw_field {
                    RawField::Control { data, .. } => {
                        if let Some(ct) = &compiled.control_target {
                            let text = match encoding.decode(data) {
                                Ok(t) => t,
                                Err(_) => continue,
                            };
                            let sliced = apply_slice(&text, &ct.slice);
                            let trimmed = sliced.trim();
                            if trimmed.is_empty() {
                                continue;
                            }
                            let translated = forward_translate(trimmed, &ct.rules_entries, &ct.rules_default);
                            (ct.resolved.setter)(&mut rec, &translated);
                        }
                    }
                    RawField::Data { body, .. } => {
                        let subfield_values = extract_subfields(encoding, body);
                        for binding in &compiled.bindings {
                            for (code, raw_value) in &subfield_values {
                                if *code != binding.code {
                                    continue;
                                }
                                let sliced = apply_slice(raw_value, &binding.slice);
                                let trimmed = apply_trim(&sliced, &binding.trim_chars);
                                let trimmed = trimmed.trim();
                                if trimmed.is_empty() {
                                    continue;
                                }
                                let translated = forward_translate(trimmed, &binding.rules_entries, &binding.rules_default);

                                if is_agent_path(&binding.target_path) {
                                    apply_agent_forward(&mut rec, &binding.target_path, &translated, compiled.field_type.as_deref());
                                } else {
                                    (binding.resolved.setter)(&mut rec, &translated);
                                }
                            }
                        }
                        apply_defaults(&mut rec, compiled);
                    }
                }
            }
        }
        Ok(rec)
    }

    // -----------------------------------------------------------------------
    // Reverse mapping: Record -> raw
    // -----------------------------------------------------------------------

    pub fn to_raw(&self, encoding: &Encoding, record: &Record) -> Result<OwnedRawRecord, MarcError> {
        let mut fields: Vec<([u8; 3], ReverseFieldData)> = Vec::new();

        for compiled in &self.fields {
            if !compiled.reverse_enabled {
                continue;
            }

            // Control field
            if let Some(ct) = &compiled.control_target {
                let value = if let Some(getter) = &ct.resolved.option_getter {
                    getter(record)
                } else if let Some(getter) = &ct.resolved.vec_getter {
                    getter(record).into_iter().next()
                } else {
                    None
                };
                if let Some(val) = value {
                    if !val.trim().is_empty() {
                        let raw_val = reverse_translate(&val, &ct.rules_entries);
                        let output = place_in_slice(&raw_val, &ct.slice, ct.length);
                        fields.push((compiled.tag, ReverseFieldData::Control(output)));
                    }
                }
                continue;
            }

            // Data field with subfield bindings
            if compiled.bindings.is_empty() {
                continue;
            }

            // Agent fields need special handling
            if is_agent_field(compiled) {
                let agent_fields = reverse_agent(record, compiled);
                fields.extend(agent_fields);
                continue;
            }

            // Determine if this is a Vec-of-struct field by checking the first binding
            let first_binding = &compiled.bindings[0];
            let is_vec = matches!(first_binding.resolved.kind, PathKind::VecStructCreator | PathKind::VecPush);

            if is_vec {
                let count = first_binding.resolved.vec_getter.as_ref().map(|g| g(record).len()).unwrap_or(0);

                for idx in 0..count {
                    if !matches_defaults_at_index(record, compiled, idx) {
                        continue;
                    }
                    let mut subfields: Vec<(u8, String)> = Vec::new();
                    for binding in &compiled.bindings {
                        let values: Vec<String> = if let Some(g) = &binding.resolved.vec_getter { g(record) } else { Vec::new() };
                        if let Some(val) = values.get(idx) {
                            if !val.trim().is_empty() {
                                let raw_val = reverse_translate(val, &binding.rules_entries);
                                subfields.push((binding.code, raw_val));
                            }
                        }
                    }
                    if !subfields.is_empty() {
                        fields.push((
                            compiled.tag,
                            ReverseFieldData::Data {
                                ind1: compiled.indicators[0],
                                ind2: compiled.indicators[1],
                                subfields,
                            },
                        ));
                    }
                }
            } else {
                // Option or flat fields
                if !matches_defaults_option(record, compiled) {
                    continue;
                }
                let mut subfields: Vec<(u8, String)> = Vec::new();
                let mut slice_buffers: HashMap<u8, Vec<u8>> = HashMap::new();
                for binding in &compiled.bindings {
                    let val = if let Some(g) = &binding.resolved.option_getter {
                        g(record)
                    } else if let Some(g) = &binding.resolved.vec_getter {
                        let vals = g(record);
                        for v in &vals {
                            if !v.trim().is_empty() {
                                let raw_val = reverse_translate(v, &binding.rules_entries);
                                subfields.push((binding.code, raw_val));
                            }
                        }
                        continue;
                    } else {
                        None
                    };
                    if let Some(v) = val {
                        if !v.trim().is_empty() {
                            let raw_val = reverse_translate(&v, &binding.rules_entries);
                            if let Some(ref slice) = binding.slice {
                                let field_len = compiled.length.unwrap_or(slice.offset + slice.length);
                                let buf = slice_buffers.entry(binding.code).or_insert_with(|| vec![b' '; field_len]);
                                let bytes = raw_val.as_bytes();
                                let len = bytes.len().min(slice.length);
                                buf[slice.offset..slice.offset + len].copy_from_slice(&bytes[..len]);
                            } else {
                                subfields.push((binding.code, raw_val));
                            }
                        }
                    }
                }
                for (code, buf) in slice_buffers {
                    subfields.push((code, String::from_utf8_lossy(&buf).into_owned()));
                }
                if !subfields.is_empty() {
                    fields.push((
                        compiled.tag,
                        ReverseFieldData::Data {
                            ind1: compiled.indicators[0],
                            ind2: compiled.indicators[1],
                            subfields,
                        },
                    ));
                }
            }
        }

        merge_control_fields(&mut fields);

        let mut reverse_positions: Vec<(usize, usize, String)> = Vec::new();
        for lp in &self.leader_positions {
            if let Some(value) = get_leader_field(&record.leader, &lp.target) {
                let raw_val = reverse_translate(&value, &lp.rules_entries);
                reverse_positions.push((lp.position, lp.length, raw_val));
            } else if let Some(default) = &lp.default_raw {
                reverse_positions.push((lp.position, lp.length, default.clone()));
            }
        }

        let layout = LeaderLayout {
            reverse_positions: &reverse_positions,
            directory_map: &self.directory_map,
        };

        let mut raw = build_iso2709(encoding, &fields, &layout)?;
        self.apply_encoding_indicator(&mut raw, encoding)?;
        Ok(raw)
    }

    fn apply_encoding_indicator(&self, raw: &mut OwnedRawRecord, encoding: &Encoding) -> Result<(), MarcError> {
        let ei = match &self.encoding_indicator {
            Some(ei) => ei,
            None => return Ok(()),
        };

        let encoding_name = encoding_to_name(encoding);
        let raw_value = ei
            .rules
            .iter()
            .find(|r| r.encoding == encoding_name)
            .map(|r| r.raw.clone())
            .or_else(|| ei.default_raw.clone())
            .unwrap_or_default();

        if let Some(pos) = ei.leader_position {
            let data = raw.data_mut();
            let bytes = raw_value.as_bytes();
            let len = bytes.len().min(data.len().saturating_sub(pos));
            data[pos..pos + len].copy_from_slice(&bytes[..len]);
        } else if let Some(tag) = &ei.tag {
            let subfield_code = ei.subfield.unwrap_or(b'a');
            patch_field_subfield_slice(raw, tag, subfield_code, &ei.slice, raw_value.as_bytes())?;
        }

        Ok(())
    }
}

fn encoding_to_name(encoding: &Encoding) -> &'static str {
    match encoding {
        Encoding::Utf8 => "utf8",
        Encoding::Marc8 => "marc8",
        Encoding::Iso5426 => "iso5426",
        Encoding::Other(enc) => {
            if *enc == encoding_rs::ISO_8859_2 {
                "iso8859_2"
            } else if *enc == encoding_rs::ISO_8859_3 {
                "iso8859_3"
            } else if *enc == encoding_rs::ISO_8859_5 {
                "iso8859_5"
            } else {
                "unknown"
            }
        }
    }
}

fn patch_field_subfield_slice(raw: &mut OwnedRawRecord, tag: &[u8; 3], subfield_code: u8, slice: &Option<SliceDef>, patch_bytes: &[u8]) -> Result<(), MarcError> {
    let data = raw.data_mut();
    if data.len() < 24 {
        return Err(MarcError::InvalidRecord("record shorter than leader"));
    }
    let base_addr = std::str::from_utf8(&data[12..17])
        .ok()
        .and_then(|s| s.trim().parse::<usize>().ok())
        .ok_or(MarcError::InvalidRecord("invalid base address"))?;

    let mut dir_pos = 24;
    while dir_pos + 12 <= data.len() && data[dir_pos] != 0x1E {
        let entry_tag = &data[dir_pos..dir_pos + 3];
        if entry_tag == tag {
            let field_len = std::str::from_utf8(&data[dir_pos + 3..dir_pos + 7]).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);
            let field_offset = std::str::from_utf8(&data[dir_pos + 7..dir_pos + 12]).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);
            let abs_start = base_addr + field_offset;
            let abs_end = (abs_start + field_len).min(data.len());

            let body_start = abs_start + 2;
            let mut pos = body_start;
            while pos < abs_end {
                if data[pos] == 0x1F {
                    if pos + 1 >= abs_end {
                        break;
                    }
                    let code = data[pos + 1];
                    let sf_start = pos + 2;
                    let mut sf_end = sf_start;
                    while sf_end < abs_end && data[sf_end] != 0x1F && data[sf_end] != 0x1E {
                        sf_end += 1;
                    }
                    if code == subfield_code {
                        match slice {
                            Some(s) => {
                                let target = sf_start + s.offset;
                                let len = patch_bytes.len().min(s.length);
                                if target + len <= abs_end {
                                    data[target..target + len].copy_from_slice(&patch_bytes[..len]);
                                }
                            }
                            None => {
                                let len = patch_bytes.len().min(sf_end - sf_start);
                                data[sf_start..sf_start + len].copy_from_slice(&patch_bytes[..len]);
                            }
                        }
                        return Ok(());
                    }
                    pos = sf_end;
                } else if data[pos] == 0x1E {
                    break;
                } else {
                    pos += 1;
                }
            }
        }
        dir_pos += 12;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_subfields(encoding: &Encoding, body: &[u8]) -> Vec<(u8, String)> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < body.len() {
        if body[pos] == 0x1F {
            if pos + 1 >= body.len() {
                break;
            }
            let code = body[pos + 1];
            let start = pos + 2;
            let mut end = start;
            while end < body.len() && body[end] != 0x1F && body[end] != 0x1E {
                end += 1;
            }
            if let Ok(text) = encoding.decode(&body[start..end]) {
                result.push((code, text.into_owned()));
            }
            pos = end;
        } else if body[pos] == 0x1E {
            break;
        } else {
            pos += 1;
        }
    }
    result
}

fn apply_slice(text: &str, slice: &Option<SliceDef>) -> String {
    match slice {
        None => text.to_string(),
        Some(s) => {
            if s.offset + s.length <= text.len() {
                text[s.offset..s.offset + s.length].to_string()
            } else {
                String::new()
            }
        }
    }
}

fn apply_trim(text: &str, trim_chars: &Option<Vec<char>>) -> String {
    match trim_chars {
        None => text.to_string(),
        Some(chars) => {
            let pat: &[char] = chars;
            text.trim_matches(pat).trim().to_string()
        }
    }
}

fn place_in_slice(value: &str, slice: &Option<SliceDef>, total_length: Option<usize>) -> String {
    match slice {
        None => value.to_string(),
        Some(s) => {
            let total = total_length.unwrap_or(s.offset + s.length);
            let mut buf = vec![b' '; total];
            let bytes = value.as_bytes();
            let len = bytes.len().min(s.length);
            buf[s.offset..s.offset + len].copy_from_slice(&bytes[..len]);
            String::from_utf8_lossy(&buf).into_owned()
        }
    }
}

fn merge_control_fields(fields: &mut Vec<([u8; 3], ReverseFieldData)>) {
    let mut merged: HashMap<[u8; 3], Vec<u8>> = HashMap::new();
    let mut other: Vec<([u8; 3], ReverseFieldData)> = Vec::new();

    for (tag, data) in fields.drain(..) {
        match data {
            ReverseFieldData::Control(val) => {
                let bytes = val.as_bytes();
                let buf = merged.entry(tag).or_insert_with(|| vec![b' '; bytes.len()]);
                if buf.len() < bytes.len() {
                    buf.resize(bytes.len(), b' ');
                }
                for (i, &b) in bytes.iter().enumerate() {
                    if b != b' ' {
                        buf[i] = b;
                    }
                }
            }
            other_data => other.push((tag, other_data)),
        }
    }

    *fields = other;
    for (tag, buf) in merged {
        fields.push((tag, ReverseFieldData::Control(String::from_utf8_lossy(&buf).into_owned())));
    }
}

fn is_agent_path(path: &str) -> bool {
    path.starts_with("responsibility.main_entry.") || path.starts_with("responsibility.added_entries.")
}

fn is_agent_field(compiled: &CompiledField) -> bool {
    compiled.bindings.iter().any(|b| is_agent_path(&b.target_path))
}

fn apply_agent_forward(rec: &mut Record, path: &str, value: &str, agent_type: Option<&str>) {
    let is_main = path.starts_with("responsibility.main_entry.");
    let field_name = path.rsplit('.').next().unwrap_or("");

    match (is_main, field_name) {
        (true, "name") => match agent_type {
            Some("person") => {
                rec.responsibility.main_entry = Some(Agent::Person(Person {
                    name: value.to_string(),
                    forename: None,
                    dates: None,
                    numeration: None,
                    titles_associated: None,
                    fuller_form: None,
                    relator: None,
                }));
            }
            Some("corporate_body") => {
                rec.responsibility.main_entry = Some(Agent::CorporateBody(CorporateBody {
                    name: value.to_string(),
                    subordinate_unit: None,
                    location: None,
                    date: None,
                }));
            }
            Some("meeting") => {
                rec.responsibility.main_entry = Some(Agent::Meeting(Meeting {
                    name: value.to_string(),
                    location: None,
                    date: None,
                    subordinate_unit: None,
                }));
            }
            _ => {}
        },
        (true, "forename") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.forename = Some(value.to_string());
            }
        }
        (true, "dates") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.dates = Some(value.to_string());
            }
        }
        (true, "numeration") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.numeration = Some(value.to_string());
            }
        }
        (true, "titles_associated") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.titles_associated = Some(value.to_string());
            }
        }
        (true, "fuller_form") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.fuller_form = Some(value.to_string());
            }
        }
        (true, "relator") => {
            if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                p.relator = Some(Relator::from_marc_str(value));
            }
        }
        (true, "subordinate_unit") => match &mut rec.responsibility.main_entry {
            Some(Agent::CorporateBody(c)) => c.subordinate_unit = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.subordinate_unit = Some(value.to_string()),
            _ => {}
        },
        (true, "location") => match &mut rec.responsibility.main_entry {
            Some(Agent::CorporateBody(c)) => c.location = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.location = Some(value.to_string()),
            _ => {}
        },
        (true, "date") => match &mut rec.responsibility.main_entry {
            Some(Agent::CorporateBody(c)) => c.date = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.date = Some(value.to_string()),
            _ => {}
        },
        (false, "name") => match agent_type {
            Some("person") => {
                rec.responsibility.added_entries.push(Agent::Person(Person {
                    name: value.to_string(),
                    forename: None,
                    dates: None,
                    numeration: None,
                    titles_associated: None,
                    fuller_form: None,
                    relator: None,
                }));
            }
            Some("corporate_body") => {
                rec.responsibility.added_entries.push(Agent::CorporateBody(CorporateBody {
                    name: value.to_string(),
                    subordinate_unit: None,
                    location: None,
                    date: None,
                }));
            }
            Some("meeting") => {
                rec.responsibility.added_entries.push(Agent::Meeting(Meeting {
                    name: value.to_string(),
                    location: None,
                    date: None,
                    subordinate_unit: None,
                }));
            }
            _ => {}
        },
        (false, "forename") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.forename = Some(value.to_string());
            }
        }
        (false, "dates") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.dates = Some(value.to_string());
            }
        }
        (false, "numeration") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.numeration = Some(value.to_string());
            }
        }
        (false, "titles_associated") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.titles_associated = Some(value.to_string());
            }
        }
        (false, "fuller_form") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.fuller_form = Some(value.to_string());
            }
        }
        (false, "relator") => {
            if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                p.relator = Some(Relator::from_marc_str(value));
            }
        }
        (false, "subordinate_unit") => match rec.responsibility.added_entries.last_mut() {
            Some(Agent::CorporateBody(c)) => c.subordinate_unit = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.subordinate_unit = Some(value.to_string()),
            _ => {}
        },
        (false, "location") => match rec.responsibility.added_entries.last_mut() {
            Some(Agent::CorporateBody(c)) => c.location = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.location = Some(value.to_string()),
            _ => {}
        },
        (false, "date") => match rec.responsibility.added_entries.last_mut() {
            Some(Agent::CorporateBody(c)) => c.date = Some(value.to_string()),
            Some(Agent::Meeting(m)) => m.date = Some(value.to_string()),
            _ => {}
        },
        _ => {}
    }
}

fn apply_defaults(rec: &mut Record, compiled: &CompiledField) {
    if let Some(defaults) = &compiled.defaults {
        for (key, value) in defaults {
            // Apply defaults to the last created struct element
            match key.as_str() {
                "heading_type" => {
                    if let Some(last) = rec.indexing.subjects.last_mut() {
                        last.heading_type = SubjectType::from_rule_value(value);
                    }
                }
                "scheme" => {
                    if let Some(last) = rec.indexing.classifications.last_mut() {
                        last.scheme = ClassificationScheme::from_rule_value(value);
                    }
                }
                "note_type" => {
                    if let Some(last) = rec.notes.items.last_mut() {
                        last.note_type = Some(NoteType::from_rule_value(value));
                    }
                }
                "link_type" => {
                    if let Some(last) = rec.links.records.last_mut() {
                        last.link_type = Some(LinkType::from_rule_value(value));
                    }
                }
                _ => {}
            }
        }
    }
}

fn matches_defaults_at_index(record: &Record, compiled: &CompiledField, idx: usize) -> bool {
    let Some(defaults) = &compiled.defaults else {
        return true;
    };
    for (key, expected) in defaults {
        let actual = match key.as_str() {
            "heading_type" => record.indexing.subjects.get(idx).map(|s| s.heading_type.to_rule_value()),
            "scheme" => record.indexing.classifications.get(idx).map(|c| c.scheme.to_rule_value()),
            "note_type" => record.notes.items.get(idx).and_then(|n| n.note_type.as_ref().map(|t| t.to_rule_value())),
            "link_type" => record.links.records.get(idx).and_then(|l| l.link_type.as_ref().map(|t| t.to_rule_value())),
            _ => None,
        };
        if actual.as_deref() != Some(expected) {
            return false;
        }
    }
    true
}

fn matches_defaults_option(record: &Record, compiled: &CompiledField) -> bool {
    let Some(defaults) = &compiled.defaults else {
        return true;
    };
    for (key, expected) in defaults {
        let actual = match key.as_str() {
            "heading_type" => record.indexing.subjects.last().map(|s| s.heading_type.to_rule_value()),
            "scheme" => record.indexing.classifications.last().map(|c| c.scheme.to_rule_value()),
            "note_type" => record.notes.items.last().and_then(|n| n.note_type.as_ref().map(|t| t.to_rule_value())),
            "link_type" => record.links.records.last().and_then(|l| l.link_type.as_ref().map(|t| t.to_rule_value())),
            _ => None,
        };
        if actual.as_deref() != Some(expected) {
            return false;
        }
    }
    true
}

fn reverse_agent(record: &Record, compiled: &CompiledField) -> Vec<([u8; 3], ReverseFieldData)> {
    let mut result = Vec::new();
    let agent_type = compiled.field_type.as_deref();
    let is_main = compiled.bindings.iter().any(|b| b.target_path.starts_with("responsibility.main_entry."));

    if is_main {
        let agent = &record.responsibility.main_entry;
        if let Some(entry) = agent {
            if let Some(data) = agent_to_subfields(entry, agent_type, compiled) {
                result.push((compiled.tag, data));
            }
        }
    } else {
        for entry in &record.responsibility.added_entries {
            if let Some(data) = agent_to_subfields(entry, agent_type, compiled) {
                result.push((compiled.tag, data));
            }
        }
    }
    result
}

fn agent_to_subfields(agent: &Agent, expected_type: Option<&str>, compiled: &CompiledField) -> Option<ReverseFieldData> {
    if !matches!(
        (expected_type, agent),
        (Some("person"), Agent::Person(_)) | (Some("corporate_body"), Agent::CorporateBody(_)) | (Some("meeting"), Agent::Meeting(_)) | (None, _)
    ) {
        return None;
    }

    let mut subfields: Vec<(u8, String)> = Vec::new();
    for binding in &compiled.bindings {
        let field_name = binding.target_path.rsplit('.').next().unwrap_or("");
        let val = match (agent, field_name) {
            (Agent::Person(p), "name") => Some(p.name.clone()),
            (Agent::Person(p), "forename") => p.forename.clone(),
            (Agent::Person(p), "dates") => p.dates.clone(),
            (Agent::Person(p), "numeration") => p.numeration.clone(),
            (Agent::Person(p), "titles_associated") => p.titles_associated.clone(),
            (Agent::Person(p), "fuller_form") => p.fuller_form.clone(),
            (Agent::Person(p), "relator") => p.relator.as_ref().map(|r| r.to_marc_str()),
            (Agent::CorporateBody(c), "name") => Some(c.name.clone()),
            (Agent::CorporateBody(c), "subordinate_unit") => c.subordinate_unit.clone(),
            (Agent::CorporateBody(c), "location") => c.location.clone(),
            (Agent::CorporateBody(c), "date") => c.date.clone(),
            (Agent::Meeting(m), "name") => Some(m.name.clone()),
            (Agent::Meeting(m), "location") => m.location.clone(),
            (Agent::Meeting(m), "date") => m.date.clone(),
            (Agent::Meeting(m), "subordinate_unit") => m.subordinate_unit.clone(),
            _ => None,
        };
        if let Some(v) = val {
            if !v.trim().is_empty() {
                let raw_val = reverse_translate(&v, &binding.rules_entries);
                subfields.push((binding.code, raw_val));
            }
        }
    }

    if subfields.is_empty() {
        None
    } else {
        Some(ReverseFieldData::Data {
            ind1: compiled.indicators[0],
            ind2: compiled.indicators[1],
            subfields,
        })
    }
}

// ---------------------------------------------------------------------------
// Public API: load JSON config and create engine
// ---------------------------------------------------------------------------

pub fn load_config(json: &str) -> Result<(CatalogConfig, CompiledConfig), ConfigError> {
    let config: CatalogConfig = serde_json::from_str(json).map_err(ConfigError::Json)?;
    config.validate()?;
    let compiled = config.compile()?;
    Ok((config, compiled))
}

#[cfg(test)]
mod tests {
    use super::*;

    static UNIMARC_JSON: &str = include_str!("../../resources/unimarc.json");
    static MARC21_JSON: &str = include_str!("../../resources/marc21.json");

    #[test]
    fn unimarc_config_validates_and_compiles() {
        let (_, compiled) = load_config(UNIMARC_JSON).expect("unimarc config should compile");
        assert!(!compiled.fields.is_empty(), "should have compiled fields");
    }

    #[test]
    fn marc21_config_validates_and_compiles() {
        let (_, compiled) = load_config(MARC21_JSON).expect("marc21 config should compile");
        assert!(!compiled.fields.is_empty(), "should have compiled fields");
    }

    #[test]
    fn known_paths_resolve() {
        let paths = [
            // identification
            "identification.record_id",
            "identification.isbn.value",
            "identification.isbn.qualifying",
            "identification.issn.value",
            "identification.patent_numbers.number",
            "identification.patent_numbers.country",
            "identification.technical_report_numbers.value",
            "identification.publisher_numbers.value",
            "identification.publisher_numbers.source",
            "identification.codens.value",
            "identification.original_study_numbers.value",
            "identification.government_document_numbers.value",
            "identification.report_numbers.value",
            "identification.national_bibliography_numbers",
            "identification.system_control_numbers",
            // coded
            "coded.languages",
            "coded.original_languages",
            "coded.country",
            "coded.publication_dates",
            "coded.target_audience",
            "coded.geographic_area_codes",
            "coded.time_period_codes",
            "coded.date_entered_on_file",
            "coded.type_of_date",
            "coded.date1",
            "coded.date2",
            "coded.government_publication",
            "coded.modified_record",
            "coded.cataloging_language",
            "coded.transliteration_code",
            "coded.character_set",
            "coded.additional_character_set",
            "coded.script_of_title",
            "coded.place_of_publication_code",
            "coded.cataloging_source_code",
            // description
            "description.title.main",
            "description.title.subtitle",
            "description.title.parallel",
            "description.title.responsibility",
            "description.title.medium",
            "description.title.number_of_part",
            "description.title.name_of_part",
            "description.edition",
            "description.publication.place",
            "description.publication.publisher",
            "description.publication.date",
            "description.publication.manufacture_place",
            "description.publication.manufacturer",
            "description.publication.manufacture_date",
            "description.physical_description.extent",
            "description.series.title",
            "description.series.volume",
            "description.series.issn",
            "description.varying_titles.title",
            "description.varying_titles.remainder",
            "description.frequency",
            // notes
            "notes.items.text",
            "notes.items.note_type",
            // links
            "links.records.identifier",
            "links.records.link_type",
            "links.records.title",
            "links.records.edition",
            "links.records.qualifier",
            "links.records.issn",
            "links.records.volume",
            "links.records.relationship_info",
            // associated titles
            "associated_titles.uniform_title.main",
            "associated_titles.key_title.main",
            "associated_titles.former_titles.main",
            "associated_titles.variant_titles.main",
            "associated_titles.abbreviated_title",
            // indexing
            "indexing.subjects.value",
            "indexing.subjects.heading_type",
            "indexing.classifications.number",
            "indexing.classifications.scheme",
            "indexing.uncontrolled_terms",
            // international
            "international.cataloging_sources.country",
            "international.cataloging_sources.agency",
            "international.cataloging_sources.transcribing_agency",
            "international.cataloging_sources.modifying_agencies",
            "international.cataloging_sources.cataloging_language",
            "international.cataloging_sources.describing_conventions",
            "international.holding_institutions",
            "international.location_call_numbers.location",
            "international.location_call_numbers.shelving_location",
            "international.location_call_numbers.classification_part",
            "international.location_call_numbers.country_code",
            "international.electronic_locations.uri",
            // local
            "local.specimens.library",
            "local.specimens.barcode",
            // responsibility (agent paths)
            "responsibility.main_entry.name",
            "responsibility.main_entry.forename",
            "responsibility.main_entry.dates",
            "responsibility.main_entry.numeration",
            "responsibility.main_entry.titles_associated",
            "responsibility.main_entry.fuller_form",
            "responsibility.main_entry.relator",
            "responsibility.main_entry.subordinate_unit",
            "responsibility.main_entry.location",
            "responsibility.main_entry.date",
            "responsibility.added_entries.name",
            "responsibility.added_entries.forename",
            "responsibility.added_entries.dates",
            "responsibility.added_entries.numeration",
            "responsibility.added_entries.titles_associated",
            "responsibility.added_entries.fuller_form",
            "responsibility.added_entries.relator",
            "responsibility.added_entries.subordinate_unit",
            "responsibility.added_entries.location",
            "responsibility.added_entries.date",
        ];
        for p in &paths {
            assert!(resolve_path(p).is_ok(), "path '{}' should resolve", p);
        }
    }

    #[test]
    fn unknown_path_errors() {
        assert!(resolve_path("nonexistent.path").is_err());
        assert!(resolve_path("identification.unknown_field").is_err());
    }

    #[test]
    fn path_kinds_match() {
        use PathKind::*;
        let cases: Vec<(&str, PathKind)> = vec![
            ("identification.record_id", OptionSet),
            ("identification.isbn.value", VecStructCreator),
            ("identification.isbn.qualifying", VecStructField),
            ("coded.languages", VecPush),
            ("coded.country", OptionSet),
            ("description.title.main", OptionInit),
            ("description.title.parallel", VecPush),
            ("description.publication.place", VecStructCreator),
            ("description.physical_description.extent", OptionInit),
            ("notes.items.text", VecStructCreator),
            ("notes.items.note_type", VecStructField),
            ("indexing.subjects.value", VecStructCreator),
            ("local.specimens.library", VecStructCreator),
            ("international.holding_institutions", VecPush),
        ];
        for (path, expected) in &cases {
            let resolved = resolve_path(path).unwrap();
            assert_eq!(resolved.kind, *expected, "kind mismatch for '{}'", path);
        }
    }

    #[test]
    fn marc_set_and_get_option_string() {
        let mut rec = Record::default();
        rec.marc_set("identification.record_id", "REC001");
        assert_eq!(rec.marc_get_option("identification.record_id"), Some("REC001".to_string()));
    }

    #[test]
    fn marc_set_and_get_vec_string() {
        let mut rec = Record::default();
        rec.marc_set("identification.national_bibliography_numbers", "NBN1");
        rec.marc_set("identification.national_bibliography_numbers", "NBN2");
        assert_eq!(rec.marc_get_vec("identification.national_bibliography_numbers"), Some(vec!["NBN1".to_string(), "NBN2".to_string()]));
    }

    #[test]
    fn marc_set_vec_struct_creator_and_field() {
        let mut rec = Record::default();
        rec.marc_set("identification.isbn.value", "978-0-13-110362-7");
        rec.marc_set("identification.isbn.qualifying", "hardcover");
        rec.marc_set("identification.isbn.value", "978-0-13-110363-4");

        assert_eq!(rec.identification.isbn.len(), 2);
        assert_eq!(rec.identification.isbn[0].value, "978-0-13-110362-7");
        assert_eq!(rec.identification.isbn[0].qualifying, Some("hardcover".to_string()));
        assert_eq!(rec.identification.isbn[1].value, "978-0-13-110363-4");
    }

    #[test]
    fn marc_set_option_init() {
        let mut rec = Record::default();
        rec.marc_set("description.title.main", "Test Title");
        rec.marc_set("description.title.subtitle", "A Subtitle");

        let title = rec.description.title.as_ref().unwrap();
        assert_eq!(title.main, "Test Title");
        assert_eq!(title.subtitle, Some("A Subtitle".to_string()));
    }

    #[test]
    fn marc_set_vec_push_leaf_value() {
        let mut rec = Record::default();
        rec.marc_set("coded.languages", "fre");
        rec.marc_set("coded.languages", "eng");
        assert_eq!(rec.coded.languages.len(), 2);
    }

    #[test]
    fn marc_set_option_leaf_value() {
        let mut rec = Record::default();
        rec.marc_set("coded.country", "xxu");
        assert!(rec.coded.country.is_some());
    }

    #[test]
    fn marc_set_bare_leaf_value() {
        let mut rec = Record::default();
        rec.marc_set("indexing.subjects.value", "Mathematics");
        rec.marc_set("indexing.subjects.heading_type", "topical");
        assert_eq!(rec.indexing.subjects.len(), 1);
        assert_eq!(rec.indexing.subjects[0].value, "Mathematics");
    }

    #[test]
    fn marc_get_vec_through_vec_struct() {
        let mut rec = Record::default();
        rec.marc_set("notes.items.text", "Note 1");
        rec.marc_set("notes.items.note_type", "general");
        rec.marc_set("notes.items.text", "Note 2");

        let texts = rec.marc_get_vec("notes.items.text").unwrap();
        assert_eq!(texts, vec!["Note 1", "Note 2"]);

        let types = rec.marc_get_vec("notes.items.note_type").unwrap();
        assert_eq!(types.len(), 1);
    }

    #[test]
    fn marc_get_option_through_option_struct() {
        let mut rec = Record::default();
        rec.marc_set("description.physical_description.extent", "300 p.");
        assert_eq!(rec.marc_get_option("description.physical_description.extent"), Some("300 p.".to_string()));
    }

    #[test]
    fn marc_get_vec_through_option_struct() {
        let mut rec = Record::default();
        rec.marc_set("description.title.parallel", "Titre parallèle");
        rec.marc_set("description.title.parallel", "Another parallel");
        let v = rec.marc_get_vec("description.title.parallel").unwrap();
        assert_eq!(v, vec!["Titre parallèle", "Another parallel"]);
    }

    #[test]
    fn new_paths_resolve_for_extended_fields() {
        let new_paths = [
            "description.series.title",
            "description.series.volume",
            "description.varying_titles.title",
            "description.frequency",
            "links.records.title",
            "links.records.volume",
            "associated_titles.key_title.main",
            "associated_titles.former_titles.main",
            "associated_titles.variant_titles.main",
            "indexing.uncontrolled_terms",
            "coded.original_languages",
            "international.cataloging_sources.transcribing_agency",
        ];
        for p in &new_paths {
            assert!(resolve_path(p).is_ok(), "new path '{}' should resolve", p);
        }
    }

    #[test]
    fn marc_set_series_statement() {
        let mut rec = Record::default();
        rec.marc_set("description.series.title", "Collection sciences");
        rec.marc_set("description.series.volume", "vol. 3");
        rec.marc_set("description.series.issn", "1234-5678");
        assert_eq!(rec.description.series.len(), 1);
        assert_eq!(rec.description.series[0].title, "Collection sciences");
        assert_eq!(rec.description.series[0].volume, Some("vol. 3".to_string()));
        assert_eq!(rec.description.series[0].issn, Some("1234-5678".to_string()));
    }

    #[test]
    fn marc_set_varying_titles() {
        let mut rec = Record::default();
        rec.marc_set("description.varying_titles.title", "Alt Title");
        rec.marc_set("description.varying_titles.remainder", "subtitle");
        rec.marc_set("description.varying_titles.title", "Another Title");
        assert_eq!(rec.description.varying_titles.len(), 2);
        assert_eq!(rec.description.varying_titles[0].title, "Alt Title");
        assert_eq!(rec.description.varying_titles[0].remainder, Some("subtitle".to_string()));
    }

    #[test]
    fn marc_set_linked_record_with_rich_fields() {
        let mut rec = Record::default();
        rec.marc_set("links.records.identifier", "REC123");
        rec.marc_set("links.records.title", "Parent Work");
        rec.marc_set("links.records.edition", "2nd ed.");
        rec.marc_set("links.records.issn", "0000-1111");
        rec.marc_set("links.records.volume", "v. 1");
        assert_eq!(rec.links.records.len(), 1);
        assert_eq!(rec.links.records[0].identifier, "REC123");
        assert_eq!(rec.links.records[0].title, Some("Parent Work".to_string()));
        assert_eq!(rec.links.records[0].edition, Some("2nd ed.".to_string()));
        assert_eq!(rec.links.records[0].issn, Some("0000-1111".to_string()));
        assert_eq!(rec.links.records[0].volume, Some("v. 1".to_string()));
    }

    #[test]
    fn marc_set_associated_titles() {
        let mut rec = Record::default();
        rec.marc_set("associated_titles.key_title.main", "Key Title");
        rec.marc_set("associated_titles.key_title.subtitle", "Qualifier");
        assert!(rec.associated_titles.key_title.is_some());
        let kt = rec.associated_titles.key_title.as_ref().unwrap();
        assert_eq!(kt.main, "Key Title");
        assert_eq!(kt.subtitle, Some("Qualifier".to_string()));

        rec.marc_set("associated_titles.former_titles.main", "Old Title");
        rec.marc_set("associated_titles.former_titles.main", "Older Title");
        assert_eq!(rec.associated_titles.former_titles.len(), 2);

        rec.marc_set("associated_titles.variant_titles.main", "Variant");
        assert_eq!(rec.associated_titles.variant_titles.len(), 1);

        rec.marc_set("associated_titles.abbreviated_title", "Abbrev.");
        assert_eq!(rec.associated_titles.abbreviated_title, Some("Abbrev.".to_string()));
    }

    #[test]
    fn marc_set_uncontrolled_terms() {
        let mut rec = Record::default();
        rec.marc_set("indexing.uncontrolled_terms", "term1");
        rec.marc_set("indexing.uncontrolled_terms", "term2");
        assert_eq!(rec.indexing.uncontrolled_terms, vec!["term1", "term2"]);
    }

    #[test]
    fn marc_set_original_languages() {
        let mut rec = Record::default();
        rec.marc_set("coded.original_languages", "english");
        assert_eq!(rec.coded.original_languages.len(), 1);
    }

    #[test]
    fn marc_set_cataloging_source_extended() {
        let mut rec = Record::default();
        rec.marc_set("international.cataloging_sources.agency", "DLC");
        rec.marc_set("international.cataloging_sources.cataloging_language", "eng");
        rec.marc_set("international.cataloging_sources.transcribing_agency", "DLC");
        rec.marc_set("international.cataloging_sources.describing_conventions", "rda");
        assert_eq!(rec.international.cataloging_sources.len(), 1);
        assert_eq!(rec.international.cataloging_sources[0].agency, Some("DLC".to_string()));
        assert_eq!(rec.international.cataloging_sources[0].cataloging_language, Some("eng".to_string()));
    }

    #[test]
    fn marc_set_publication_manufacture() {
        let mut rec = Record::default();
        rec.marc_set("description.publication.place", "New York");
        rec.marc_set("description.publication.publisher", "Publisher");
        rec.marc_set("description.publication.date", "2024");
        rec.marc_set("description.publication.manufacture_place", "Factory City");
        rec.marc_set("description.publication.manufacturer", "Printer Inc.");
        assert_eq!(rec.description.publication.len(), 1);
        assert_eq!(rec.description.publication[0].manufacture_place, Some("Factory City".to_string()));
    }

    #[test]
    fn marc_set_title_extended_fields() {
        let mut rec = Record::default();
        rec.marc_set("description.title.main", "Title");
        rec.marc_set("description.title.medium", "[electronic resource]");
        rec.marc_set("description.title.number_of_part", "Part 1");
        rec.marc_set("description.title.name_of_part", "Introduction");
        let t = rec.description.title.as_ref().unwrap();
        assert_eq!(t.medium, Some("[electronic resource]".to_string()));
        assert_eq!(t.number_of_part, Some("Part 1".to_string()));
        assert_eq!(t.name_of_part, Some("Introduction".to_string()));
    }

    #[test]
    fn new_note_types_resolve_via_from_rule_value() {
        let note_types = ["dissertation", "systemDetails", "languageNote", "awards", "reproduction", "frequency", "credits"];
        for nt in &note_types {
            let val = NoteType::from_rule_value(nt);
            assert_ne!(val.to_rule_value(), "", "note type '{}' should round-trip", nt);
        }
    }

    #[test]
    fn new_link_types_resolve_via_from_rule_value() {
        let link_types = [
            "series",
            "supplement",
            "supplementParent",
            "issuedWith",
            "continues",
            "continuedBy",
            "supersedes",
            "supersededBy",
            "translation",
            "translationOf",
            "otherEdition",
            "otherEditionDiffLang",
            "preceding",
            "succeeding",
            "setLevel",
            "subsetLevel",
            "pieceLevel",
        ];
        for lt in &link_types {
            let val = LinkType::from_rule_value(lt);
            assert_eq!(val.to_rule_value(), *lt, "link type '{}' should round-trip", lt);
        }
    }

    #[test]
    fn new_subject_types_resolve_via_from_rule_value() {
        let subject_types = ["personal", "corporate", "meeting", "uniformTitle", "topical", "geographic", "genre", "uncontrolled"];
        for st in &subject_types {
            let val = SubjectType::from_rule_value(st);
            assert_eq!(val.to_rule_value(), *st, "subject type '{}' should round-trip", st);
        }
    }

    #[test]
    fn path_kinds_for_new_fields() {
        use PathKind::*;
        let cases: Vec<(&str, PathKind)> = vec![
            ("description.series.title", VecStructCreator),
            ("description.series.volume", VecStructField),
            ("description.series.issn", VecStructField),
            ("description.varying_titles.title", VecStructCreator),
            ("description.varying_titles.remainder", VecStructField),
            ("description.frequency", OptionSet),
            ("links.records.title", VecStructField),
            ("links.records.volume", VecStructField),
            ("associated_titles.key_title.main", OptionInit),
            ("associated_titles.former_titles.main", VecStructCreator),
            ("associated_titles.variant_titles.main", VecStructCreator),
            ("associated_titles.abbreviated_title", OptionSet),
            ("indexing.uncontrolled_terms", VecPush),
            ("coded.original_languages", VecPush),
        ];
        for (path, expected) in &cases {
            let resolved = resolve_path(path).unwrap();
            assert_eq!(resolved.kind, *expected, "kind mismatch for '{}'", path);
        }
    }
}
