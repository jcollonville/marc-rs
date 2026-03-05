use serde::{Deserialize, Serialize};

use crate::datatypes::{
    data_field_from_desc, get_remaining_subfields, get_subfield_by_names, known_codes_from_map,
    push_subfield_by_names,
};
use crate::formats::TagDescriptor;
use crate::record::DataField;

/// One physical item/copy — one 995 (UNIMARC) or 952 (Koha/MARC21) field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specimen {
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
    pub fn new() -> Self {
        Self {
            library: None,
            section: None,
            sub_library: None,
            section_code: None,
            level_code: None,
            barcode: None,
            call_number: None,
            inventory_number: None,
            creation_date: None,
            modification_date: None,
            loan_date: None,
            return_date: None,
            acquisition_date: None,
            item_type: None,
            record_control_number: None,
            document_type: None,
            circulation_status: None,
            other_subfields: Vec::new(),
        }
    }

    /// Format-agnostic parse via TagDescriptor subfield_map.
    pub fn from_subfields_with_desc(desc: &TagDescriptor, subfields: &[(char, String)]) -> Self {
        let map = desc.subfield_map;
        let known = known_codes_from_map(map);
        Self {
            library: get_subfield_by_names(subfields, map, &["library"]),
            section: get_subfield_by_names(subfields, map, &["section"]),
            sub_library: get_subfield_by_names(subfields, map, &["sub_library"]),
            section_code: get_subfield_by_names(subfields, map, &["section_code"]),
            level_code: get_subfield_by_names(subfields, map, &["level_code"]),
            barcode: get_subfield_by_names(subfields, map, &["barcode"]),
            call_number: get_subfield_by_names(subfields, map, &["call_number"]),
            inventory_number: get_subfield_by_names(subfields, map, &["inventory_number"]),
            creation_date: get_subfield_by_names(subfields, map, &["creation_date"]),
            modification_date: get_subfield_by_names(subfields, map, &["modification_date"]),
            loan_date: get_subfield_by_names(subfields, map, &["loan_date"]),
            return_date: get_subfield_by_names(subfields, map, &["return_date"]),
            acquisition_date: get_subfield_by_names(subfields, map, &["acquisition_date"]),
            item_type: get_subfield_by_names(subfields, map, &["item_type"]),
            record_control_number: get_subfield_by_names(subfields, map, &["record_control_number"]),
            document_type: get_subfield_by_names(subfields, map, &["document_type"]),
            circulation_status: get_subfield_by_names(subfields, map, &["circulation_status"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        }
    }

    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        data_field_from_desc(desc, ' ', ' ', self.to_subfields(desc))
    }

    pub fn to_subfields(&self, desc: &TagDescriptor) -> Vec<(char, String)> {
        let map = desc.subfield_map;
        let mut out = Vec::new();
        push_subfield_by_names(&mut out, map, &["library"], &self.library);
        push_subfield_by_names(&mut out, map, &["section"], &self.section);
        push_subfield_by_names(&mut out, map, &["sub_library"], &self.sub_library);
        push_subfield_by_names(&mut out, map, &["section_code"], &self.section_code);
        push_subfield_by_names(&mut out, map, &["level_code"], &self.level_code);
        push_subfield_by_names(&mut out, map, &["barcode"], &self.barcode);
        push_subfield_by_names(&mut out, map, &["call_number"], &self.call_number);
        push_subfield_by_names(&mut out, map, &["inventory_number"], &self.inventory_number);
        push_subfield_by_names(&mut out, map, &["creation_date"], &self.creation_date);
        push_subfield_by_names(&mut out, map, &["modification_date"], &self.modification_date);
        push_subfield_by_names(&mut out, map, &["loan_date"], &self.loan_date);
        push_subfield_by_names(&mut out, map, &["return_date"], &self.return_date);
        push_subfield_by_names(&mut out, map, &["acquisition_date"], &self.acquisition_date);
        push_subfield_by_names(&mut out, map, &["item_type"], &self.item_type);
        push_subfield_by_names(
            &mut out,
            map,
            &["record_control_number"],
            &self.record_control_number,
        );
        push_subfield_by_names(&mut out, map, &["document_type"], &self.document_type);
        push_subfield_by_names(
            &mut out,
            map,
            &["circulation_status"],
            &self.circulation_status,
        );
        out.extend(self.other_subfields.clone());
        out
    }
}

impl Default for Specimen {
    fn default() -> Self {
        Self::new()
    }
}
