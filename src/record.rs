use serde::{Deserialize, Serialize};

use crate::fields::{
    AddedEntry, Control, Edition, Linking, MainEntry, Note, Physical, Series, Subject, Title,
};

/// MARC record structure with typed fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub leader: Leader,
    pub control: Vec<Control>,
    pub titles: Vec<Title>,
    pub main_entries: Vec<MainEntry>,
    pub editions: Vec<Edition>,
    pub physical: Vec<Physical>,
    pub series: Vec<Series>,
    pub notes: Vec<Note>,
    pub subjects: Vec<Subject>,
    pub added_entries: Vec<AddedEntry>,
    pub linking: Vec<Linking>,
    /// Control fields (tag < 010) not mapped to a typed variant
    pub other_control: Vec<ControlField>,
    /// Data fields (tag >= 010) not mapped to a typed variant
    pub other_data: Vec<DataField>,
}

impl Record {
    pub fn new(leader: Leader) -> Self {
        Self {
            leader,
            control: Vec::new(),
            titles: Vec::new(),
            main_entries: Vec::new(),
            editions: Vec::new(),
            physical: Vec::new(),
            series: Vec::new(),
            notes: Vec::new(),
            subjects: Vec::new(),
            added_entries: Vec::new(),
            linking: Vec::new(),
            other_control: Vec::new(),
            other_data: Vec::new(),
        }
    }
}

/// MARC leader (24 bytes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leader {
    pub record_length: u16,
    pub record_status: char,
    pub record_type: char,
    pub bibliographic_level: char,
    pub type_of_control: char,
    pub character_coding_scheme: char,
    pub indicator_count: u8,
    pub subfield_code_count: u8,
    pub base_address_of_data: u16,
    pub encoding_level: char,
    pub descriptive_cataloging_form: char,
    pub multipart_resource_record_level: char,
    pub length_of_length_of_field_portion: u8,
    pub length_of_starting_character_position_portion: u8,
    pub length_of_implementation_defined_portion: u8,
    pub undefined: char,
}

impl Leader {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 24 {
            return Err(format!("Leader must be 24 bytes, got {}", data.len()));
        }

        let record_length = parse_u16(&data[0..5])?;
        let base_address = parse_u16(&data[12..17])?;

        Ok(Leader {
            record_length,
            record_status: data[5] as char,
            record_type: data[6] as char,
            bibliographic_level: data[7] as char,
            type_of_control: data[8] as char,
            character_coding_scheme: data[9] as char,
            indicator_count: data[10] - b'0',
            subfield_code_count: data[11] - b'0',
            base_address_of_data: base_address,
            encoding_level: data[17] as char,
            descriptive_cataloging_form: data[18] as char,
            multipart_resource_record_level: data[19] as char,
            length_of_length_of_field_portion: data[20] - b'0',
            length_of_starting_character_position_portion: data[21] - b'0',
            length_of_implementation_defined_portion: data[22] - b'0',
            undefined: data[23] as char,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 24];
        let record_length_str = format!("{:05}", self.record_length);
        let base_address_str = format!("{:05}", self.base_address_of_data);

        bytes[0..5].copy_from_slice(record_length_str.as_bytes());
        bytes[5] = self.record_status as u8;
        bytes[6] = self.record_type as u8;
        bytes[7] = self.bibliographic_level as u8;
        bytes[8] = self.type_of_control as u8;
        bytes[9] = self.character_coding_scheme as u8;
        bytes[10] = b'0' + self.indicator_count;
        bytes[11] = b'0' + self.subfield_code_count;
        bytes[12..17].copy_from_slice(base_address_str.as_bytes());
        bytes[17] = self.encoding_level as u8;
        bytes[18] = self.descriptive_cataloging_form as u8;
        bytes[19] = self.multipart_resource_record_level as u8;
        bytes[20] = b'0' + self.length_of_length_of_field_portion;
        bytes[21] = b'0' + self.length_of_starting_character_position_portion;
        bytes[22] = b'0' + self.length_of_implementation_defined_portion;
        bytes[23] = self.undefined as u8;

        bytes
    }
}

fn parse_u16(bytes: &[u8]) -> Result<u16, String> {
    let s = std::str::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))?;
    s.parse::<u16>()
        .map_err(|e| format!("Invalid number: {}", e))
}

/// Raw control field (001-009) — used for the "other" bucket and writing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlField {
    pub tag: String,
    pub value: String,
}

/// Raw data field (010-999) — used for the "other" bucket and writing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataField {
    pub tag: String,
    pub ind1: char,
    pub ind2: char,
    pub subfields: Vec<Subfield>,
}

/// Subfield within a data field
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subfield {
    pub code: char,
    pub value: String,
}
