use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{data_field_from_desc, get_subfield, get_subfield_by_names, to_data_field};
use crate::formats::{BlockId, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// Bloc 8 – Usage international (8xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InternationalUseBlock {
    /// 801/040 – Cataloging source / agency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cataloging_source: Option<String>,
    /// 850 – Holding institution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding_institution: Vec<String>,
    /// 852 – Location / call number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_call_number: Option<String>,
    /// 856 – Electronic location and access (URI)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub electronic_location_access: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for InternationalUseBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        let Some(desc) = descriptor.tag_descriptor(tag) else {
            self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
            return;
        };

        match desc.field {
            "cataloging_source" => {
                self.cataloging_source = get_subfield_by_names(subfields, desc.subfield_map, &["transcribing_agency", "value"])
                    .or_else(|| get_subfield(subfields, 'b'))
                    .or_else(|| get_subfield(subfields, 'a'));
            }
            "holding_institution" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.holding_institution.push(v);
                }
            }
            "location_call_number" => {
                self.location_call_number = get_subfield_by_names(subfields, desc.subfield_map, &["call_number", "value"])
                    .or_else(|| get_subfield(subfields, 'j'))
                    .or_else(|| get_subfield(subfields, 'a'));
            }
            "electronic_location_access" => {
                if let Some(uri) = get_subfield_by_names(subfields, desc.subfield_map, &["uri", "value"]) {
                    self.electronic_location_access.push(uri);
                }
            }
            _ => {
                self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
            }
        }
    }

    fn dispatch_control(&mut self, _tag: &str, _value: &str, _descriptor: &dyn FormatDescriptor) {}

    fn collect_data_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out = Vec::new();

        if let Some(ref agency) = self.cataloging_source {
            if let Some(desc) = descriptor.field_to_tag(BlockId::InternationalUse, "cataloging_source") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('b', agency.clone())]));
            } else {
                out.push(to_data_field("040", ' ', ' ', vec![('b', agency.clone())]));
            }
        }
        for v in &self.holding_institution {
            if let Some(desc) = descriptor.field_to_tag(BlockId::InternationalUse, "holding_institution") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', v.clone())]));
            } else {
                out.push(to_data_field("850", ' ', ' ', vec![('a', v.clone())]));
            }
        }
        if let Some(ref call) = self.location_call_number {
            if let Some(desc) = descriptor.field_to_tag(BlockId::InternationalUse, "location_call_number") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('j', call.clone())]));
            } else {
                out.push(to_data_field("852", ' ', ' ', vec![('j', call.clone())]));
            }
        }
        for uri in &self.electronic_location_access {
            if let Some(desc) = descriptor.field_to_tag(BlockId::InternationalUse, "electronic_location_access") {
                out.push(data_field_from_desc(desc, '4', '0', vec![('u', uri.clone())]));
            } else {
                out.push(to_data_field("856", '4', '0', vec![('u', uri.clone())]));
            }
        }
        out.extend(self.other_data.clone());
        out
    }

    fn collect_control_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<ControlField> {
        Vec::new()
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
