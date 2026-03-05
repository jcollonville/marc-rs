use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{data_field_from_desc, nonfiling_chars_to_ind};
use crate::datatypes::edition::{Edition, PublicationData};
use crate::datatypes::physical::PhysicalDescriptionData;
use crate::datatypes::series::SeriesStatementData;
use crate::datatypes::title::Title;
use crate::datatypes::UniformTitleData;
use crate::formats::{BlockId, FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// Bloc 2 – Description (2xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DescriptionBlock {
    /// 245/200 – Title statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_statement: Option<Title>,
    /// 250/205 – Edition statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_statement: Option<Edition>,
    /// 260/264/210 – Publication / distribution / imprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_distribution_imprint: Option<PublicationData>,
    /// 300/215 – Physical description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_description: Option<PhysicalDescriptionData>,
    /// 490/225 – Series statement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series_statement: Vec<SeriesStatementData>,
    /// 240/500 – Uniform title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_title: Option<UniformTitleData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for DescriptionBlock {
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
            "title_statement" | "title_statement_obsolete" => {
                self.title_statement = Title::from_desc(ind1, ind2, subfields, desc);
            }
            "edition_statement" => {
                if let Some(e) = Edition::from_desc(ind1, ind2, subfields, desc) {
                    self.edition_statement = Some(e);
                }
            }
            "publication_distribution_imprint" | "publication_distribution_imprint_rda" => {
                self.publication_distribution_imprint =
                    Some(PublicationData::from_subfields_with_map(ind2, subfields, desc));
            }
            "physical_description" => {
                self.physical_description = PhysicalDescriptionData::from_subfields_with_map(subfields, desc);
            }
            "series_statement" => {
                if let Some(s) = SeriesStatementData::from_subfields_with_map(ind1, subfields, desc) {
                    self.series_statement.push(s);
                }
            }
            "uniform_title" | "uniform_title_isbd" => {
                self.uniform_title = UniformTitleData::from_subfields_with_map(ind1, subfields, desc.subfield_map);
            }
            // Route remaining description-block fields via FieldType
            _ => match desc.field_type {
                FieldType::EditionStatement | FieldType::Publication => {
                    if let Some(e) = Edition::from_desc(ind1, ind2, subfields, desc) {
                        self.edition_statement = Some(e);
                    }
                }
                _ => {
                    self.other_data.push(raw_data_field(tag, ind1, ind2, subfields));
                }
            },
        }
    }

    fn dispatch_control(&mut self, _tag: &str, _value: &str, _descriptor: &dyn FormatDescriptor) {}

    fn collect_data_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        let mut out = Vec::new();

        if let Some(ref t) = self.title_statement {
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, "title_statement") {
                out.push(t.to_raw_with_desc(desc));
            }
        }
        if let Some(ref e) = self.edition_statement {
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, "edition_statement") {
                if let Some(df) = e.to_raw_with_desc(desc) {
                    out.push(df);
                }
            }
        }
        if let Some(ref p) = self.publication_distribution_imprint {
            // Try RDA tag first (264), fall back to non-RDA (260/210)
            let field_name = if p.is_rda {
                "publication_distribution_imprint_rda"
            } else {
                "publication_distribution_imprint"
            };
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, field_name) {
                out.push(p.to_raw_with_desc(desc));
            } else if let Some(desc) = descriptor.field_to_tag(BlockId::Description, "publication_distribution_imprint") {
                out.push(p.to_raw_with_desc(desc));
            }
        }
        if let Some(ref ph) = self.physical_description {
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, "physical_description") {
                out.push(ph.to_raw_with_desc(desc));
            }
        }
        for s in &self.series_statement {
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, "series_statement") {
                out.push(s.to_raw_with_desc(desc));
            }
        }
        if let Some(ref u) = self.uniform_title {
            let field_name = if descriptor.field_to_tag(BlockId::Description, "uniform_title_isbd").is_some() {
                "uniform_title_isbd"
            } else {
                "uniform_title"
            };
            if let Some(desc) = descriptor.field_to_tag(BlockId::Description, field_name) {
                out.push(data_field_from_desc(
                    desc,
                    nonfiling_chars_to_ind(u.nonfiling_chars),
                    ' ',
                    u.to_subfields_with_map(desc.subfield_map),
                ));
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
