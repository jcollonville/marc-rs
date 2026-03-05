use crate::formats::{SubfieldMapping, TagDescriptor};
use crate::record::{ControlField, DataField, Subfield};

// ── Type modules ──────────────────────────────────────────────────────────

pub mod classification;
pub mod corporate_name;
pub mod edition;
pub mod isbn;
pub mod language;
pub mod linking;
pub mod note;
pub mod personal_name;
pub mod physical;
pub mod series;
pub mod specimen;
pub mod subject;
pub mod title;
pub mod uniform_title;

pub use classification::*;
pub use corporate_name::*;
pub use edition::*;
pub use isbn::*;
pub use language::*;
pub use linking::*;
pub use note::*;
pub use personal_name::*;
pub use physical::*;
pub use series::*;
pub use specimen::*;
pub use subject::*;
pub use title::*;
pub use uniform_title::*;

// ── Helper functions ──────────────────────────────────────────────────────

/// Returns the first occurrence of subfield `code` as owned String.
pub fn get_subfield(subfields: &[(char, String)], code: char) -> Option<String> {
    subfields
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, v)| v.clone())
}

/// Returns subfields not consumed by named fields.
/// Skips the first occurrence of each known code, keeps duplicates and unknown codes.
pub fn get_remaining_subfields(subfields: &[(char, String)], known: &[char]) -> Vec<(char, String)> {
    let mut seen: Vec<bool> = known.iter().map(|_| false).collect();
    let mut result = Vec::new();

    for (code, value) in subfields {
        if let Some(pos) = known.iter().position(|k| k == code) {
            if !seen[pos] {
                seen[pos] = true;
            } else {
                result.push((*code, value.clone()));
            }
        } else {
            result.push((*code, value.clone()));
        }
    }

    result
}

/// Default indicator value used when none is provided.
pub fn default_indicator() -> char {
    ' '
}

/// Helper for Serde: indicators that are the default (blank) are not serialized.
pub fn is_default_indicator(c: &char) -> bool {
    *c == ' ' || *c == '\0'
}

pub fn default_true() -> bool {
    true
}

pub fn is_true(b: &bool) -> bool {
    *b
}

pub fn is_zero(n: &u8) -> bool {
    *n == 0
}

pub fn nonfiling_chars_to_ind(n: u8) -> char {
    char::from_digit(n as u32, 10).unwrap_or('0')
}

pub fn ind_to_nonfiling_chars(c: char) -> u8 {
    c.to_digit(10).map(|d| d as u8).unwrap_or(0)
}

pub fn push_subfield(out: &mut Vec<(char, String)>, code: char, value: &Option<String>) {
    if let Some(ref v) = value {
        out.push((code, v.clone()));
    }
}

// ── SubfieldMapping helpers ───────────────────────────────────────────────

/// Returns the subfield code associated with a semantic `name` in the mapping.
pub fn find_code_for_name(map: &[SubfieldMapping], name: &str) -> Option<char> {
    map.iter().find(|m| m.name == name).map(|m| m.code)
}

/// Returns the value of the subfield identified by semantic `name` via the mapping.
pub fn get_subfield_by_name(
    subfields: &[(char, String)],
    map: &[SubfieldMapping],
    name: &str,
) -> Option<String> {
    let code = find_code_for_name(map, name)?;
    get_subfield(subfields, code)
}

/// Returns the value of the first matching alias name in the mapping.
pub fn get_subfield_by_names(
    subfields: &[(char, String)],
    map: &[SubfieldMapping],
    names: &[&str],
) -> Option<String> {
    for name in names {
        if let Some(v) = get_subfield_by_name(subfields, map, name) {
            return Some(v);
        }
    }
    None
}

/// Push a subfield using the first matching alias in the mapping (no-op if no alias found).
pub fn push_subfield_by_names(
    out: &mut Vec<(char, String)>,
    map: &[SubfieldMapping],
    names: &[&str],
    value: &Option<String>,
) {
    if let Some(v) = value {
        for name in names {
            if let Some(code) = find_code_for_name(map, name) {
                out.push((code, v.clone()));
                return;
            }
        }
    }
}

/// Return all codes present in the mapping (used to compute remaining subfields).
pub fn known_codes_from_map(map: &[SubfieldMapping]) -> Vec<char> {
    map.iter().map(|m| m.code).collect()
}

pub fn subfields_to_raw(subfields: &[(char, String)]) -> Vec<Subfield> {
    subfields
        .iter()
        .map(|(c, v)| Subfield {
            code: *c,
            value: v.clone(),
        })
        .collect()
}

pub fn to_control_field(tag: &str, value: &str) -> ControlField {
    ControlField {
        tag: tag.to_string(),
        value: value.to_string(),
    }
}

/// Build a raw DataField from a TagDescriptor, indicators, and pre-computed subfields.
pub fn data_field_from_desc(
    desc: &TagDescriptor,
    ind1: char,
    ind2: char,
    subfields: Vec<(char, String)>,
) -> DataField {
    to_data_field(desc.tag, ind1, ind2, subfields)
}

pub fn to_data_field(
    tag: &str,
    ind1: char,
    ind2: char,
    subfields: Vec<(char, String)>,
) -> DataField {
    DataField {
        tag: tag.to_string(),
        ind1,
        ind2,
        subfields: subfields_to_raw(&subfields),
    }
}
