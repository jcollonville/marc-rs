use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{get_subfield_by_names, to_control_field, to_data_field, data_field_from_desc};
use crate::datatypes::isbn::Isbn;
use crate::formats::{BlockId, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// Bloc 0 – Identification (0xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IdentificationBlock {
    /// 001 – Record identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_identifier: Option<String>,
    /// 003 – Agency identifier (control number identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_identifier: Option<String>,
    /// 005 – Date and time of record version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_version_date: Option<String>,
    /// 010/020 – ISBN(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isbn: Vec<Isbn>,
    /// 010 MARC21 / 020 UNIMARC – LCCN / French legal deposit
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lccn: Vec<String>,
    /// 011/022 – ISSN
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issn: Vec<String>,
    /// 035 – System control number (other systems)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub system_control_number: Vec<String>,
    /// 015 – National bibliography number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_bibliography_number: Vec<String>,
    /// 016 – National library record number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_library_record_number: Vec<String>,
    /// 017 – Legal deposit number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal_deposit_number: Vec<String>,
    /// Other unrecognized fields in this block
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_control: Vec<ControlField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for IdentificationBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        _ind1: char,
        _ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        let Some(desc) = descriptor.tag_descriptor(tag) else {
            self.other_data.push(raw_data_field(tag, _ind1, _ind2, subfields));
            return;
        };

        match desc.field {
            "isbn" => {
                if let Some(isbn) = Isbn::from_subfields_with_map(subfields, desc) {
                    self.isbn.push(isbn);
                }
            }
            "lccn" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.lccn.push(v);
                }
            }
            "issn" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.issn.push(v);
                }
            }
            "system_control_number" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.system_control_number.push(v);
                }
            }
            "national_bibliography_number" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.national_bibliography_number.push(v);
                }
            }
            "national_library_record_number" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.national_library_record_number.push(v);
                }
            }
            "legal_deposit_number" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.legal_deposit_number.push(v);
                }
            }
            _ => {
                self.other_data.push(raw_data_field(tag, _ind1, _ind2, subfields));
            }
        }
    }

    fn dispatch_control(&mut self, tag: &str, value: &str, _descriptor: &dyn FormatDescriptor) {
        match tag {
            "001" => self.record_identifier = Some(value.to_string()),
            "003" => self.agency_identifier = Some(value.to_string()),
            "005" => self.record_version_date = Some(value.to_string()),
            _ => self.other_control.push(ControlField {
                tag: tag.to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn collect_data_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out = Vec::new();

        // isbn
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "isbn") {
            for isbn in &self.isbn {
                out.push(isbn.to_raw_with_desc(desc));
            }
        }
        // lccn
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "lccn") {
            for v in &self.lccn {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            }
        }
        // issn
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "issn") {
            for v in &self.issn {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            }
        }
        // system_control_number – tag is the same (035) in both formats
        for v in &self.system_control_number {
            out.push(to_data_field("035", ' ', ' ', vec![('a', v.clone())]));
        }
        // national_bibliography_number
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "national_bibliography_number") {
            for v in &self.national_bibliography_number {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            }
        }
        // national_library_record_number
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "national_library_record_number") {
            for v in &self.national_library_record_number {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            }
        }
        // legal_deposit_number
        if let Some(desc) = descriptor.field_to_tag(BlockId::Identification, "legal_deposit_number") {
            for v in &self.legal_deposit_number {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            }
        }
        out.extend(self.other_data.clone());
        out
    }

    fn collect_control_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<ControlField> {
        let mut out = Vec::new();
        if let Some(ref v) = self.record_identifier {
            out.push(to_control_field("001", v));
        }
        if let Some(ref v) = self.agency_identifier {
            out.push(to_control_field("003", v));
        }
        if let Some(ref v) = self.record_version_date {
            out.push(to_control_field("005", v));
        }
        out.extend(self.other_control.clone());
        out
    }
}

fn raw_data_field(tag: &str, ind1: char, ind2: char, subfields: &[(char, String)]) -> DataField {
    DataField {
        tag: tag.to_string(),
        ind1,
        ind2,
        subfields: subfields.iter().map(|(c, v)| Subfield { code: *c, value: v.clone() }).collect(),
    }
}
