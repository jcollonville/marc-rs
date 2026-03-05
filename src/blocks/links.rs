use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{LinkingData, to_data_field};
use crate::formats::{FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// One linking entry tagged with its MARC tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedLink {
    pub tag: String,
    pub data: LinkingData,
}

/// Bloc 4 – Liens (4xx/76x-78x).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LinksBlock {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<TaggedLink>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for LinksBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        if let Some(desc) = descriptor.tag_descriptor(tag) {
            if let FieldType::LinkingEntry = desc.field_type {
                let data = LinkingData::from_subfields_with_map(ind1, subfields, desc.subfield_map);
                self.links.push(TaggedLink { tag: tag.to_string(), data });
                return;
            }
        }
        self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
    }

    fn dispatch_control(&mut self, _tag: &str, _value: &str, _descriptor: &dyn FormatDescriptor) {}

    fn collect_data_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out: Vec<DataField> = self.links
            .iter()
            .map(|l| {
                let ind1 = l.data.display_note_ind1();
                to_data_field(&l.tag, ind1, ' ', l.data.to_subfields())
            })
            .collect();
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
