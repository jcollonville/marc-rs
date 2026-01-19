use crate::encoding::convert_to_utf8;
use crate::format::{FormatEncoding, MarcFormat};
use crate::record::{ControlField, DataField, Leader, Record, Subfield};

/// Parse error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidLeader(String),
    InvalidRecordLength(String),
    InvalidField(String),
    InvalidEncoding(String),
    UnexpectedEof,
    InvalidXml(String),
    Other(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLeader(msg) => write!(f, "Invalid leader: {}", msg),
            ParseError::InvalidRecordLength(msg) => write!(f, "Invalid record length: {}", msg),
            ParseError::InvalidField(msg) => write!(f, "Invalid field: {}", msg),
            ParseError::InvalidEncoding(msg) => write!(f, "Invalid encoding: {}", msg),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::InvalidXml(msg) => write!(f, "Invalid XML: {}", msg),
            ParseError::Other(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse MARC records from bytes
pub fn parse(data: &[u8], format_encoding: FormatEncoding) -> Result<Vec<Record>, ParseError> {
    match format_encoding.format {
        MarcFormat::Marc21 => parse_marc21_binary(data, format_encoding),
        MarcFormat::Unimarc => parse_unimarc_binary(data, format_encoding),
        MarcFormat::MarcXml => parse_marc_xml(data, format_encoding),
    }
}

/// Parse MARC21 binary format
pub fn parse_marc21_binary(
    data: &[u8],
    format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    let mut records = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        if data.len() - offset < 24 {
            break; // Not enough data for a leader
        }

        let leader = Leader::from_bytes(&data[offset..offset + 24])
            .map_err(|e| ParseError::InvalidLeader(e))?;

        let record_length = leader.record_length as usize;
        if record_length == 0 || record_length > data.len() - offset {
            return Err(ParseError::InvalidRecordLength(format!(
                "Record length {} exceeds available data {}",
                record_length,
                data.len() - offset
            )));
        }

        let record_data = &data[offset..offset + record_length];
        let record = parse_single_marc21_record(record_data, &leader, format_encoding)?;
        records.push(record);

        offset += record_length;
    }

    Ok(records)
}

/// Parse a single MARC21 record
fn parse_single_marc21_record(
    data: &[u8],
    leader: &Leader,
    format_encoding: FormatEncoding,
) -> Result<Record, ParseError> {
    if data.len() < leader.base_address_of_data as usize {
        return Err(ParseError::UnexpectedEof);
    }

    let base_address = leader.base_address_of_data as usize;
    let directory = &data[24..base_address];
    let data_area = &data[base_address..];

    let mut control_fields = Vec::new();
    let mut data_fields = Vec::new();

    let mut dir_offset = 0;
    while dir_offset + 12 <= directory.len() {
        let tag_bytes = &directory[dir_offset..dir_offset + 3];
        let tag = std::str::from_utf8(tag_bytes)
            .map_err(|e| ParseError::InvalidField(format!("Invalid tag: {}", e)))?;

        let length_bytes = &directory[dir_offset + 3..dir_offset + 7];
        let length = std::str::from_utf8(length_bytes)
            .map_err(|e| ParseError::InvalidField(format!("Invalid length: {}", e)))?
            .parse::<usize>()
            .map_err(|e| ParseError::InvalidField(format!("Invalid length number: {}", e)))?;

        let start_bytes = &directory[dir_offset + 7..dir_offset + 12];
        let start = std::str::from_utf8(start_bytes)
            .map_err(|e| ParseError::InvalidField(format!("Invalid start: {}", e)))?
            .parse::<usize>()
            .map_err(|e| ParseError::InvalidField(format!("Invalid start number: {}", e)))?;

        if start + length > data_area.len() {
            return Err(ParseError::InvalidField(format!(
                "Field extends beyond data area: start={}, length={}, data_len={}",
                start,
                length,
                data_area.len()
            )));
        }

        let field_data = &data_area[start..start + length];

        if tag < "010" {
            // Control field
            let value = convert_to_utf8(field_data, format_encoding.encoding)
                .map_err(|e| ParseError::InvalidEncoding(e))?;
            control_fields.push(ControlField {
                tag: tag.to_string(),
                value,
            });
        } else {
            // Data field
            if field_data.is_empty() {
                dir_offset += 12;
                continue;
            }

            let ind1 = field_data[0] as char;
            let ind2 = field_data[1] as char;
            let subfield_data = &field_data[2..];

            let mut subfields = Vec::new();
            let mut i = 0;
            while i < subfield_data.len() {
                if subfield_data[i] == 0x1F {
                    // Subfield delimiter
                    i += 1;
                    if i >= subfield_data.len() {
                        break;
                    }
                    let code = subfield_data[i] as char;
                    i += 1;

                    let value_start = i;
                    while i < subfield_data.len() && subfield_data[i] != 0x1F && subfield_data[i] != 0x1E {
                        i += 1;
                    }

                    let value_bytes = &subfield_data[value_start..i];
                    let value = convert_to_utf8(value_bytes, format_encoding.encoding)
                        .map_err(|e| ParseError::InvalidEncoding(e))?;

                    subfields.push(Subfield {
                        code,
                        value,
                    });
                } else {
                    i += 1;
                }
            }

            data_fields.push(DataField {
                tag: tag.to_string(),
                ind1,
                ind2,
                subfields,
            });
        }

        dir_offset += 12;
    }

    Ok(Record {
        leader: leader.clone(),
        control_fields,
        data_fields,
    })
}

/// Parse UNIMARC binary format
pub fn parse_unimarc_binary(
    data: &[u8],
    format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    // UNIMARC uses the same binary structure as MARC21
    // The main differences are in field definitions and content
    parse_marc21_binary(data, format_encoding)
}

/// Parse MARC XML format
pub fn parse_marc_xml(
    data: &[u8],
    _format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(
        std::str::from_utf8(data)
            .map_err(|e| ParseError::InvalidXml(format!("Invalid UTF-8: {}", e)))?,
    );
    reader.trim_text(true);

    let mut records = Vec::new();
    let mut buf = Vec::new();
    let mut current_record: Option<Record> = None;
    let mut current_field: Option<DataField> = None;
    let mut current_subfield: Option<Subfield> = None;
    let mut current_tag = String::new();
    let mut current_value = String::new();
    let mut in_collection = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"collection" => {
                        in_collection = true;
                    }
                    b"record" => {
                        current_record = Some(Record {
                            leader: Leader {
                                record_length: 0,
                                record_status: ' ',
                                record_type: ' ',
                                bibliographic_level: ' ',
                                type_of_control: ' ',
                                character_coding_scheme: ' ',
                                indicator_count: 2,
                                subfield_code_count: 2,
                                base_address_of_data: 0,
                                encoding_level: ' ',
                                descriptive_cataloging_form: ' ',
                                multipart_resource_record_level: ' ',
                                length_of_length_of_field_portion: 4,
                                length_of_starting_character_position_portion: 5,
                                length_of_implementation_defined_portion: 0,
                                undefined: ' ',
                            },
                            control_fields: Vec::new(),
                            data_fields: Vec::new(),
                        });
                    }
                    b"leader" => {
                        current_value.clear();
                    }
                    b"controlfield" => {
                        current_tag = String::from_utf8_lossy(
                            e.attributes()
                                .find(|a| a.as_ref().unwrap().key.as_ref() == b"tag")
                                .ok_or_else(|| {
                                    ParseError::InvalidXml("Missing tag attribute".to_string())
                                })?
                                .as_ref()
                                .unwrap()
                                .value
                                .as_ref(),
                        )
                        .to_string();
                        current_value.clear();
                    }
                    b"datafield" => {
                        let tag = String::from_utf8_lossy(
                            e.attributes()
                                .find(|a| a.as_ref().unwrap().key.as_ref() == b"tag")
                                .ok_or_else(|| {
                                    ParseError::InvalidXml("Missing tag attribute".to_string())
                                })?
                                .as_ref()
                                .unwrap()
                                .value
                                .as_ref(),
                        )
                        .to_string();

                        let ind1 = e
                            .attributes()
                            .find(|a| a.as_ref().unwrap().key.as_ref() == b"ind1")
                            .map(|a| {
                                String::from_utf8_lossy(a.as_ref().unwrap().value.as_ref())
                                    .chars()
                                    .next()
                                    .unwrap_or(' ')
                            })
                            .unwrap_or(' ');

                        let ind2 = e
                            .attributes()
                            .find(|a| a.as_ref().unwrap().key.as_ref() == b"ind2")
                            .map(|a| {
                                String::from_utf8_lossy(a.as_ref().unwrap().value.as_ref())
                                    .chars()
                                    .next()
                                    .unwrap_or(' ')
                            })
                            .unwrap_or(' ');

                        current_field = Some(DataField {
                            tag,
                            ind1,
                            ind2,
                            subfields: Vec::new(),
                        });
                    }
                    b"subfield" => {
                        let code = String::from_utf8_lossy(
                            e.attributes()
                                .find(|a| a.as_ref().unwrap().key.as_ref() == b"code")
                                .ok_or_else(|| {
                                    ParseError::InvalidXml("Missing code attribute".to_string())
                                })?
                                .as_ref()
                                .unwrap()
                                .value
                                .as_ref(),
                        )
                        .chars()
                        .next()
                        .ok_or_else(|| {
                            ParseError::InvalidXml("Empty code attribute".to_string())
                        })?;
                        current_subfield = Some(Subfield {
                            code,
                            value: String::new(),
                        });
                        current_value.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                current_value = e.unescape().unwrap_or_default().to_string();
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"record" => {
                        if let Some(record) = current_record.take() {
                            records.push(record);
                        }
                    }
                    b"leader" => {
                        if let Some(ref mut record) = current_record {
                            // Parse leader from string (24 bytes)
                            if current_value.len() >= 24 {
                                let leader_bytes = current_value.as_bytes()[..24].to_vec();
                                record.leader = Leader::from_bytes(&leader_bytes)
                                    .map_err(|e| ParseError::InvalidLeader(e))?;
                            }
                        }
                    }
                    b"controlfield" => {
                        if let Some(ref mut record) = current_record {
                            record.control_fields.push(ControlField {
                                tag: current_tag.clone(),
                                value: current_value.clone(),
                            });
                        }
                        current_tag.clear();
                        current_value.clear();
                    }
                    b"datafield" => {
                        if let Some(field) = current_field.take() {
                            if let Some(ref mut record) = current_record {
                                record.data_fields.push(field);
                            }
                        }
                    }
                    b"subfield" => {
                        if let Some(subfield) = current_subfield.take() {
                            if let Some(ref mut field) = current_field {
                                field.subfields.push(Subfield {
                                    code: subfield.code,
                                    value: current_value.clone(),
                                });
                            }
                        }
                        current_value.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(ParseError::InvalidXml(format!("XML parsing error: {}", e)));
            }
            _ => {}
        }
        buf.clear();
    }

    // If we have a single record outside a collection
    if !in_collection && records.is_empty() {
        if let Some(record) = current_record {
            records.push(record);
        }
    }

    Ok(records)
}
