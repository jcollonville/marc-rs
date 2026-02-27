use crate::encoding::{convert_to_utf8, convert_to_utf8_heuristic};
use crate::fields::{
    AddedEntry, Control, DeweyClassification, Edition, Isbn, LanguageData, Linking, MainEntry,
    Note, Physical, Series, Specimen, Subject, Title,
};
use crate::format::{Encoding, FormatEncoding, MarcFormat};
use crate::leader::*;
use crate::record::{ControlField, DataField, Record, Subfield};

/// Result of auto-detection parsing.
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub records: Vec<Record>,
    /// Detected container/source format (MarcXml, Marc21, or Unimarc).
    pub format: MarcFormat,
    /// Semantic format used for field dispatch (Marc21 or Unimarc — never MarcXml).
    pub semantic_format: MarcFormat,
}

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

/// Check if raw data looks like XML (handles optional BOM and leading whitespace).
fn is_xml_data(data: &[u8]) -> bool {
    let mut start = 0;
    // Skip UTF-8 BOM
    if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
        start = 3;
    }
    let trimmed = match data[start..].iter().position(|&b| !b.is_ascii_whitespace()) {
        Some(i) => &data[start + i..],
        None => return false,
    };
    trimmed.starts_with(b"<?xml")
        || trimmed.starts_with(b"<record")
        || trimmed.starts_with(b"<collection")
}

/// Detect the MARC format from raw data.
/// Returns `MarcXml` for XML content, `Marc21` or `Unimarc` for binary content.
pub fn detect_format(data: &[u8]) -> Result<MarcFormat, ParseError> {
    if is_xml_data(data) {
        return Ok(MarcFormat::MarcXml);
    }
    if data.len() < 24 {
        return Err(ParseError::Other(
            "Data too short to detect format".into(),
        ));
    }
    let leader = Leader::from_bytes(&data[..24]).map_err(ParseError::InvalidLeader)?;
    let base_address = leader.base_address_of_data as usize;
    if base_address <= 24 || data.len() < base_address {
        return Err(ParseError::UnexpectedEof);
    }
    let directory = &data[24..base_address];
    Ok(detect_record_format(data, directory))
}

/// Detect semantic format (Marc21 vs Unimarc) from XML content by scanning for tag patterns.
fn detect_xml_semantic_format(data: &[u8]) -> MarcFormat {
    let text = std::str::from_utf8(data).unwrap_or("");
    let mut marc21_score: i32 = 0;
    let mut unimarc_score: i32 = 0;

    if text.contains("tag=\"245\"") { marc21_score += 3; }
    if text.contains("tag=\"020\"") { marc21_score += 2; }
    if text.contains("tag=\"650\"") { marc21_score += 2; }
    if text.contains("tag=\"260\"") { marc21_score += 1; }
    if text.contains("tag=\"264\"") { marc21_score += 1; }
    if text.contains("tag=\"300\"") { marc21_score += 1; }

    if text.contains("tag=\"200\"") { unimarc_score += 3; }
    if text.contains("tag=\"010\"") { unimarc_score += 2; }
    if text.contains("tag=\"606\"") { unimarc_score += 2; }
    if text.contains("tag=\"215\"") { unimarc_score += 2; }
    if text.contains("tag=\"225\"") { unimarc_score += 1; }
    if text.contains("tag=\"101\"") { unimarc_score += 2; }
    if text.contains("tag=\"102\"") { unimarc_score += 2; }
    if text.contains("tag=\"801\"") { unimarc_score += 2; }

    if unimarc_score > marc21_score {
        MarcFormat::Unimarc
    } else {
        MarcFormat::Marc21
    }
}

/// Unified binary parser. If `forced_format` is None, auto-detects per record.
fn parse_binary(data: &[u8], forced_format: Option<MarcFormat>) -> Result<Vec<Record>, ParseError> {
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
        let record = parse_single_record(record_data, &leader, forced_format)?;
        records.push(record);

        offset += record_length;
    }

    Ok(records)
}

/// Parse MARC records with automatic format detection.
///
/// `forced_format` overrides auto-detection:
/// - `None` — container (XML/binary) and semantic format (Marc21/Unimarc) are both auto-detected.
/// - `Some(MarcFormat::MarcXml)` — forces XML parsing; semantic format is auto-detected from tags.
/// - `Some(MarcFormat::Marc21)` or `Some(MarcFormat::Unimarc)` — forces that semantic format;
///   container is still auto-detected from content.
///
/// Returns a `ParseResult` with records, detected container format, and semantic format.
pub fn parse_auto(
    data: &[u8],
    forced_format: Option<MarcFormat>,
) -> Result<ParseResult, ParseError> {


    let format = detect_format(data)?;
    let is_xml = is_xml_data(data) || forced_format == Some(MarcFormat::MarcXml);

    if is_xml {
        let semantic = match forced_format {
            Some(MarcFormat::Marc21) => MarcFormat::Marc21,
            Some(MarcFormat::Unimarc) => MarcFormat::Unimarc,
            _ => detect_xml_semantic_format(data),
        };
        let fe = FormatEncoding::new(semantic, Encoding::Utf8);
        let records = parse_marc_xml(data, fe)?;
        Ok(ParseResult {
            records,
            format: MarcFormat::MarcXml,
            semantic_format: semantic,
        })
    } else {
        let forced_semantic = match forced_format {
            Some(f @ (MarcFormat::Marc21 | MarcFormat::Unimarc)) => Some(f),
            _ => None,
        };
        let semantic = match forced_semantic {
            Some(f) => f,
            None => {
                let detected = detect_format(data)?;
                if detected == MarcFormat::MarcXml {
                    MarcFormat::Marc21
                } else {
                    detected
                }
            }
        };
        let records = parse_binary(data, forced_semantic)?;
        Ok(ParseResult {
            records,
            format: semantic,
            semantic_format: semantic,
        })
    }
}

/// Parse MARC records from bytes (legacy API — prefer `parse_auto` for auto-detection).
pub fn parse(data: &[u8], format_encoding: FormatEncoding) -> Result<Vec<Record>, ParseError> {
    match format_encoding.format {
        MarcFormat::Marc21 => parse_marc21_binary(data, format_encoding),
        MarcFormat::Unimarc => parse_unimarc_binary(data, format_encoding),
        MarcFormat::MarcXml => parse_marc_xml(data, format_encoding),
    }
}

/// Parse MARC21 binary format (legacy — prefer `parse_auto`).
pub fn parse_marc21_binary(
    data: &[u8],
    _format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    parse_binary(data, Some(MarcFormat::Marc21))
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

/// Heuristic detection of MARC flavor (MARC21 vs UNIMARC) for a binary record.
/// Uses weighted scoring on directory tags — mirrors `detect_xml_semantic_format`.
fn detect_record_format(record_data: &[u8], directory: &[u8]) -> MarcFormat {
    if record_data.len() > 9 && record_data[9] == b'a' {
        return MarcFormat::Marc21;
    }

    let mut marc21_score: i32 = 0;
    let mut unimarc_score: i32 = 0;
    let mut dir_offset = 0;

    while dir_offset + 12 <= directory.len() {
        let tag_bytes = &directory[dir_offset..dir_offset + 3];
        match tag_bytes {
            // Strong UNIMARC signals
            b"200" => unimarc_score += 3, // title (UNIMARC-exclusive)
            b"010" => unimarc_score += 2, // ISBN
            b"215" => unimarc_score += 2, // physical description
            b"101" | b"102" => unimarc_score += 2, // language / country coded
            b"801" => unimarc_score += 2, // originating source
            b"225" | b"606" | b"607" | b"608" | b"676" | b"952" | b"995" => {
                unimarc_score += 1;
            }
            // Weak: tag 100 exists in both formats but UNIMARC uses it as coded data
            // (single $a with fixed-length value) — only a mild signal
            b"100" => unimarc_score += 1,

            // Strong MARC21 signals
            b"245" => marc21_score += 3, // title (MARC21-exclusive)
            b"020" => marc21_score += 2, // ISBN
            b"300" => marc21_score += 2, // physical description
            b"650" | b"651" => marc21_score += 2, // subject headings
            b"260" | b"264" => marc21_score += 1, // publication
            b"041" | b"050" | b"082" | b"008" => marc21_score += 1,
            _ => {}
        }
        dir_offset += 12;
    }

    if unimarc_score > marc21_score {
        MarcFormat::Unimarc
    } else {
        MarcFormat::Marc21
    }
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
        record.push_control(ctrl);
    } else {
        record.push_other_control(ControlField {
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
    // Try each field module in priority order (ISBN before Physical so 010 UNIMARC is ISBN)
    if let Some(isbn) = Isbn::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_isbn(isbn);
        return;
    }
    if let Some(t) = Title::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_title(t);
        return;
    }
    if let Some(me) = MainEntry::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_main_entry(me);
        return;
    }
    if let Some(ed) = Edition::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_edition(ed);
        return;
    }
    if let Some(ph) = Physical::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_physical(ph);
        return;
    }
    if let Some(se) = Series::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_series(se);
        return;
    }
    if let Some(dc) = DeweyClassification::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_classification(dc);
        return;
    }
    if let Some(lang) = LanguageData::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_language(lang);
        return;
    }
    if let Some(no) = Note::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_note(no);
        return;
    }
    if let Some(su) = Subject::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_subject(su);
        return;
    }
    if let Some(ae) = AddedEntry::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_added_entry(ae);
        return;
    }
    if let Some(li) = Linking::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_linking(li);
        return;
    }
    if let Some(sp) = Specimen::try_parse(tag, ind1, ind2, subfields, format) {
        record.push_specimen(sp);
        return;
    }

    // Unrecognized tag => other_data
    record.push_other_data(DataField {
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
    forced_format: Option<MarcFormat>,
) -> Result<Record, ParseError> {
    if data.len() < leader.base_address_of_data as usize {
        return Err(ParseError::UnexpectedEof);
    }

    let base_address = leader.base_address_of_data as usize;
    let directory = &data[24..base_address];
    let data_area = &data[base_address..];

    let format = forced_format.unwrap_or_else(|| detect_record_format(data, directory));
    let record_encoding = detect_record_encoding(data, leader, format);

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

/// Parse UNIMARC binary format (legacy — prefer `parse_auto`).
pub fn parse_unimarc_binary(
    data: &[u8],
    _format_encoding: FormatEncoding,
) -> Result<Vec<Record>, ParseError> {
    parse_binary(data, Some(MarcFormat::Unimarc))
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
        record_status: RecordStatus::Unknown(' '),
        record_type: RecordType::Unknown(' '),
        bibliographic_level: BibliographicLevel::Unknown(' '),
        type_of_control: TypeOfControl::NoSpecifiedType,
        character_coding_scheme: CharacterCodingScheme::Marc8OrUnspecified,
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 0,
        encoding_level: EncodingLevel::Unknown(' '),
        descriptive_cataloging_form: DescriptiveCatalogingForm::NonIsbd,
        multipart_resource_record_level: MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: LeaderUndefined::Blank,
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
                            let leader = Leader::from_bytes(&leader_bytes)
                                .map_err(ParseError::InvalidLeader)?;
                            record.set_leader(leader);
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
