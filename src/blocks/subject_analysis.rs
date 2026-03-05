use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::classification::DeweyClassification;
use crate::datatypes::{SubjectData, data_field_from_desc, get_subfield_by_names, to_data_field};
use crate::formats::{BlockId, FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// One subject entry tagged with its MARC tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaggedSubject {
    pub tag: String,
    pub data: SubjectData,
}

/// Bloc 6 – Analyse matière (6xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SubjectAnalysisBlock {
    /// 082/083/676 – Dewey classification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dewey_classification: Vec<DeweyClassification>,
    /// 050/680 – LC classification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lc_classification: Vec<String>,
    /// 690/610 – Local / uncontrolled subject headings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_subject_heading: Vec<String>,
    /// All other subject entries (600, 606, 650, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<TaggedSubject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for SubjectAnalysisBlock {
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

        match desc.field_type {
            FieldType::DeweyClassification => {
                if let Some(d) = DeweyClassification::from_subfields_with_map(ind1, ind2, subfields, desc) {
                    self.dewey_classification.push(d);
                }
            }
            FieldType::SubjectEntry => {
                if let Some(data) = SubjectData::from_subfields_with_map(ind2, subfields, desc.subfield_map) {
                    match desc.field {
                        "lc_classification" => {
                            if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value", "term"]) {
                                self.lc_classification.push(v);
                            }
                        }
                        "local_subject_heading" | "uncontrolled_subject_terms" => {
                            self.local_subject_heading.push(data.term);
                        }
                        _ => {
                            self.subjects.push(TaggedSubject { tag: tag.to_string(), data });
                        }
                    }
                } else {
                    self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
                }
            }
            FieldType::SimpleString => {
                match desc.field {
                    "lc_classification" => {
                        if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                            self.lc_classification.push(v);
                        }
                    }
                    "local_subject_heading" | "uncontrolled_subject_terms" => {
                        if let Some(v) = get_subfield_by_names(subfields, desc.subfield_map, &["value"]) {
                            self.local_subject_heading.push(v);
                        }
                    }
                    _ => self.other_data.push(raw_data_field(tag, ind1, ind2, subfields)),
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

        for d in &self.dewey_classification {
            if let Some(desc) = descriptor.field_to_tag(BlockId::SubjectAnalysis, "dewey_classification") {
                out.push(d.to_raw_with_desc(desc));
            }
        }
        for lc in &self.lc_classification {
            if let Some(desc) = descriptor.field_to_tag(BlockId::SubjectAnalysis, "lc_classification") {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', lc.clone())]));
            } else {
                // Fallback: use a hardcoded common tag
                out.push(to_data_field("050", ' ', ' ', vec![('a', lc.clone())]));
            }
        }
        for term in &self.local_subject_heading {
            if let Some(desc) = descriptor.field_to_tag(BlockId::SubjectAnalysis, "local_subject_heading")
                .or_else(|| descriptor.field_to_tag(BlockId::SubjectAnalysis, "uncontrolled_subject_terms")) {
                out.push(data_field_from_desc(desc, ' ', ' ', vec![('a', term.clone())]));
            } else {
                out.push(to_data_field("653", ' ', ' ', vec![('a', term.clone())]));
            }
        }
        for s in &self.subjects {
            let ind2 = s.data.thesaurus.to_ind2();
            out.push(to_data_field(&s.tag, ' ', ind2, s.data.to_subfields()));
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
