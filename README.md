[![crates.io](https://img.shields.io/crates/v/marc-rs.svg)](https://crates.io/crates/marc-rs)
[![docs.rs](https://docs.rs/marc-rs/badge.svg)](https://docs.rs/marc-rs)

# marc-rs

A Rust library for parsing and writing MARC21, UNIMARC, and MARC XML bibliographic records.

## Features

- Support for MARC21, UNIMARC, and MARC XML formats
- Multiple character encodings (UTF-8, MARC-8, ISO-8859-*, ISO-5426)
- Parse multiple records from a single buffer
- Write single or multiple records
- Optional Serde support for serialization/deserialization
- Comprehensive field type enums organized by category

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
marc-rs = "0.1.0"

# Optional: Enable Serde support
marc-rs = { version = "0.1.0", features = ["serde"] }
```

## Usage

### Parsing MARC21 Records

```rust
use marc_rs::{parse, FormatEncoding, MarcFormat, Encoding};

let data = b"..."; // MARC binary data
let format_encoding = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
let records = parse(data, format_encoding)?;

for record in records {
    println!("Record with {} fields", record.data_fields.len());
}
```

### Writing MARC XML

```rust
use marc_rs::{write, FormatEncoding, Record};
use std::io::stdout;

let records = vec![/* your records */];
let format_encoding = FormatEncoding::marc_xml();
write(&records, format_encoding, &mut stdout())?;
```

### Using Field Enums

```rust
use marc_rs::{MainEntry, Title, Subject, MarcFormat};

// Tags depend on the format
let format = MarcFormat::Marc21;
let main_entry_tag = MainEntry::PersonalName.tag(format); // "100" in MARC21, "700" in UNIMARC
let title_tag = Title::TitleStatement.tag(format); // "245" in MARC21, "200" in UNIMARC
let subject_tag = Subject::SubjectTopicalTerm.tag(format); // "650" in MARC21, "606" in UNIMARC
```

### Serde Support

With the `serde` feature enabled, you can serialize/deserialize directly to/from MARC formats:

```rust
use marc_rs::{Record, FormatEncoding, MarcFormat, Encoding, serde_marc};
use std::fs::File;

// Deserialize from bytes
let data = b"..."; // MARC binary data
let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
let record = serde_marc::from_slice(data, format)?;

// Deserialize from reader
let file = File::open("record.mrc")?;
let record = serde_marc::from_reader(file, format)?;

// Deserialize from string (for XML)
let xml = r#"<?xml version="1.0"?><record>...</record>"#;
let xml_format = FormatEncoding::marc_xml();
let record = serde_marc::from_str(xml, xml_format)?;

// Serialize to bytes
let bytes = serde_marc::to_vec(&record, format)?;

// Serialize to writer
let mut output = Vec::new();
serde_marc::to_writer(&record, format, &mut output)?;

// Serialize to string (for XML)
let xml_string = serde_marc::to_string(&record, xml_format)?;

// Multiple records
let records = serde_marc::from_slice_many(data, format)?;
let bytes = serde_marc::to_records(&records, format)?;

// Or use JSON for cross-format serialization
use serde_json;
let json = serde_json::to_string(&record)?;
let deserialized: Record = serde_json::from_str(&json)?;
```

## Format Support

### MARC21
- Binary format parsing and writing
- XML format parsing and writing
- Default encoding: MARC-8

### UNIMARC
- Binary format parsing and writing
- XML format parsing and writing
- Default encoding: UTF-8

### MARC XML
- Full XML parsing with collection support
- XML writing with automatic collection wrapping for multiple records

## Character Encodings

Supported encodings:
- UTF-8
- MARC-8 (fallback to ISO-8859-1)
- ISO-8859-1 (Latin-1)
- ISO-8859-2 (Latin-2)
- ISO-8859-5 (Cyrillic)
- ISO-8859-7 (Greek)
- ISO-8859-15 (Latin-9)
- ISO-5426 (Extension of the Latin alphabet for bibliographic information interchange)

## Field Categories

The library provides enums for different field categories:

- **Main Entry** (1XX): Personal names, corporate names, meeting names, uniform titles
- **Title** (20X-24X): Title statements, varying forms, former titles
- **Edition** (25X): Edition statements, cartographic data, computer file characteristics
- **Physical Description** (3XX): Physical descriptions, playing time, publication frequency
- **Series** (4XX): Series statements and added entries
- **Notes** (5XX): General notes, contents notes, summary, etc.
- **Subject Access** (6XX): Subject headings, topical terms, geographic names
- **Added Entries** (70X-75X): Personal names, corporate names, uniform titles
- **Linking Entries** (76X-78X): Series entries, translation entries, related entries
- **Control Fields** (00X): Control numbers, fixed-length data elements

## Command Line Tool

The crate includes a command-line viewer tool to inspect MARC files:

```bash
# Build the viewer (requires serde feature)
cargo build --bin marc-viewer --features serde

# View a MARC file (auto-detect format, plain output)
cargo run --bin marc-viewer --features serde -- path/to/file.mrc

# Specify format explicitly
cargo run --bin marc-viewer --features serde -- path/to/file.mrc marc21

# Specify format and encoding
cargo run --bin marc-viewer --features serde -- path/to/file.mrc unimarc utf8

# Output in JSON format
cargo run --bin marc-viewer --features serde -- path/to/file.mrc marc21 utf8 json

# Output in XML format
cargo run --bin marc-viewer --features serde -- path/to/file.mrc marc21 utf8 xml

# Output in MARC21 binary format
cargo run --bin marc-viewer --features serde -- path/to/file.mrc marc21 utf8 marc > output.mrc

# Output in UNIMARC binary format
cargo run --bin marc-viewer --features serde -- path/to/file.mrc unimarc utf8 unimarc > output.mrc
```

The viewer supports five output formats:
- **plain** (default): Human-readable text format with leader, control fields, and data fields
- **json**: JSON serialization using serde_json
- **xml**: MARC XML format using serde_marc
- **marc** or **marc21**: MARC21 binary format using serde_marc (outputs to stdout)
- **unimarc**: UNIMARC binary format using serde_marc (outputs to stdout)

The plain format displays:
- File information and detected format
- Leader information
- All control fields (001-009)
- All data fields with indicators and subfields

## References

- [MARC 21 Format for Bibliographic Data](https://www.loc.gov/marc/bibliographic/)
- [MARC XML Schema](https://www.loc.gov/standards/marcxml/schema/MARC21slim.xsd)
- [UNIMARC Manual](https://www.transition-bibliographique.fr/unimarc/manuel-unimarc-format-bibliographique/)

## License

MIT OR Apache-2.0
