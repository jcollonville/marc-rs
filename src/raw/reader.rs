use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::raw::{RawRecord, RawRecordView};

/// Simple in-memory binary reader over concatenated ISO2709 records.
///
/// Yields `RawRecordView` so that an optional encoding override can be applied
/// when decoding field data (e.g. for display).
pub struct BinaryReader<'a> {
    data: &'a [u8],
    pos: usize,
    encoding_override: Option<Encoding>,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            encoding_override: None,
        }
    }

    pub fn with_encoding(data: &'a [u8], encoding_override: Option<Encoding>) -> Self {
        Self { data, pos: 0, encoding_override }
    }
}

impl<'a> Iterator for BinaryReader<'a> {
    type Item = Result<RawRecordView<'a>, MarcError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        if self.data.len() - self.pos < 24 {
            return Some(Err(MarcError::InvalidRecord("trailing bytes shorter than leader")));
        }
        let leader = &self.data[self.pos..self.pos + 24];
        let len = match std::str::from_utf8(&leader[0..5]).ok().and_then(|s| s.trim().parse::<usize>().ok()) {
            Some(l) if l >= 24 && self.pos + l <= self.data.len() => l,
            _ => return Some(Err(MarcError::InvalidRecord("invalid record length in leader"))),
        };
        let start = self.pos;
        let end = self.pos + len;
        self.pos = end;
        let raw = RawRecord(&self.data[start..end]);
        Some(Ok(RawRecordView::new(raw, self.encoding_override)))
    }
}
