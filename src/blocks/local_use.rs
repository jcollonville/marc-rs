use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::formats::{BlockId, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// Bloc 9 – Usage local (9xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LocalUseBlock {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<DataField>,
}


impl MarcBlock for LocalUseBlock {
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        _descriptor: &dyn FormatDescriptor,
    ) {
        self.data.push(DataField {
            tag: tag.to_string(),
            ind1,
            ind2,
            subfields: subfields
                .iter()
                .map(|(c, v)| Subfield { code: *c, value: v.clone() })
                .collect(),
        });
    }

    fn dispatch_control(&mut self, _tag: &str, _value: &str, _descriptor: &dyn FormatDescriptor) {}

    fn collect_data_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<DataField> {
        self.data.clone()
    }

    fn collect_control_fields(&self, _descriptor: &dyn FormatDescriptor) -> Vec<ControlField> {
        Vec::new()
    }
}
