use std::io::Write;

use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::raw::{RawField, RawRecord};
use crate::{MarcFormat, Record};

/// Writes MARC records as MARC-XML (MARCXML).
/// Output is always UTF-8, regardless of the source record encoding.
pub struct XmlWriter<W: Write> {
    writer: W,
}

impl<W: Write> XmlWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn start_collection(&mut self) -> Result<(), MarcError> {
        writeln!(
            self.writer,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        )?;
        writeln!(
            self.writer,
            "<collection xmlns=\"http://www.loc.gov/MARC21/slim\">"
        )?;
        Ok(())
    }

    pub fn end_collection(&mut self) -> Result<(), MarcError> {
        writeln!(self.writer, "</collection>")?;
        Ok(())
    }

    /// Write a raw ISO2709 record as a MARC-XML `<record>` element.
    /// Uses `encoding_override` when provided, otherwise detects from the record.
    pub fn write_raw(
        &mut self,
        raw: &RawRecord,
        encoding_override: Option<Encoding>,
    ) -> Result<(), MarcError> {
        let format = MarcFormat::detect(raw)?;
        let encoding = format.effective_encoding(encoding_override);

        writeln!(self.writer, "  <record>")?;

        // Leader (24 ASCII bytes)
        let leader = raw.leader()?;
        let leader_str = String::from_utf8_lossy(leader);
        writeln!(self.writer, "    <leader>{}</leader>", xml_escape(&leader_str))?;

        for field in raw.fields()? {
            match field {
                RawField::Control { tag, data } => {
                    let tag_str = String::from_utf8_lossy(&tag);
                    let clean = strip_field_terminator(data);
                    let text = encoding.decode(clean).unwrap_or_else(|_| "".into());
                    writeln!(
                        self.writer,
                        "    <controlfield tag=\"{tag_str}\">{}</controlfield>",
                        xml_escape(&text)
                    )?;
                }
                RawField::Data {
                    tag,
                    indicators,
                    body,
                } => {
                    let tag_str = String::from_utf8_lossy(&tag);
                    let ind1 = indicators[0] as char;
                    let ind2 = indicators[1] as char;
                    writeln!(
                        self.writer,
                        "    <datafield tag=\"{tag_str}\" ind1=\"{}\" ind2=\"{}\">",
                        xml_escape_char(ind1),
                        xml_escape_char(ind2)
                    )?;

                    let mut pos = 0;
                    while pos < body.len() {
                        if body[pos] == 0x1F {
                            if pos + 1 >= body.len() {
                                break;
                            }
                            let code = body[pos + 1] as char;
                            let start = pos + 2;
                            let mut end = start;
                            while end < body.len() && body[end] != 0x1F && body[end] != 0x1E {
                                end += 1;
                            }
                            let text =
                                encoding.decode(&body[start..end]).unwrap_or_else(|_| "".into());
                            writeln!(
                                self.writer,
                                "      <subfield code=\"{}\">{}</subfield>",
                                xml_escape_char(code),
                                xml_escape(&text)
                            )?;
                            pos = end;
                        } else if body[pos] == 0x1E {
                            break;
                        } else {
                            pos += 1;
                        }
                    }

                    writeln!(self.writer, "    </datafield>")?;
                }
            }
        }

        writeln!(self.writer, "  </record>")?;
        Ok(())
    }

    /// Convert a semantic `Record` to ISO2709 via the given format, then write as XML.
    pub fn write_record(&mut self, format: &MarcFormat, record: &Record) -> Result<(), MarcError> {
        let raw_bytes = format.to_raw(record)?;
        let raw = RawRecord(&raw_bytes);
        self.write_raw(&raw, None)
    }

    pub fn flush(&mut self) -> Result<(), MarcError> {
        self.writer.flush()?;
        Ok(())
    }
}

fn strip_field_terminator(data: &[u8]) -> &[u8] {
    if data.last() == Some(&0x1E) {
        &data[..data.len() - 1]
    } else {
        data
    }
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

fn xml_escape_char(c: char) -> String {
    match c {
        '&' => "&amp;".into(),
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '"' => "&quot;".into(),
        '\'' => "&apos;".into(),
        _ => c.to_string(),
    }
}
