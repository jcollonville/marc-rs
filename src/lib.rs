#![forbid(unsafe_code)]

//! MARC21 / UNIMARC parsing and writing library.
//!
//! High-level model:
//! - `raw::RawRecord<'a>`: zero-copy view over a binary ISO2709 record.
//! - `record::Record`: semantic, serde-compatible model organised in UNIMARC-style blocks.
//! - `format`: dictionaries for MARC21 and UNIMARC that map raw fields to semantic fields,
//!   including positional extraction for coded fields (e.g. UNIMARC 100$a, MARC21 008).

pub mod encoding;
pub mod error;
pub mod format;
pub mod raw;
pub mod reader;
pub mod record;
pub mod xml;

pub use crate::encoding::Encoding;
pub use crate::error::MarcError;
pub use crate::format::MarcFormat;
pub use crate::raw::{BinaryReader, BinaryWriter, RawRecord, RawRecordView};
pub use crate::reader::MarcReader;
pub use crate::record::Record;
pub use crate::record::RecordValidationIssue;
pub use crate::xml::{XmlReader, XmlWriter};

/// Detected file format (binary ISO2709 vs MARC-XML).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Binary,
    Xml,
}

/// Detect whether `data` is binary ISO2709 or MARC-XML by inspecting
/// the first non-whitespace byte (after an optional UTF-8 BOM).
pub fn detect_file_format(data: &[u8]) -> FileFormat {
    let mut start = 0;
    if data.len() >= 3 && data[..3] == [0xEF, 0xBB, 0xBF] {
        start = 3;
    }
    while start < data.len() && data[start].is_ascii_whitespace() {
        start += 1;
    }
    if start < data.len() && data[start] == b'<' {
        FileFormat::Xml
    } else {
        FileFormat::Binary
    }
}

/// Parse records from any supported format (binary ISO2709 or MARC-XML).
/// The format is auto-detected from the data.
pub fn parse_records(data: &[u8]) -> Result<Vec<Record>, MarcError> {
    MarcReader::from_bytes(data.to_vec())?.into_records()
}
