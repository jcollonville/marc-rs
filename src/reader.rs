use std::path::Path;

use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::format::MarcFormat;
use crate::raw::{BinaryReader, RawRecordView};
use crate::record::Record;
use crate::xml::XmlReader;
use crate::{detect_file_format, FileFormat};

/// Unified MARC reader with auto-detection of file format (binary ISO2709 or MARC-XML).
///
/// Internally, all data is stored as ISO2709 bytes. XML input is eagerly
/// converted by `XmlReader::parse()`, so iteration always goes through
/// `BinaryReader`.
pub struct MarcReader {
    data: Vec<u8>,
    source_format: FileFormat,
    /// When set, used instead of the encoding detected from the record (e.g. leader / 100$a).
    encoding_override: Option<Encoding>,
}

impl MarcReader {
    /// Auto-detect format from raw bytes and build the reader.
    /// XML data is eagerly converted to ISO2709.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, MarcError> {
        let fmt = detect_file_format(&data);
        match fmt {
            FileFormat::Binary => Ok(Self {
                data,
                source_format: FileFormat::Binary,
                encoding_override: None,
            }),
            FileFormat::Xml => Ok(Self {
                data: XmlReader::parse(&data)?,
                source_format: FileFormat::Xml,
                encoding_override: None,
            }),
        }
    }

    /// Read a file, auto-detect its format, and build the reader.
    pub fn from_file(path: &Path) -> Result<Self, MarcError> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data)
    }

    /// Force binary ISO2709 format (no detection).
    pub fn from_binary(data: Vec<u8>) -> Self {
        Self {
            data,
            source_format: FileFormat::Binary,
            encoding_override: None,
        }
    }

    /// Force MARC-XML format (no detection). Parses XML eagerly into ISO2709.
    pub fn from_xml(data: &[u8]) -> Result<Self, MarcError> {
        Ok(Self {
            data: XmlReader::parse(data)?,
            source_format: FileFormat::Xml,
            encoding_override: None,
        })
    }

    /// Force the encoding used when decoding raw record data. Use when the record's
    /// declared encoding (e.g. leader byte 9 for MARC21, 100$a for UNIMARC) is wrong.
    pub fn with_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding_override = Some(encoding);
        self
    }

    /// Clear any encoding override.
    pub fn clear_encoding_override(&mut self) {
        self.encoding_override = None;
    }

    /// Returns the original file format before conversion.
    pub fn source_format(&self) -> FileFormat {
        self.source_format
    }

    /// Returns the encoding override when set.
    pub fn encoding_override(&self) -> Option<Encoding> {
        self.encoding_override
    }

    /// Iterate over raw ISO2709 records (with optional encoding override applied when decoding).
    pub fn iter(&self) -> BinaryReader<'_> {
        BinaryReader::with_encoding(&self.data, self.encoding_override)
    }

    /// Raw ISO2709 bytes (useful for direct binary output).
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Parse all records into the high-level semantic model.
    /// If `with_encoding()` was used, that encoding is applied instead of the one
    /// detected from each record.
    pub fn into_records(self) -> Result<Vec<Record>, MarcError> {
        let mut records = Vec::new();
        for view in BinaryReader::with_encoding(&self.data, self.encoding_override) {
            let view = view?;
            let raw = view.as_raw();
            let format = MarcFormat::detect(raw, self.encoding_override)?;

            records.push(format.to_record(raw)?);
        }
        Ok(records)
    }
}

impl<'a> IntoIterator for &'a MarcReader {
    type Item = Result<RawRecordView<'a>, MarcError>;
    type IntoIter = BinaryReader<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
