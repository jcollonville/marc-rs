#[cfg(feature = "serde")]
use crate::format::FormatEncoding;
#[cfg(feature = "serde")]
use crate::parser::{parse, ParseError};
#[cfg(feature = "serde")]
use crate::record::Record;
#[cfg(feature = "serde")]
use crate::writer::{write, WriteError};
#[cfg(feature = "serde")]
use std::io::{Read, Write};

#[cfg(feature = "serde")]
/// Deserialize a single MARC record from a byte slice
pub fn from_slice(data: &[u8], format_encoding: FormatEncoding) -> Result<Record, ParseError> {
    let records = parse(data, format_encoding)?;
    records.into_iter().next().ok_or_else(|| ParseError::Other("No record found in data".to_string()))
}

#[cfg(feature = "serde")]
/// Deserialize MARC records from a byte slice
pub fn from_slice_many(data: &[u8], format_encoding: FormatEncoding) -> Result<Vec<Record>, ParseError> {
    parse(data, format_encoding)
}

#[cfg(feature = "serde")]
/// Deserialize a single MARC record from a string (for XML format)
pub fn from_str(data: &str, format_encoding: FormatEncoding) -> Result<Record, ParseError> {
    from_slice(data.as_bytes(), format_encoding)
}

#[cfg(feature = "serde")]
/// Deserialize MARC records from a string (for XML format)
pub fn from_str_many(data: &str, format_encoding: FormatEncoding) -> Result<Vec<Record>, ParseError> {
    from_slice_many(data.as_bytes(), format_encoding)
}

#[cfg(feature = "serde")]
/// Deserialize a single MARC record from a reader
pub fn from_reader<R: Read>(mut reader: R, format_encoding: FormatEncoding) -> Result<Record, ParseError> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| ParseError::Other(format!("IO error: {}", e)))?;
    from_slice(&buffer, format_encoding)
}

#[cfg(feature = "serde")]
/// Deserialize MARC records from a reader
pub fn from_reader_many<R: Read>(mut reader: R, format_encoding: FormatEncoding) -> Result<Vec<Record>, ParseError> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| ParseError::Other(format!("IO error: {}", e)))?;
    from_slice_many(&buffer, format_encoding)
}

#[cfg(feature = "serde")]
/// Serialize a single MARC record to a writer
pub fn to_writer<W: Write>(record: &Record, format_encoding: FormatEncoding, writer: &mut W) -> Result<(), WriteError> {
    write(&[record.clone()], format_encoding, writer)
}

#[cfg(feature = "serde")]
/// Serialize multiple MARC records to a writer
pub fn to_writer_many<W: Write>(records: &[Record], format_encoding: FormatEncoding, writer: &mut W) -> Result<(), WriteError> {
    write(records, format_encoding, writer)
}

#[cfg(feature = "serde")]
/// Serialize a single MARC record to a byte vector
pub fn to_vec(record: &Record, format_encoding: FormatEncoding) -> Result<Vec<u8>, WriteError> {
    let mut buffer = Vec::new();
    to_writer(record, format_encoding, &mut buffer)?;
    Ok(buffer)
}

#[cfg(feature = "serde")]
/// Serialize multiple MARC records to a byte vector
pub fn to_vec_many(records: &[Record], format_encoding: FormatEncoding) -> Result<Vec<u8>, WriteError> {
    let mut buffer = Vec::new();
    to_writer_many(records, format_encoding, &mut buffer)?;
    Ok(buffer)
}

#[cfg(feature = "serde")]
/// Serialize a single MARC record to a string (for XML format)
pub fn to_string(record: &Record, format_encoding: FormatEncoding) -> Result<String, WriteError> {
    let bytes = to_vec(record, format_encoding)?;
    String::from_utf8(bytes).map_err(|e| WriteError::Other(format!("Invalid UTF-8: {}", e)))
}

#[cfg(feature = "serde")]
/// Serialize multiple MARC records to a string (for XML format)
pub fn to_string_many(records: &[Record], format_encoding: FormatEncoding) -> Result<String, WriteError> {
    let bytes = to_vec_many(records, format_encoding)?;
    String::from_utf8(bytes).map_err(|e| WriteError::Other(format!("Invalid UTF-8: {}", e)))
}

#[cfg(feature = "serde")]
/// Convenience function to serialize a single record (alias for to_vec)
pub fn to_record(record: &Record, format_encoding: FormatEncoding) -> Result<Vec<u8>, WriteError> {
    to_vec(record, format_encoding)
}

#[cfg(feature = "serde")]
/// Convenience function to serialize multiple records (alias for to_vec_many)
pub fn to_records(records: &[Record], format_encoding: FormatEncoding) -> Result<Vec<u8>, WriteError> {
    to_vec_many(records, format_encoding)
}
