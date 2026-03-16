[![crates.io](https://img.shields.io/crates/v/marc-rs.svg)](https://crates.io/crates/marc-rs)
[![docs.rs](https://docs.rs/marc-rs/badge.svg)](https://docs.rs/marc-rs)

# marc-rs

Rust library for reading and writing bibliographic records in **MARC21**, **UNIMARC**, and **MARC XML** formats.

## What the library does

```
.mrc / .xml file
      │
      ▼
 RawRecord          ← zero-copy view over raw ISO2709 bytes
      │  JSON dictionary (marc21.json / unimarc.json)
      ▼
   Record           ← structured semantic model, serializable to JSON
```

The format (MARC21 or UNIMARC) and the character encoding are **auto-detected** on read.
The reverse conversion (`Record → binary`) is also supported.

## Features

- Read and write binary MARC21, binary UNIMARC, MARC XML
- Auto-detection of format and encoding
- High-level `Record` model organized by blocks (0XX–9XX)
- Helpers on `Record`: `title_main()`, `authors()`, `isbn()`, `media_type()`, etc.
- Serde support: native JSON serialization/deserialization
- Encodings: UTF-8, MARC-8, ISO-5426, ISO-8859-*

## Installation

```toml
[dependencies]
marc-rs = "0.0.5"
```

## Usage

### Read records from a file

```rust
use marc_rs::MarcReader;

let reader = MarcReader::from_file("records.mrc".as_ref())?;
let records = reader.into_records()?;

for record in &records {
    println!("{:?}", record.title_main());
    println!("{:?}", record.media_type());   // RecordType: LanguageMaterial, Video, Sound…
    println!("{:?}", record.authors().collect::<Vec<_>>());
}
```

### Read from bytes

```rust
use marc_rs::{parse_records, MarcReader, Encoding};

// Auto-detect format + encoding
let records = parse_records(&data)?;

// Force encoding if the record is incorrectly declared
let records = MarcReader::from_bytes(data)?
    .with_encoding(Encoding::Iso5426)
    .into_records()?;
```

### Convert a Record to binary MARC21

```rust
use marc_rs::{MarcFormat, Encoding};

let format = MarcFormat::Marc21(Encoding::Utf8);
let raw = format.to_raw(&record)?;
std::fs::write("out.mrc", raw.as_bytes())?;
```

### JSON serialization

```rust
let json = serde_json::to_string_pretty(&record)?;
let record: marc_rs::Record = serde_json::from_str(&json)?;
```

## How dictionaries work

The translation between raw MARC fields and the `Record` model is driven by two JSON files:
`resources/marc21.json` and `resources/unimarc.json`. These files are compiled into the binary via `include_str!`.

### Dictionary structure

```json
{
  "name": "marc21",
  "leader": [ ... ],
  "encoding_indicator": { ... },
  "rules": { ... },
  "blocks": [ ... ]
}
```

#### `leader`

List of positions within the 24 bytes of the ISO2709 leader. Each entry extracts one or more bytes and translates them to a field of the model.

```json
{ "position": 6, "target": "record_type", "rules": [
    { "raw": "a", "value": "languageMaterial" },
    { "raw": "g", "value": "projectedMedium" }
]}
```

→ Byte 6 of the leader becomes `record.leader.record_type`.

#### `encoding_indicator`

Indicates where to read the character encoding:
- **MARC21**: byte 9 of the leader (`"a"` = UTF-8, ` ` = MARC-8)
- **UNIMARC**: subfield `$a` of field 100, positions 26–28 (`"50"` = UTF-8, `"01"` = ISO-5426)

#### `rules`

Named reusable translation tables. Example: the `"languages"` table is shared by all language fields to translate `"fre"` → `"french"`, `"eng"` → `"english"`, etc.

#### `blocks`

List of field blocks (0XX, 1XX, 2XX…). Each block contains **fields** (`FieldDef`) with their **subfields** (`SubfieldBinding`).

A subfield binding maps a MARC subfield code to a dotted path in the `Record` model:

```json
{
  "tag": "245",
  "subfields": [
    { "code": "a", "target": "description.title.main" },
    { "code": "b", "target": "description.title.subtitle" },
    { "code": "c", "target": "description.title.responsibility" }
  ]
}
```

For fields with fixed-length subfields (such as UNIMARC 100$a), a `"slice"` allows extracting a specific position:

```json
{ "code": "a", "target": "coded.date_entered_on_file", "slice": { "offset": 0, "length": 8 } }
```

### Format auto-detection

On read, the engine checks whether the record contains field `200` (UNIMARC title) without field `245` (MARC21 title). If so → UNIMARC; otherwise → MARC21.

### Adding new fields

To map a field not yet supported, simply add an entry in the appropriate block of the JSON file — no Rust code to modify.

## `Record` model

```
Record
├── leader          (RecordType, BibliographicLevel, RecordStatus…)
├── identification  (ISBN, ISSN, LCCN, control numbers…)
├── coded           (languages, country, target audience, dates…)
├── description     (title, edition, publication, physical description…)
├── notes           (general notes, summary, table of contents…)
├── links           (links to other records)
├── associated_titles
├── indexing        (subjects, classifications, uncontrolled terms)
├── responsibility  (main and added entries)
├── international   (cataloging sources, locations, electronic access)
└── local           (specimens)
```

Available helpers on `Record`:

| Method | Return |
|--------|--------|
| `media_type()` | `&RecordType` (text, video, sound…) |
| `authors()` | `Iterator<Item = &Agent>` |
| `titles()` | `Vec<&Title>` |
| `title_main()` | `Option<&str>` |
| `isbn()` | `&[Isbn]` |
| `isbn_string()` | `Option<String>` |
| `languages()` | `&[Language]` |
| `lang_primary()` | `Option<&Language>` |
| `lang_original()` | `Option<&Language>` |
| `audience()` | `Option<&TargetAudience>` |
| `subject_main()` | `Option<&str>` |
| `keywords()` | `&[String]` |
| `publication_date()` | `Option<&str>` |
| `abstract_text()` | `Option<&str>` |
| `specimens()` | `&[Specimen]` |

## Command-line tool

```bash
# Display fields in human-readable mode (auto-detection)
cargo run --bin marc-rs -- records.mrc

# JSON output
cargo run --bin marc-rs -- records.mrc json

# XML output
cargo run --bin marc-rs -- records.mrc xml

# Convert to MARC21 UTF-8
cargo run --bin marc-rs -- records.mrc marc21-utf8 > out.mrc

# Convert to UNIMARC ISO-5426
cargo run --bin marc-rs -- records.mrc unimarc-iso5426 > out.mrc

# Force input encoding
cargo run --bin marc-rs -- --encoding utf8 records.mrc fields
```

## Supported encodings

| Identifier | Description |
|------------|-------------|
| `utf8` | Unicode UTF-8 |
| `marc8` | MARC-8 (fallback to Windows-1252) |
| `iso5426` | ISO-5426 (extended bibliographic Latin) |
| `iso8859_2` to `iso8859_5` | Latin-2, Latin-3, Cyrillic… |

## References

- [MARC 21 Format for Bibliographic Data](https://www.loc.gov/marc/bibliographic/)
- [MARC XML Schema](https://www.loc.gov/standards/marcxml/schema/MARC21slim.xsd)
- [UNIMARC Manual](https://www.transition-bibliographique.fr/unimarc/manuel-unimarc-format-bibliographique/)

## License

MIT OR Apache-2.0
