use crate::encoding::convert_from_encoding;
use crate::format::{FormatEncoding, MarcFormat};
use crate::record::Record;
use std::io::Write;

/// Write error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    IoError(String),
    InvalidRecord(String),
    InvalidEncoding(String),
    Other(String),
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::IoError(msg) => write!(f, "IO error: {}", msg),
            WriteError::InvalidRecord(msg) => write!(f, "Invalid record: {}", msg),
            WriteError::InvalidEncoding(msg) => write!(f, "Invalid encoding: {}", msg),
            WriteError::Other(msg) => write!(f, "Write error: {}", msg),
        }
    }
}

impl std::error::Error for WriteError {}

impl From<std::io::Error> for WriteError {
    fn from(e: std::io::Error) -> Self {
        WriteError::IoError(e.to_string())
    }
}

impl From<quick_xml::Error> for WriteError {
    fn from(e: quick_xml::Error) -> Self {
        WriteError::Other(format!("XML error: {}", e))
    }
}

/// Write MARC records to output
pub fn write(
    records: &[Record],
    format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    match format_encoding.format {
        MarcFormat::Marc21 => write_marc21_binary(records, format_encoding, output),
        MarcFormat::Unimarc => write_unimarc_binary(records, format_encoding, output),
        MarcFormat::MarcXml => write_marc_xml(records, format_encoding, output),
    }
}

/// Write a single record (convenience function)
pub fn write_one(
    record: &Record,
    format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    write(&[record.clone()], format_encoding, output)
}

/// Write MARC21 binary format
pub fn write_marc21_binary(
    records: &[Record],
    format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    for record in records {
        write_single_marc21_binary(record, format_encoding, output)?;
    }
    Ok(())
}

/// Write a single MARC21 binary record
fn write_single_marc21_binary(
    record: &Record,
    format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    // Calculate base address (24 bytes leader + directory)
    let mut directory_entries = Vec::new();
    let mut data_area = Vec::new();

    // Write control fields
    for field in &record.control_fields {
        let value_bytes = convert_from_encoding(&field.value, format_encoding.encoding)
            .map_err(|e| WriteError::InvalidEncoding(e))?;
        let start = data_area.len();
        data_area.extend_from_slice(&value_bytes);
        data_area.push(0x1E); // Field terminator

        directory_entries.push((field.tag.clone(), start, value_bytes.len() + 1));
    }

    // Write data fields
    for field in &record.data_fields {
        let mut field_data = Vec::new();
        field_data.push(field.ind1 as u8);
        field_data.push(field.ind2 as u8);

            for subfield in &field.subfields {
                field_data.push(0x1F); // Subfield delimiter
                field_data.push(subfield.code as u8);
            let value_bytes = convert_from_encoding(&subfield.value, format_encoding.encoding)
                .map_err(|e| WriteError::InvalidEncoding(e))?;
            field_data.extend_from_slice(&value_bytes);
        }

        field_data.push(0x1E); // Field terminator

        let start = data_area.len();
        data_area.extend_from_slice(&field_data);

        directory_entries.push((field.tag.clone(), start, field_data.len()));
    }

    data_area.push(0x1D); // Record terminator

    // Build directory
    let mut directory = Vec::new();
    for (tag, start, length) in &directory_entries {
        let tag_bytes = tag.as_bytes();
        if tag_bytes.len() != 3 {
            return Err(WriteError::InvalidRecord(format!(
                "Invalid tag length: {}",
                tag
            )));
        }
        directory.extend_from_slice(tag_bytes);
        directory.extend_from_slice(format!("{:04}{:05}", length, start).as_bytes());
    }

    // Calculate base address
    let base_address = 24 + directory.len();

    // Update leader
    let mut leader = record.leader.clone();
    leader.base_address_of_data = base_address as u16;
    leader.record_length = (base_address + data_area.len()) as u16;

    // Write leader
    let leader_bytes = leader.to_bytes();
    output.write_all(&leader_bytes)?;

    // Write directory
    output.write_all(&directory)?;

    // Write data area
    output.write_all(&data_area)?;

    Ok(())
}

/// Write UNIMARC binary format
pub fn write_unimarc_binary(
    records: &[Record],
    format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    // UNIMARC uses the same binary structure as MARC21
    write_marc21_binary(records, format_encoding, output)
}

/// Write MARC XML format
pub fn write_marc_xml(
    records: &[Record],
    _format_encoding: FormatEncoding,
    output: &mut dyn Write,
) -> Result<(), WriteError> {
    use quick_xml::events::{BytesEnd, BytesStart, Event};
    use quick_xml::Writer;

    let mut writer = Writer::new(output);

    // Write XML declaration
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    if records.len() > 1 {
        // Write collection wrapper
        let mut collection_start = BytesStart::new("collection");
        collection_start.push_attribute(("xmlns", "http://www.loc.gov/MARC21/slim"));
        writer.write_event(Event::Start(collection_start))?;
    }

    for record in records {
        // Write record
        let mut record_start = BytesStart::new("record");
        record_start.push_attribute(("xmlns", "http://www.loc.gov/MARC21/slim"));
        writer.write_event(Event::Start(record_start))?;

        // Write leader
        let leader_bytes = record.leader.to_bytes();
        let leader_str = std::str::from_utf8(&leader_bytes)
            .map_err(|e| WriteError::Other(format!("Invalid leader UTF-8: {}", e)))?;
        let leader_start = BytesStart::new("leader");
        writer.write_event(Event::Start(leader_start))?;
        writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(leader_str)))?;
        writer.write_event(Event::End(BytesEnd::new("leader")))?;

        // Write control fields
        for field in &record.control_fields {
            let mut field_start = BytesStart::new("controlfield");
            field_start.push_attribute(("tag", field.tag.as_str()));
            writer.write_event(Event::Start(field_start.clone()))?;
            writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(
                &field.value,
            )))?;
            writer.write_event(Event::End(BytesEnd::new("controlfield")))?;
        }

        // Write data fields
        for field in &record.data_fields {
            let mut field_start = BytesStart::new("datafield");
            field_start.push_attribute(("tag", field.tag.as_str()));
            field_start.push_attribute(("ind1", field.ind1.to_string().as_str()));
            field_start.push_attribute(("ind2", field.ind2.to_string().as_str()));
            writer.write_event(Event::Start(field_start))?;

            for subfield in &field.subfields {
                let mut subfield_start = BytesStart::new("subfield");
                subfield_start.push_attribute(("code", subfield.code.to_string().as_str()));
                writer.write_event(Event::Start(subfield_start.clone()))?;
                writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(
                    &subfield.value,
                )))?;
                writer.write_event(Event::End(BytesEnd::new("subfield")))?;
            }

            writer.write_event(Event::End(BytesEnd::new("datafield")))?;
        }

        writer.write_event(Event::End(BytesEnd::new("record")))?;
    }

    if records.len() > 1 {
        writer.write_event(Event::End(BytesEnd::new("collection")))?;
    }

    Ok(())
}
