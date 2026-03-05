use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{UniformTitleData, data_field_from_desc, get_subfield, get_subfield_by_names, nonfiling_chars_to_ind, to_data_field};
use crate::formats::{BlockId, FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// One associated title entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedTitle {
    pub tag: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// Bloc 5 – Titres associés (5xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AssociatedTitlesBlock {
    /// Uniform title (500/240)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_title: Option<UniformTitleData>,
    /// Collective uniform title (501)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collective_uniform_title: Option<UniformTitleData>,
    /// Other associated titles (cover, abbreviated, varying form, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub titles: Vec<TaggedTitle>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for AssociatedTitlesBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        let Some(desc) = descriptor.tag_descriptor(tag) else {
            // Unknown tag — try to extract a title string for simple storage
            if let Some(title) = get_subfield(subfields, 'a') {
                self.titles.push(TaggedTitle { tag: tag.to_string(), title, qualifier: get_subfield(subfields, 'b') });
            } else {
                self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
            }
            return;
        };

        match desc.field_type {
            FieldType::UniformTitle => {
                let ut = UniformTitleData::from_subfields_with_map(ind1, subfields, desc.subfield_map);
                match desc.field {
                    "uniform_title" | "uniform_title_added_entry" => self.uniform_title = ut,
                    "collective_uniform_title" => self.collective_uniform_title = ut,
                    _ => {
                        if let Some(ut) = ut {
                            self.titles.push(TaggedTitle {
                                tag: tag.to_string(),
                                title: ut.title,
                                qualifier: None,
                            });
                        }
                    }
                }
            }
            FieldType::TitleStatement | FieldType::SimpleString | FieldType::GenericDataField => {
                let title = get_subfield_by_names(subfields, desc.subfield_map, &["title", "value"]);
                if let Some(title) = title {
                    self.titles.push(TaggedTitle {
                        tag: tag.to_string(),
                        title,
                        qualifier: get_subfield_by_names(subfields, desc.subfield_map, &["qualifier", "remainder"]),
                    });
                } else {
                    self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
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

        if let Some(ref u) = self.uniform_title {
            if let Some(desc) = descriptor.field_to_tag(BlockId::AssociatedTitles, "uniform_title") {
                out.push(data_field_from_desc(desc, nonfiling_chars_to_ind(u.nonfiling_chars), ' ', u.to_subfields_with_map(desc.subfield_map)));
            } else {
                out.push(to_data_field("500", nonfiling_chars_to_ind(u.nonfiling_chars), ' ', u.to_subfields()));
            }
        }
        if let Some(ref u) = self.collective_uniform_title {
            if let Some(desc) = descriptor.field_to_tag(BlockId::AssociatedTitles, "collective_uniform_title") {
                out.push(data_field_from_desc(desc, nonfiling_chars_to_ind(u.nonfiling_chars), ' ', u.to_subfields_with_map(desc.subfield_map)));
            } else {
                out.push(to_data_field("501", nonfiling_chars_to_ind(u.nonfiling_chars), ' ', u.to_subfields()));
            }
        }
        for t in &self.titles {
            let mut sfs = vec![('a', t.title.clone())];
            if let Some(ref q) = t.qualifier { sfs.push(('b', q.clone())); }
            out.push(to_data_field(&t.tag, ' ', ' ', sfs));
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
