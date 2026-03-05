use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{data_field_from_desc, get_subfield_by_names, to_control_field};
use crate::datatypes::language::LanguageData;
use crate::formats::{BlockId, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// Bloc 1 – Informations codées (1xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CodedInformationBlock {
    /// 100/100 – General processing data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_processing_data: Option<String>,
    /// 101/041 – Language(s) of resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_of_resource: Option<LanguageData>,
    /// 102/044 – Country of publication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_of_publication: Vec<String>,
    /// 006 – Additional fixed-length data elements (MARC21 control)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fixed_length_elements: Option<String>,
    /// 007 – Physical description fixed field (MARC21 control)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_description_fixed_field: Option<String>,
    /// 008 – Fixed-length data elements (MARC21 control)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_length_data_elements: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_control: Vec<ControlField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for CodedInformationBlock {
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
            "language_of_resource" => {
                self.language_of_resource = LanguageData::from_subfields_with_map(ind1, subfields, desc);
            }
            "country_of_publication" => {
                if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                    self.country_of_publication.push(v);
                }
            }
            "general_processing_data" => {
                self.general_processing_data = get_subfield_by_names(subfields, desc.subfield_map, &["value"]);
            }
            _ => {
                self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
            }
        }
    }

    fn dispatch_control(&mut self, tag: &str, value: &str, _descriptor: &dyn FormatDescriptor) {
        match tag {
            "006" => self.additional_fixed_length_elements = Some(value.to_string()),
            "007" => self.physical_description_fixed_field = Some(value.to_string()),
            "008" => self.fixed_length_data_elements = Some(value.to_string()),
            _ => self.other_control.push(ControlField {
                tag: tag.to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn collect_data_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out = Vec::new();

        if let Some(ref lang) = self.language_of_resource {
            if let Some(desc) = descriptor.field_to_tag(BlockId::CodedInformation, "language_of_resource") {
                out.push(lang.to_raw_with_desc(desc));
            }
        }
        if let Some(ref gp) = self.general_processing_data {
            if let Some(desc) = descriptor.field_to_tag(BlockId::CodedInformation, "general_processing_data") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', gp.clone())]));
            }
        }
        for country in &self.country_of_publication {
            if let Some(desc) = descriptor.field_to_tag(BlockId::CodedInformation, "country_of_publication") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', country.clone())]));
            }
        }
        out.extend(self.other_data.clone());
        out
    }

    fn collect_control_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<ControlField> {
        let mut out = Vec::new();
        if let Some(ref v) = self.additional_fixed_length_elements {
            out.push(to_control_field("006", v));
        }
        if let Some(ref v) = self.physical_description_fixed_field {
            out.push(to_control_field("007", v));
        }
        if let Some(ref v) = self.fixed_length_data_elements {
            out.push(to_control_field("008", v));
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
