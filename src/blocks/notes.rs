use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{NoteData, to_data_field};
use crate::formats::{FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// One note entry tagged with its MARC tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedNote {
    pub tag: String,
    pub data: NoteData,
}

/// Bloc 3 – Notes (3xx/5xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NotesBlock {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<TaggedNote>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for NotesBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        if let Some(desc) = descriptor.tag_descriptor(tag) {
            if let FieldType::NoteEntry = desc.field_type {
                if let Some(data) = NoteData::from_subfields_with_map(subfields, desc.subfield_map) {
                    self.notes.push(TaggedNote { tag: tag.to_string(), data });
                    return;
                }
            }
        }
        self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
    }

    fn dispatch_control(&mut self, _tag: &str, _value: &str, _descriptor: &dyn FormatDescriptor) {}

    fn collect_data_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out: Vec<DataField> = self.notes
            .iter()
            .map(|n| to_data_field(&n.tag, ' ', ' ', n.data.to_subfields()))
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
