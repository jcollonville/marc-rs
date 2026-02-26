use crate::encoding::{convert_to_utf8, convert_to_utf8_heuristic};
use crate::fields::{
    AddedEntry, Control, Edition, Linking, MainEntry, Note, Physical, Series, Specimen, Subject,
    Title,
};
use crate::format::{Encoding, FormatEncoding, MarcFormat};
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
            break;
        }

        let leader =
            Leader::from_bytes(&data[offset..offset + 24]).map_err(ParseError::InvalidLeader)?;

        let record_length = leader.record_length as usize;
        if record_length == 0 || record_length > data.len() - offset {
            return Err(ParseError::InvalidRecordLength(format!(
                "Record length {} exceeds available data {}",
                record_length,
                data.len() - offset
            )));
        }

        let record_data = &data[offset..offset + record_length];
        let record = parse_single_record(record_data, &leader, format_encoding)?;
        records.push(record);

        offset += record_length;
    }

    Ok(records)
}

/// Detect per-record encoding.
/// Rule: Leader pos 9 = 'a' => UTF-8 (absolute priority).
/// Otherwise, for UNIMARC, field 100 $a positions 26-29 is authoritative.
/// For MARC21, Leader is the reference; field 100 used as fallback.
fn detect_record_encoding(
    record_data: &[u8],
    leader: &Leader,
    _format: MarcFormat,
) -> Option<Encoding> {
    // A. Leader position 9: 'a' (0x61) = UTF-8 (structural, always wins)
    if record_data.len() > 9 && record_data[9] == 0x61 {
        return Some(Encoding::Utf8);
    }

    // B. Field 100 $a, positions 26-29 (authoritative for UNIMARC, fallback for MARC21)
    let base_address = leader.base_address_of_data as usize;
    if record_data.len() < base_address + 12 {
        return None;
    }
    let directory = &record_data[24..base_address];
    let data_area = &record_data[base_address..];

    let mut dir_offset = 0;
    while dir_offset + 12 <= directory.len() {
        let tag = std::str::from_utf8(&directory[dir_offset..dir_offset + 3]).unwrap_or("");
        if tag != "100" {
            dir_offset += 12;
            continue;
        }
        let length = std::str::from_utf8(&directory[dir_offset + 3..dir_offset + 7])
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        let start = std::str::from_utf8(&directory[dir_offset + 7..dir_offset + 12])
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        if start + length > data_area.len() {
            return None;
        }
        let field_data = &data_area[start..start + length];
        if field_data.len() < 2 {
            return None;
        }
        let subfield_data = &field_data[2..];
        let mut i = 0;
        while i + 2 <= subfield_data.len() {
            if subfield_data[i] == 0x1F && subfield_data[i + 1] == 0x61 {
                let value_start = i + 2;
                let mut value_end = value_start;
                while value_end < subfield_data.len()
                    && subfield_data[value_end] != 0x1F
                    && subfield_data[value_end] != 0x1E
                {
                    value_end += 1;
                }
                let value = &subfield_data[value_start..value_end];
                return parse_unimarc_100a_encoding(value);
            }
            i += 1;
        }
        return None;
    }
    None
}

fn parse_unimarc_100a_encoding(value: &[u8]) -> Option<Encoding> {
    if value.len() < 28 {
        return None;
    }

    match &value[26..28] {
        b"50" => Some(Encoding::Utf8),
        b"01" => Some(Encoding::Iso5426),
        b"02" => Some(Encoding::Iso6937),
        b"03" => Some(Encoding::Iso5427),
        b"05" => Some(Encoding::Iso5428),
        _ => None,
    }
}

/// Decode field bytes using per-record encoding or heuristic fallback.
fn decode_field_bytes(bytes: &[u8], encoding: Option<Encoding>) -> Result<String, ParseError> {
    match encoding {
        Some(enc) => convert_to_utf8(bytes, enc).map_err(ParseError::InvalidEncoding),
        None => convert_to_utf8_heuristic(bytes).map_err(ParseError::InvalidEncoding),
    }
}

/// Dispatch a control field (tag < "010") into typed Record fields.
fn dispatch_control_field(
    tag: &str,
    value: &str,
    format: MarcFormat,
    record: &mut Record,
) {
    if let Some(ctrl) = Control::try_parse(tag, value, format) {
        record.control.push(ctrl);
    } else {
        record.other_control.push(ControlField {
            tag: tag.to_string(),
            value: value.to_string(),
        });
    }
}

/// Dispatch a data field (tag >= "010") into typed Record fields.
fn dispatch_data_field(
    tag: &str,
    ind1: char,
    ind2: char,
    subfields: &[(char, String)],
    format: MarcFormat,
    record: &mut Record,
) {
    // Try each field module in priority order
    if let Some(t) = Title::try_parse(tag, ind1, ind2, subfields, format) {
        record.titles.push(t);
        return;
    }
    if let Some(me) = MainEntry::try_parse(tag, ind1, ind2, subfields, format) {
        record.main_entries.push(me);
        return;
    }
    if let Some(ed) = Edition::try_parse(tag, ind1, ind2, subfields, format) {
        record.editions.push(ed);
        return;
    }
    if let Some(ph) = Physical::try_parse(tag, ind1, ind2, subfields, format) {
        record.physical.push(ph);
        return;
    }
    if let Some(se) = Series::try_parse(tag, ind1, ind2, subfields, format) {
        record.series.push(se);
        return;
    }
    if let Some(no) = Note::try_parse(tag, ind1, ind2, subfields, format) {
        record.notes.push(no);
        return;
    }
    if let Some(su) = Subject::try_parse(tag, ind1, ind2, subfields, format) {
        record.subjects.push(su);
        return;
    }
    if let Some(ae) = AddedEntry::try_parse(tag, ind1, ind2, subfields, format) {
        record.added_entries.push(ae);
        return;
    }
    if let Some(li) = Linking::try_parse(tag, ind1, ind2, subfields, format) {
        record.linking.push(li);
        return;
    }
    if let Some(sp) = Specimen::try_parse(tag, ind1, ind2, subfields, format) {
        record.specimens.push(sp);
        return;
    }

    // Unrecognized tag => other_data
    record.other_data.push(DataField {
        tag: tag.to_string(),
        ind1,
        ind2,
        subfields: subfields
            .iter()
            .map(|(c, v)| Subfield {
                code: *c,
                value: v.clone(),
            })
            .collect(),
    });
}

/// Parse raw subfield bytes into (code, value) tuples.
fn parse_subfield_bytes(
    subfield_data: &[u8],
    encoding: Option<Encoding>,
) -> Result<Vec<(char, String)>, ParseError> {
    let mut subfields = Vec::new();
    let mut i = 0;
    while i < subfield_data.len() {
        if subfield_data[i] == 0x1F {
            i += 1;
            if i >= subfield_data.len() {
                break;
            }
            let code = subfield_data[i] as char;
            i += 1;
            let value_start = i;
            while i < subfield_data.len() && subfield_data[i] != 0x1F && subfield_data[i] != 0x1E
            {
                i += 1;
            }
            let value = decode_field_bytes(&subfield_data[value_start..i], encoding)?;
            subfields.push((code, value));
        } else {
            i += 1;
        }
    }
    Ok(subfields)
}

/// Parse a single binary record (works for both MARC21 and UNIMARC).
fn parse_single_record(
    data: &[u8],
    leader: &Leader,
    format_encoding: FormatEncoding,
) -> Result<Record, ParseError> {
    if data.len() < leader.base_address_of_data as usize {
        return Err(ParseError::UnexpectedEof);
    }

    let format = format_encoding.format;
    let record_encoding = detect_record_encoding(data, leader, format);

    let base_address = leader.base_address_of_data as usize;
    let directory = &data[24..base_address];
    let data_area = &data[base_address..];

    let mut record = Record::new(leader.clone());

    let mut dir_offset = 0;
    while dir_offset + 12 <= directory.len() {
        let tag = std::str::from_utf8(&directory[dir_offset..dir_offset + 3])
            .map_err(|e| ParseError::InvalidField(format!("Invalid tag: {}", e)))?;

        let length = std::str::from_utf8(&directory[dir_offset + 3..dir_offset + 7])
            .map_err(|e| ParseError::InvalidField(format!("Invalid length: {}", e)))?
            .parse::<usize>()
            .map_err(|e| ParseError::InvalidField(format!("Invalid length number: {}", e)))?;

        let start = std::str::from_utf8(&directory[dir_offset + 7..dir_offset + 12])
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
            let value = decode_field_bytes(field_data, record_encoding)?;
            dispatch_control_field(tag, &value, format, &mut record);
        } else {
            if field_data.is_empty() {
                dir_offset += 12;
                continue;
            }

            let ind1 = field_data[0] as char;
            let ind2 = field_data[1] as char;
            let subfields = parse_subfield_bytes(&field_data[2..], record_encoding)?;
            dispatch_data_field(tag, ind1, ind2, &subfields, format, &mut record);
        }

        dir_offset += 12;
    }

    Ok(record)
}

/// Parse UNIMARC binary format
pub fn parse_unimarc_binary(
    data: &[u8],
    format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    parse_marc21_binary(data, format_encoding)
}

/// Parse MARC XML format
pub fn parse_marc_xml(
    data: &[u8],
    format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let format = format_encoding.format;

    let mut reader = Reader::from_str(
        std::str::from_utf8(data)
            .map_err(|e| ParseError::InvalidXml(format!("Invalid UTF-8: {}", e)))?,
    );
    reader.trim_text(true);

    let mut records = Vec::new();
    let mut buf = Vec::new();

    let mut current_record: Option<Record> = None;
    // Accumulator for the current datafield being built
    let mut current_df_tag = String::new();
    let mut current_df_ind1 = ' ';
    let mut current_df_ind2 = ' ';
    let mut current_df_subfields: Vec<(char, String)> = Vec::new();
    let mut in_datafield = false;

    let mut current_sf_code: Option<char> = None;
    let mut current_tag = String::new();
    let mut current_value = String::new();
    let mut in_collection = false;

    let default_leader = Leader {
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
    };

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"collection" => {
                    in_collection = true;
                }
                b"record" => {
                    current_record = Some(Record::new(default_leader.clone()));
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
                    current_df_tag = String::from_utf8_lossy(
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

                    current_df_ind1 = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key.as_ref() == b"ind1")
                        .map(|a| {
                            String::from_utf8_lossy(a.as_ref().unwrap().value.as_ref())
                                .chars()
                                .next()
                                .unwrap_or(' ')
                        })
                        .unwrap_or(' ');

                    current_df_ind2 = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key.as_ref() == b"ind2")
                        .map(|a| {
                            String::from_utf8_lossy(a.as_ref().unwrap().value.as_ref())
                                .chars()
                                .next()
                                .unwrap_or(' ')
                        })
                        .unwrap_or(' ');

                    current_df_subfields.clear();
                    in_datafield = true;
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
                    current_sf_code = Some(code);
                    current_value.clear();
                }
                _ => {}
            },
            Ok(Event::Text(e)) => {
                current_value = e.unescape().unwrap_or_default().to_string();
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"record" => {
                    if let Some(record) = current_record.take() {
                        records.push(record);
                    }
                }
                b"leader" => {
                    if let Some(ref mut record) = current_record {
                        if current_value.len() >= 24 {
                            let leader_bytes = current_value.as_bytes()[..24].to_vec();
                            record.leader = Leader::from_bytes(&leader_bytes)
                                .map_err(ParseError::InvalidLeader)?;
                        }
                    }
                }
                b"controlfield" => {
                    if let Some(ref mut record) = current_record {
                        dispatch_control_field(
                            &current_tag,
                            &current_value,
                            format,
                            record,
                        );
                    }
                    current_tag.clear();
                    current_value.clear();
                }
                b"datafield" => {
                    if in_datafield {
                        if let Some(ref mut record) = current_record {
                            dispatch_data_field(
                                &current_df_tag,
                                current_df_ind1,
                                current_df_ind2,
                                &current_df_subfields,
                                format,
                                record,
                            );
                        }
                        in_datafield = false;
                    }
                }
                b"subfield" => {
                    if let Some(code) = current_sf_code.take() {
                        current_df_subfields.push((code, current_value.clone()));
                    }
                    current_value.clear();
                }
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(ParseError::InvalidXml(format!("XML parsing error: {}", e)));
            }
            _ => {}
        }
        buf.clear();
    }

    if !in_collection && records.is_empty() {
        if let Some(record) = current_record {
            records.push(record);
        }
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unimarc_100a_with_code(code: &[u8; 2]) -> Vec<u8> {
        let mut value = vec![b'0'; 30];
        value[26] = code[0];
        value[27] = code[1];
        value
    }

    #[test]
    fn parse_unimarc_100a_encoding_supports_all_requested_codes() {
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"01")),
            Some(Encoding::Iso5426)
        );
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"02")),
            Some(Encoding::Iso6937)
        );
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"03")),
            Some(Encoding::Iso5427)
        );
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"05")),
            Some(Encoding::Iso5428)
        );
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"50")),
            Some(Encoding::Utf8)
        );
    }

    #[test]
    fn parse_unimarc_100a_encoding_rejects_unknown_code() {
        assert_eq!(
            parse_unimarc_100a_encoding(&unimarc_100a_with_code(b"99")),
            None
        );
    }
}
