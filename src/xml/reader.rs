use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::error::MarcError;

/// Converts MARC-XML data into concatenated ISO2709 bytes, allowing reuse
/// of the entire existing binary pipeline (format detection, encoding, mapping).
pub struct XmlReader;

struct XmlField {
    tag: [u8; 3],
    data: XmlFieldData,
}

enum XmlFieldData {
    Control(String),
    Data {
        ind1: u8,
        ind2: u8,
        subfields: Vec<(u8, String)>,
    },
}

impl XmlReader {
    /// Parse MARC-XML bytes into a single buffer of concatenated ISO2709 records.
    pub fn parse(data: &[u8]) -> Result<Vec<u8>, MarcError> {
        let mut reader = Reader::from_reader(data);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut output = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if local_name_eq(e, b"record") => {
                    let iso_bytes = Self::parse_record(&mut reader)?;
                    output.extend_from_slice(&iso_bytes);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(MarcError::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(output)
    }

    fn parse_record<R: BufRead>(reader: &mut Reader<R>) -> Result<Vec<u8>, MarcError> {
        let mut buf = Vec::new();
        let mut leader = String::new();
        let mut fields: Vec<XmlField> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let local = e.local_name();
                    match local.as_ref() {
                        b"leader" => {
                            leader = read_element_text(reader)?;
                        }
                        b"controlfield" => {
                            let tag = parse_tag(e)?;
                            let text = read_element_text(reader)?;
                            fields.push(XmlField {
                                tag,
                                data: XmlFieldData::Control(text),
                            });
                        }
                        b"datafield" => {
                            let tag = parse_tag(e)?;
                            let ind1 = parse_indicator(e, b"ind1");
                            let ind2 = parse_indicator(e, b"ind2");
                            let subfields = Self::parse_subfields(reader)?;
                            fields.push(XmlField {
                                tag,
                                data: XmlFieldData::Data {
                                    ind1,
                                    ind2,
                                    subfields,
                                },
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) if e.local_name().as_ref() == b"record" => break,
                Ok(Event::Eof) => {
                    return Err(MarcError::Xml("unexpected EOF inside <record>".into()))
                }
                Err(e) => return Err(MarcError::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        build_iso2709_from_xml(&leader, &fields)
    }

    fn parse_subfields<R: BufRead>(
        reader: &mut Reader<R>,
    ) -> Result<Vec<(u8, String)>, MarcError> {
        let mut buf = Vec::new();
        let mut subfields = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"subfield" => {
                    let code = parse_subfield_code(e);
                    let text = read_element_text(reader)?;
                    subfields.push((code, text));
                }
                Ok(Event::Empty(ref e)) if e.local_name().as_ref() == b"subfield" => {
                    let code = parse_subfield_code(e);
                    subfields.push((code, String::new()));
                }
                Ok(Event::End(ref e)) if e.local_name().as_ref() == b"datafield" => break,
                Ok(Event::Eof) => {
                    return Err(MarcError::Xml("unexpected EOF inside <datafield>".into()))
                }
                Err(e) => return Err(MarcError::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(subfields)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn local_name_eq(e: &BytesStart, name: &[u8]) -> bool {
    e.local_name().as_ref() == name
}

fn read_element_text<R: BufRead>(reader: &mut Reader<R>) -> Result<String, MarcError> {
    let mut buf = Vec::new();
    let mut text = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text.push_str(
                    &e.unescape()
                        .map_err(|e| MarcError::Xml(e.to_string()))?,
                );
            }
            Ok(Event::CData(e)) => {
                text.push_str(&String::from_utf8_lossy(&e));
            }
            Ok(Event::End(_)) => break,
            Ok(Event::Eof) => return Err(MarcError::Xml("unexpected EOF in element text".into())),
            Err(e) => return Err(MarcError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    Ok(text)
}

fn attr_value(e: &BytesStart, name: &[u8]) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| a.key.as_ref() == name)
        .map(|a| String::from_utf8_lossy(&a.value).into_owned())
}

fn parse_tag(e: &BytesStart) -> Result<[u8; 3], MarcError> {
    let val = attr_value(e, b"tag")
        .ok_or_else(|| MarcError::Xml("missing tag attribute".into()))?;
    let bytes = val.as_bytes();
    if bytes.len() != 3 {
        return Err(MarcError::Xml(format!(
            "tag must be exactly 3 characters, got '{val}'"
        )));
    }
    let mut tag = [0u8; 3];
    tag.copy_from_slice(bytes);
    Ok(tag)
}

fn parse_indicator(e: &BytesStart, name: &[u8]) -> u8 {
    attr_value(e, name)
        .and_then(|v| v.bytes().next())
        .unwrap_or(b' ')
}

fn parse_subfield_code(e: &BytesStart) -> u8 {
    attr_value(e, b"code")
        .and_then(|v| v.bytes().next())
        .unwrap_or(b'a')
}

/// Builds a valid ISO2709 byte sequence from XML-parsed fields.
/// The resulting bytes are always UTF-8 encoded (leader byte 9 = 'a').
fn build_iso2709_from_xml(leader_str: &str, fields: &[XmlField]) -> Result<Vec<u8>, MarcError> {
    let mut directory: Vec<u8> = Vec::new();
    let mut field_data: Vec<u8> = Vec::new();
    let mut offset: usize = 0;

    for field in fields {
        let mut fb: Vec<u8> = Vec::new();
        match &field.data {
            XmlFieldData::Control(text) => {
                fb.extend_from_slice(text.as_bytes());
                fb.push(0x1E);
            }
            XmlFieldData::Data {
                ind1,
                ind2,
                subfields,
            } => {
                fb.push(*ind1);
                fb.push(*ind2);
                for (code, value) in subfields {
                    fb.push(0x1F);
                    fb.push(*code);
                    fb.extend_from_slice(value.as_bytes());
                }
                fb.push(0x1E);
            }
        }

        let length = fb.len();
        directory.extend_from_slice(&field.tag);
        directory.extend_from_slice(format!("{length:0>4}").as_bytes());
        directory.extend_from_slice(format!("{offset:0>5}").as_bytes());
        field_data.extend_from_slice(&fb);
        offset += length;
    }

    directory.push(0x1E);
    let base_address = 24 + directory.len();

    let mut leader_bytes = [b' '; 24];
    let src = leader_str.as_bytes();
    let n = src.len().min(24);
    leader_bytes[..n].copy_from_slice(&src[..n]);

    leader_bytes[9] = b'a';
    leader_bytes[10] = b'2';
    leader_bytes[11] = b'2';
    leader_bytes[20..24].copy_from_slice(b"4500");

    let record_length = base_address + field_data.len() + 1;
    if record_length > 99999 {
        return Err(MarcError::InvalidRecord(
            "record too long for ISO2709 leader",
        ));
    }
    leader_bytes[0..5].copy_from_slice(format!("{record_length:0>5}").as_bytes());
    leader_bytes[12..17].copy_from_slice(format!("{base_address:0>5}").as_bytes());

    let mut out = Vec::with_capacity(record_length);
    out.extend_from_slice(&leader_bytes);
    out.extend_from_slice(&directory);
    out.extend_from_slice(&field_data);
    out.push(0x1D);
    Ok(out)
}
