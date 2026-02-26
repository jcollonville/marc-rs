//! Specimen (item/copy) fields — 995 (French norm) and 952 (Koha).
//! Each physical copy is one repetition of the field; the parser collects all 995/952 into `record.specimens`.

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// One specimen (item/copy) — one 995 or 952 field.
/// Common subfields: $a library, $b section, $f barcode, $k call number, $v document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specimen {
    pub tag: String,
    pub ind1: char,
    pub ind2: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_library: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_control_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circulation_status: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl Specimen {
    const KNOWN_CODES: [char; 17] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 't', 'v', 'w',
    ];

    /// Parse one 995 or 952 field into a Specimen. Every occurrence is one specimen.
    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        _format: MarcFormat,
    ) -> Option<Self> {
        match tag {
            "995" | "952" => Some(Self {
                tag: tag.to_string(),
                ind1,
                ind2,
                library: get_subfield(subfields, 'a'),
                section: get_subfield(subfields, 'b'),
                sub_library: get_subfield(subfields, 'c'),
                section_code: get_subfield(subfields, 'd'),
                level_code: get_subfield(subfields, 'e'),
                barcode: get_subfield(subfields, 'f'),
                call_number: get_subfield(subfields, 'k'),
                inventory_number: get_subfield(subfields, 'l'),
                creation_date: get_subfield(subfields, 'm'),
                modification_date: get_subfield(subfields, 'n'),
                loan_date: get_subfield(subfields, 'o'),
                return_date: get_subfield(subfields, 'p'),
                acquisition_date: get_subfield(subfields, 'q'),
                item_type: get_subfield(subfields, 'r'),
                record_control_number: get_subfield(subfields, 't'),
                document_type: get_subfield(subfields, 'v'),
                circulation_status: get_subfield(subfields, 'w'),
                other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
            }),
            _ => None,
        }
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = Vec::new();
        push_subfield(&mut out, 'a', &self.library);
        push_subfield(&mut out, 'b', &self.section);
        push_subfield(&mut out, 'c', &self.sub_library);
        push_subfield(&mut out, 'd', &self.section_code);
        push_subfield(&mut out, 'e', &self.level_code);
        push_subfield(&mut out, 'f', &self.barcode);
        push_subfield(&mut out, 'k', &self.call_number);
        push_subfield(&mut out, 'l', &self.inventory_number);
        push_subfield(&mut out, 'm', &self.creation_date);
        push_subfield(&mut out, 'n', &self.modification_date);
        push_subfield(&mut out, 'o', &self.loan_date);
        push_subfield(&mut out, 'p', &self.return_date);
        push_subfield(&mut out, 'q', &self.acquisition_date);
        push_subfield(&mut out, 'r', &self.item_type);
        push_subfield(&mut out, 't', &self.record_control_number);
        push_subfield(&mut out, 'v', &self.document_type);
        push_subfield(&mut out, 'w', &self.circulation_status);
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, _format: MarcFormat) -> DataField {
        to_data_field(&self.tag, self.ind1, self.ind2, self.to_subfields())
    }
}
