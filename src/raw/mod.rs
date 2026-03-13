use std::fmt::Display;
use std::ops::Deref;

use crate::encoding::Encoding;
use crate::{MarcFormat, error::MarcError};

/// Zero-copy view over an ISO2709 MARC record.
///
/// Tuple struct by design: the inner slice is only exposed internally,
/// users go through the accessor methods and iterators.
#[derive(Clone, Copy, Debug)]
pub struct RawRecord<'a>(pub(crate) &'a [u8]);

impl<'a> RawRecord<'a> {
    pub fn data(&self) -> &'a [u8] {
        self.0
    }

    pub fn leader(&self) -> Result<&'a [u8], MarcError> {
        if self.0.len() < 24 {
            return Err(MarcError::InvalidRecord("record shorter than leader"));
        }
        Ok(&self.0[0..24])
    }

}

impl<'a> Display for RawRecord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        raw_record_display(self, None, f)
    }
}

pub struct OwnedRawRecord(pub(crate) Vec<u8>);

impl OwnedRawRecord {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
    pub fn leader(&self) -> Result<&[u8], MarcError> {
        if self.0.len() < 24 {
            return Err(MarcError::InvalidRecord("record shorter than leader"));
        }
        Ok(&self.0[0..24])
    }
    pub fn leader_mut(&mut self) -> Result<&mut [u8], MarcError> {
        if self.0.len() < 24 {
            return Err(MarcError::InvalidRecord("record shorter than leader"));
        }
        Ok(&mut self.0[0..24])
    }
    pub fn data(&self) -> &[u8] {
        &self.0
    }
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Raw record with optional encoding override for decoding field data.
#[derive(Clone, Copy, Debug)]
pub struct RawRecordView<'a> {
    pub(crate) raw: RawRecord<'a>,
    pub(crate) encoding_override: Option<Encoding>,
}

impl<'a> RawRecordView<'a> {
    pub fn new(raw: RawRecord<'a>, encoding_override: Option<Encoding>) -> Self {
        Self { raw, encoding_override }
    }

    pub fn as_raw(&self) -> &RawRecord<'a> {
        &self.raw
    }
}

impl<'a> Deref for RawRecordView<'a> {
    type Target = RawRecord<'a>;
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'a> Display for RawRecordView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        raw_record_display(&self.raw, self.encoding_override, f)
    }
}

fn raw_record_display(
    raw: &RawRecord<'_>,
    encoding_override: Option<Encoding>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for field in raw.fields().unwrap_or_default() {
        match field {
            RawField::Control { tag, data } => {
                write!(
                    f,
                    "{}: {}\n",
                    String::from_utf8_lossy(&tag),
                    String::from_utf8_lossy(data).trim()
                )?;
            }
            RawField::Data { tag, body, .. } => {
                let format = match MarcFormat::detect(raw, encoding_override) {
                    Ok(fmt) => fmt,
                    Err(_) => continue,
                };
                // let encoding = format.effective_encoding(encoding_override);
                let mut pos = 0;
                while pos < body.len() {
                    if body[pos] == 0x1F {
                        if pos + 1 >= body.len() {
                            break;
                        }
                        let code = body[pos + 1];
                        let start = pos + 2;
                        let mut end = start;
                        while end < body.len() && body[end] != 0x1F && body[end] != 0x1E {
                            end += 1;
                        }
                        let slice = &body[start..end];
                        let value = format.encoding().decode(slice).unwrap_or_else(|_| "".into());
                        write!(
                            f,
                            "{} ${}: {}\n",
                            String::from_utf8_lossy(&tag),
                            code as char,
                            String::from_utf8_lossy(value.as_bytes())
                        )?;
                        pos = end;
                    } else if body[pos] == 0x1E {
                        break;
                    } else {
                        pos += 1;
                    }
                }
            }
        }
    }
    Ok(())
}

/// One directory entry (tag, length, offset) in ISO2709.
#[derive(Clone, Copy, Debug)]
pub struct DirectoryEntry {
    pub tag: [u8; 3],
    pub length: usize,
    pub offset: usize,
}

/// Iterator over directory entries.
pub struct DirectoryIter<'a> {
    buf: &'a [u8],
    pos: usize,
    base_addr: usize,
}

impl<'a> DirectoryIter<'a> {
    pub fn new(record: &'a RawRecord<'a>) -> Result<Self, MarcError> {
        let data = record.data();
        if data.len() < 24 {
            return Err(MarcError::InvalidRecord("record shorter than leader"));
        }
        let leader = &data[..24];
        let base_addr = std::str::from_utf8(&leader[12..17])
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .ok_or(MarcError::InvalidRecord("invalid base address"))?;
        let mut end = 24;
        while end < data.len() && data[end] != 0x1E {
            end += 1;
        }
        Ok(Self {
            buf: &data[24..end],
            pos: 0,
            base_addr,
        })
    }
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = DirectoryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 12 > self.buf.len() {
            return None;
        }
        let chunk = &self.buf[self.pos..self.pos + 12];
        self.pos += 12;
        let mut tag = [0u8; 3];
        tag.copy_from_slice(&chunk[0..3]);
        let length_str = std::str::from_utf8(&chunk[3..7]).ok()?;
        let offset_str = std::str::from_utf8(&chunk[7..12]).ok()?;
        let length = match length_str.trim().parse::<usize>() {
            Ok(v) => v,
            Err(_) => return None,
        };
        let offset = match offset_str.trim().parse::<usize>() {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(DirectoryEntry {
            tag,
            length,
            offset: self.base_addr + offset,
        })
    }
}

/// Raw field view.
#[derive(Debug)]
pub enum RawField<'a> {
    Control {
        tag: [u8; 3],
        data: &'a [u8],
    },
    Data {
        tag: [u8; 3],
        indicators: [u8; 2],
        body: &'a [u8],
    },
}

impl<'a> RawRecord<'a> {
    pub fn fields(&'a self) -> Result<Vec<RawField<'a>>, MarcError> {
        let data = self.data();
        let dir_iter = DirectoryIter::new(self)?;
        let mut out = Vec::new();
        for entry in dir_iter {
            let end = (entry.offset + entry.length).min(data.len());
            if entry.offset >= end {
                continue;
            }
            let slice = &data[entry.offset..end];
            let mut tag = [0u8; 3];
            tag.copy_from_slice(&entry.tag);
            if &tag[0..2] == b"00" {
                out.push(RawField::Control { tag, data: slice });
            } else {
                if slice.len() < 3 {
                    continue;
                }
                let indicators = [slice[0], slice[1]];
                out.push(RawField::Data {
                    tag,
                    indicators,
                    body: &slice[2..],
                });
            }
        }
        Ok(out)
    }
}

mod reader;
pub use reader::BinaryReader;

mod writer;
pub use writer::BinaryWriter;

