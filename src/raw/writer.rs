use std::io::Write;

use crate::{MarcError, MarcFormat, Record};

pub struct BinaryWriter<W: Write> {
    writer: W,
}

impl<W: Write> BinaryWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write_raw(&mut self, data: &[u8]) -> Result<(), MarcError> {
        self.writer.write_all(data)?;
        Ok(())
    }

    pub fn write_record(&mut self, format: &MarcFormat, record: &mut Record) -> Result<(), MarcError> {
        let raw = format.to_raw(record)?;
        self.write_raw(raw.data())
    }

    pub fn flush(&mut self) -> Result<(), MarcError> {
        self.writer.flush()?;
        Ok(())
    }
}
