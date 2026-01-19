//! # marc-rs
//!
//! A Rust library for parsing and writing MARC21, UNIMARC, and MARC XML bibliographic records.
//!
//! ## Features
//!
//! - Support for MARC21, UNIMARC, and MARC XML formats
//! - Multiple character encodings (UTF-8, MARC-8, ISO-8859-*)
//! - Parse multiple records from a single buffer
//! - Write single or multiple records
//! - Optional Serde support for serialization/deserialization
//!
//! ## Examples
//!
//! ### Parsing MARC21 records
//!
//! ```no_run
//! use marc_rs::{parse, FormatEncoding, MarcFormat, Encoding};
//!
//! let data = b"..."; // MARC binary data
//! let format_encoding = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
//! let records = parse(data, format_encoding).unwrap();
//! ```
//!
//! ### Writing MARC XML
//!
//! ```no_run
//! use marc_rs::{write, FormatEncoding, MarcFormat, Encoding, Record};
//! use std::io::stdout;
//!
//! let records = vec![/* ... */];
//! let format_encoding = FormatEncoding::marc_xml();
//! write(&records, format_encoding, &mut stdout()).unwrap();
//! ```
//!
//! ### Serde Serialization/Deserialization
//!
//! With the `serde` feature enabled, you can serialize/deserialize directly to/from MARC formats:
//!
//! ```ignore
//! use marc_rs::{Record, FormatEncoding, MarcFormat, Encoding, serde_marc};
//! use std::fs::File;
//!
//! fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Deserialize from bytes
//!     let data = b"..."; // MARC binary data
//!     let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
//!     let record = serde_marc::from_slice(data, format)?;
//!
//!     // Deserialize from reader
//!     let file = File::open("record.mrc")?;
//!     let record = serde_marc::from_reader(file, format)?;
//!
//!     // Serialize to bytes
//!     let bytes = serde_marc::to_vec(&record, format)?;
//!
//!     // Serialize to writer
//!     let mut output = Vec::new();
//!     serde_marc::to_writer(&record, format, &mut output)?;
//!
//!     // For XML format, you can also use string functions
//!     let xml_format = FormatEncoding::marc_xml();
//!     let xml_string = serde_marc::to_string(&record, xml_format)?;
//!     let record_from_xml = serde_marc::from_str(&xml_string, xml_format)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## References
//!
//! - [MARC 21 Format for Bibliographic Data](https://www.loc.gov/marc/bibliographic/)
//! - [MARC XML Schema](https://www.loc.gov/standards/marcxml/schema/MARC21slim.xsd)
//! - [UNIMARC Manual](https://www.transition-bibliographique.fr/unimarc/manuel-unimarc-format-bibliographique/)

pub mod encoding;
pub mod fields;
pub mod format;
pub mod parser;
pub mod record;
pub mod writer;

#[cfg(feature = "serde")]
pub mod serde_marc;

pub use encoding::*;
pub use fields::*;
pub use format::*;
pub use parser::*;
pub use record::*;
pub use writer::*;

