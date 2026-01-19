/// MARC record structure
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub leader: Leader,
    pub control_fields: Vec<ControlField>,
    pub data_fields: Vec<DataField>,
}

/// MARC leader (24 bytes)
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Create a new Leader from a 24-byte string
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

    /// Convert Leader to 24-byte string
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
    s.parse::<u16>().map_err(|e| format!("Invalid number: {}", e))
}

/// Control field (001-009)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlField {
    pub tag: String,
    pub value: String,
}

/// Data field (010-999)
#[derive(Debug, Clone, PartialEq)]
pub struct DataField {
    pub tag: String,
    pub ind1: char,
    pub ind2: char,
    pub subfields: Vec<Subfield>,
}

/// Subfield within a data field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subfield {
    pub code: char,
    pub value: String,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Record {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Record", 3)?;
        state.serialize_field("leader", &self.leader)?;
        state.serialize_field("control_fields", &self.control_fields)?;
        state.serialize_field("data_fields", &self.data_fields)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Record {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct RecordVisitor;

        impl<'de> Visitor<'de> for RecordVisitor {
            type Value = Record;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Record")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Record, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut leader = None;
                let mut control_fields = None;
                let mut data_fields = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "leader" => {
                            if leader.is_some() {
                                return Err(de::Error::duplicate_field("leader"));
                            }
                            leader = Some(map.next_value()?);
                        }
                        "control_fields" => {
                            if control_fields.is_some() {
                                return Err(de::Error::duplicate_field("control_fields"));
                            }
                            control_fields = Some(map.next_value()?);
                        }
                        "data_fields" => {
                            if data_fields.is_some() {
                                return Err(de::Error::duplicate_field("data_fields"));
                            }
                            data_fields = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(Record {
                    leader: leader.ok_or_else(|| de::Error::missing_field("leader"))?,
                    control_fields: control_fields.ok_or_else(|| de::Error::missing_field("control_fields"))?,
                    data_fields: data_fields.ok_or_else(|| de::Error::missing_field("data_fields"))?,
                })
            }
        }

        deserializer.deserialize_map(RecordVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Leader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Leader", 16)?;
        state.serialize_field("record_length", &self.record_length)?;
        state.serialize_field("record_status", &self.record_status)?;
        state.serialize_field("record_type", &self.record_type)?;
        state.serialize_field("bibliographic_level", &self.bibliographic_level)?;
        state.serialize_field("type_of_control", &self.type_of_control)?;
        state.serialize_field("character_coding_scheme", &self.character_coding_scheme)?;
        state.serialize_field("indicator_count", &self.indicator_count)?;
        state.serialize_field("subfield_code_count", &self.subfield_code_count)?;
        state.serialize_field("base_address_of_data", &self.base_address_of_data)?;
        state.serialize_field("encoding_level", &self.encoding_level)?;
        state.serialize_field("descriptive_cataloging_form", &self.descriptive_cataloging_form)?;
        state.serialize_field("multipart_resource_record_level", &self.multipart_resource_record_level)?;
        state.serialize_field("length_of_length_of_field_portion", &self.length_of_length_of_field_portion)?;
        state.serialize_field("length_of_starting_character_position_portion", &self.length_of_starting_character_position_portion)?;
        state.serialize_field("length_of_implementation_defined_portion", &self.length_of_implementation_defined_portion)?;
        state.serialize_field("undefined", &self.undefined)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Leader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct LeaderVisitor;

        impl<'de> Visitor<'de> for LeaderVisitor {
            type Value = Leader;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Leader")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Leader, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut record_length = None;
                let mut record_status = None;
                let mut record_type = None;
                let mut bibliographic_level = None;
                let mut type_of_control = None;
                let mut character_coding_scheme = None;
                let mut indicator_count = None;
                let mut subfield_code_count = None;
                let mut base_address_of_data = None;
                let mut encoding_level = None;
                let mut descriptive_cataloging_form = None;
                let mut multipart_resource_record_level = None;
                let mut length_of_length_of_field_portion = None;
                let mut length_of_starting_character_position_portion = None;
                let mut length_of_implementation_defined_portion = None;
                let mut undefined = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "record_length" => record_length = Some(map.next_value()?),
                        "record_status" => record_status = Some(map.next_value()?),
                        "record_type" => record_type = Some(map.next_value()?),
                        "bibliographic_level" => bibliographic_level = Some(map.next_value()?),
                        "type_of_control" => type_of_control = Some(map.next_value()?),
                        "character_coding_scheme" => character_coding_scheme = Some(map.next_value()?),
                        "indicator_count" => indicator_count = Some(map.next_value()?),
                        "subfield_code_count" => subfield_code_count = Some(map.next_value()?),
                        "base_address_of_data" => base_address_of_data = Some(map.next_value()?),
                        "encoding_level" => encoding_level = Some(map.next_value()?),
                        "descriptive_cataloging_form" => descriptive_cataloging_form = Some(map.next_value()?),
                        "multipart_resource_record_level" => multipart_resource_record_level = Some(map.next_value()?),
                        "length_of_length_of_field_portion" => length_of_length_of_field_portion = Some(map.next_value()?),
                        "length_of_starting_character_position_portion" => length_of_starting_character_position_portion = Some(map.next_value()?),
                        "length_of_implementation_defined_portion" => length_of_implementation_defined_portion = Some(map.next_value()?),
                        "undefined" => undefined = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(Leader {
                    record_length: record_length.ok_or_else(|| de::Error::missing_field("record_length"))?,
                    record_status: record_status.ok_or_else(|| de::Error::missing_field("record_status"))?,
                    record_type: record_type.ok_or_else(|| de::Error::missing_field("record_type"))?,
                    bibliographic_level: bibliographic_level.ok_or_else(|| de::Error::missing_field("bibliographic_level"))?,
                    type_of_control: type_of_control.ok_or_else(|| de::Error::missing_field("type_of_control"))?,
                    character_coding_scheme: character_coding_scheme.ok_or_else(|| de::Error::missing_field("character_coding_scheme"))?,
                    indicator_count: indicator_count.ok_or_else(|| de::Error::missing_field("indicator_count"))?,
                    subfield_code_count: subfield_code_count.ok_or_else(|| de::Error::missing_field("subfield_code_count"))?,
                    base_address_of_data: base_address_of_data.ok_or_else(|| de::Error::missing_field("base_address_of_data"))?,
                    encoding_level: encoding_level.ok_or_else(|| de::Error::missing_field("encoding_level"))?,
                    descriptive_cataloging_form: descriptive_cataloging_form.ok_or_else(|| de::Error::missing_field("descriptive_cataloging_form"))?,
                    multipart_resource_record_level: multipart_resource_record_level.ok_or_else(|| de::Error::missing_field("multipart_resource_record_level"))?,
                    length_of_length_of_field_portion: length_of_length_of_field_portion.ok_or_else(|| de::Error::missing_field("length_of_length_of_field_portion"))?,
                    length_of_starting_character_position_portion: length_of_starting_character_position_portion
                        .ok_or_else(|| de::Error::missing_field("length_of_starting_character_position_portion"))?,
                    length_of_implementation_defined_portion: length_of_implementation_defined_portion.ok_or_else(|| de::Error::missing_field("length_of_implementation_defined_portion"))?,
                    undefined: undefined.ok_or_else(|| de::Error::missing_field("undefined"))?,
                })
            }
        }

        deserializer.deserialize_map(LeaderVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ControlField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ControlField", 2)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ControlField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct ControlFieldVisitor;

        impl<'de> Visitor<'de> for ControlFieldVisitor {
            type Value = ControlField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ControlField")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ControlField, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut tag = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "tag" => {
                            if tag.is_some() {
                                return Err(de::Error::duplicate_field("tag"));
                            }
                            tag = Some(map.next_value()?);
                        }
                        "value" => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(ControlField {
                    tag: tag.ok_or_else(|| de::Error::missing_field("tag"))?,
                    value: value.ok_or_else(|| de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_map(ControlFieldVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DataField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DataField", 4)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("ind1", &self.ind1)?;
        state.serialize_field("ind2", &self.ind2)?;
        state.serialize_field("subfields", &self.subfields)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DataField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct DataFieldVisitor;

        impl<'de> Visitor<'de> for DataFieldVisitor {
            type Value = DataField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DataField")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DataField, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut tag = None;
                let mut ind1 = None;
                let mut ind2 = None;
                let mut subfields = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "tag" => {
                            if tag.is_some() {
                                return Err(de::Error::duplicate_field("tag"));
                            }
                            tag = Some(map.next_value()?);
                        }
                        "ind1" => {
                            if ind1.is_some() {
                                return Err(de::Error::duplicate_field("ind1"));
                            }
                            ind1 = Some(map.next_value()?);
                        }
                        "ind2" => {
                            if ind2.is_some() {
                                return Err(de::Error::duplicate_field("ind2"));
                            }
                            ind2 = Some(map.next_value()?);
                        }
                        "subfields" => {
                            if subfields.is_some() {
                                return Err(de::Error::duplicate_field("subfields"));
                            }
                            subfields = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(DataField {
                    tag: tag.ok_or_else(|| de::Error::missing_field("tag"))?,
                    ind1: ind1.ok_or_else(|| de::Error::missing_field("ind1"))?,
                    ind2: ind2.ok_or_else(|| de::Error::missing_field("ind2"))?,
                    subfields: subfields.ok_or_else(|| de::Error::missing_field("subfields"))?,
                })
            }
        }

        deserializer.deserialize_map(DataFieldVisitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Subfield {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Subfield", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Subfield {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct SubfieldVisitor;

        impl<'de> Visitor<'de> for SubfieldVisitor {
            type Value = Subfield;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Subfield")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Subfield, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "code" => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        "value" => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(Subfield {
                    code: code.ok_or_else(|| de::Error::missing_field("code"))?,
                    value: value.ok_or_else(|| de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_map(SubfieldVisitor)
    }
}
