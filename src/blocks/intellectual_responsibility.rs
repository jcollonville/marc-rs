use serde::{Deserialize, Serialize};

use crate::blocks::MarcBlock;
use crate::datatypes::{
    CorporateNameData, MeetingNameData, NoteData, PersonalNameData, UniformTitleData,
    data_field_from_desc, nonfiling_chars_to_ind, to_data_field,
};
use crate::formats::{BlockId, FieldType, FormatDescriptor};
use crate::record::{ControlField, DataField, Subfield};

/// One responsibility entry tagged with its MARC tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsibilityEntry {
    PersonalName { tag: String, data: PersonalNameData },
    CorporateName { tag: String, data: CorporateNameData },
    MeetingName { tag: String, data: MeetingNameData },
    UniformTitle { tag: String, data: UniformTitleData },
    UncontrolledName { tag: String, data: NoteData },
}

/// Bloc 7 – Responsabilité intellectuelle (7xx).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IntellectualResponsibilityBlock {
    /// 100/700 – Main entry personal name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry_personal_name: Option<PersonalNameData>,
    /// 110/710 – Main entry corporate name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry_corporate_name: Option<CorporateNameData>,
    /// 111/711 – Main entry meeting name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry_meeting_name: Option<MeetingNameData>,
    /// 130/730 – Main entry uniform title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry_uniform_title: Option<UniformTitleData>,
    /// 700/701/702/800 – Added entry personal names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entry_personal_names: Vec<PersonalNameData>,
    /// 710/712/810 – Added entry corporate names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entry_corporate_names: Vec<CorporateNameData>,
    /// 711/811 – Added entry meeting names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entry_meeting_names: Vec<MeetingNameData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_entries: Vec<ResponsibilityEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

impl MarcBlock for IntellectualResponsibilityBlock {
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
            FieldType::PersonalName => {
                if let Some(d) = PersonalNameData::from_subfields_with_map(ind1, subfields, desc.subfield_map, descriptor) {
                    match desc.field {
                        "main_entry_personal_name" if self.main_entry_personal_name.is_none() => {
                            self.main_entry_personal_name = Some(d);
                        }
                        _ => self.added_entry_personal_names.push(d),
                    }
                }
            }
            FieldType::CorporateName => {
                if let Some(d) = CorporateNameData::from_subfields_with_map(ind1, subfields, desc.subfield_map, descriptor) {
                    match desc.field {
                        "main_entry_corporate_name" if self.main_entry_corporate_name.is_none() => {
                            self.main_entry_corporate_name = Some(d);
                        }
                        _ => self.added_entry_corporate_names.push(d),
                    }
                }
            }
            FieldType::MeetingName => {
                if let Some(d) = MeetingNameData::from_subfields_with_map(ind1, subfields, desc.subfield_map, descriptor) {
                    match desc.field {
                        "main_entry_meeting_name" if self.main_entry_meeting_name.is_none() => {
                            self.main_entry_meeting_name = Some(d);
                        }
                        _ => self.added_entry_meeting_names.push(d),
                    }
                }
            }
            FieldType::UniformTitle => {
                if let Some(d) = UniformTitleData::from_subfields_with_map(ind1, subfields, desc.subfield_map) {
                    self.main_entry_uniform_title = Some(d);
                }
            }
            FieldType::NoteEntry | FieldType::GenericDataField => {
                if let Some(data) = NoteData::from_subfields_with_map(subfields, desc.subfield_map) {
                    self.other_entries.push(ResponsibilityEntry::UncontrolledName {
                        tag: tag.to_string(),
                        data,
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

        if let Some(ref d) = self.main_entry_personal_name {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "main_entry_personal_name") {
                out.push(data_field_from_desc(desc, descriptor.personal_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        for d in &self.added_entry_personal_names {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "added_entry_personal_name") {
                out.push(data_field_from_desc(desc, descriptor.personal_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        if let Some(ref d) = self.main_entry_corporate_name {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "main_entry_corporate_name") {
                out.push(data_field_from_desc(desc, descriptor.organization_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        for d in &self.added_entry_corporate_names {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "added_entry_corporate_name") {
                out.push(data_field_from_desc(desc, descriptor.organization_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        if let Some(ref d) = self.main_entry_meeting_name {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "main_entry_meeting_name") {
                out.push(data_field_from_desc(desc, descriptor.organization_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        for d in &self.added_entry_meeting_names {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "added_entry_meeting_name") {
                out.push(data_field_from_desc(desc, descriptor.organization_name_type_to_ind1(&d.name_type), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        if let Some(ref d) = self.main_entry_uniform_title {
            if let Some(desc) = descriptor.field_to_tag(BlockId::IntellectualResponsibility, "main_entry_uniform_title") {
                out.push(data_field_from_desc(desc, nonfiling_chars_to_ind(d.nonfiling_chars), ' ', d.to_subfields_with_map(desc.subfield_map)));
            }
        }
        for entry in &self.other_entries {
            match entry {
                ResponsibilityEntry::UncontrolledName { tag, data } => {
                    out.push(to_data_field(tag, ' ', ' ', data.to_subfields()));
                }
                ResponsibilityEntry::PersonalName { tag, data } => {
                    out.push(to_data_field(tag, descriptor.personal_name_type_to_ind1(&data.name_type), ' ', data.to_subfields()));
                }
                ResponsibilityEntry::CorporateName { tag, data } => {
                    out.push(to_data_field(tag, descriptor.organization_name_type_to_ind1(&data.name_type), ' ', data.to_subfields()));
                }
                ResponsibilityEntry::MeetingName { tag, data } => {
                    out.push(to_data_field(tag, descriptor.organization_name_type_to_ind1(&data.name_type), ' ', data.to_subfields()));
                }
                ResponsibilityEntry::UniformTitle { tag, data } => {
                    out.push(to_data_field(
                        tag,
                        nonfiling_chars_to_ind(data.nonfiling_chars),
                        ' ',
                        data.to_subfields(),
                    ));
                }
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
