use serde::{Deserialize, Serialize};

use crate::datatypes::{data_field_from_desc, find_code_for_name, get_remaining_subfields, get_subfield_by_names, known_codes_from_map, push_subfield_by_names};
use crate::formats::TagDescriptor;
use crate::record::DataField;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalDescriptionData {
    pub extent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_physical_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accompanying_material: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl PhysicalDescriptionData {
    /// Format-agnostic parse via SubfieldMapping.
    pub fn from_subfields_with_map(subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        let extent = get_subfield_by_names(subfields, desc.subfield_map, &["extent"])?;
        let known = known_codes_from_map(desc.subfield_map);
        Some(Self {
            extent,
            other_physical_details: get_subfield_by_names(subfields, desc.subfield_map, &["other_physical_details"]),
            dimensions: get_subfield_by_names(subfields, desc.subfield_map, &["dimensions"]),
            accompanying_material: get_subfield_by_names(subfields, desc.subfield_map, &["accompanying_material"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    /// Format-agnostic serialization via TagDescriptor.
    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let extent_code = find_code_for_name(desc.subfield_map, "extent").unwrap_or('a');
        let mut out = vec![(extent_code, self.extent.clone())];
        push_subfield_by_names(&mut out, desc.subfield_map, &["other_physical_details"], &self.other_physical_details);
        push_subfield_by_names(&mut out, desc.subfield_map, &["dimensions"], &self.dimensions);
        push_subfield_by_names(&mut out, desc.subfield_map, &["accompanying_material"], &self.accompanying_material);
        out.extend(self.other_subfields.clone());
        data_field_from_desc(desc, ' ', ' ', out)
    }
}
