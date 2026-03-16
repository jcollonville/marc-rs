use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::raw::{OwnedRawRecord, RawRecord};
use crate::record::Record;

#[derive(Debug, Clone, Copy)]
pub enum MarcFormat {
    Marc21(Encoding),
    Unimarc(Encoding),
}

impl MarcFormat {
    /// Auto-detects the format by looking for fields 200 (UNIMARC) and 245 (MARC21).
    /// A record with 200 but without 245 is treated as UNIMARC; otherwise MARC21.
    pub fn detect(record: &RawRecord<'_>, encoding_override: Option<Encoding>) -> Result<Self, MarcError> {
        let mut has_200 = false;
        let mut has_245 = false;
        for field in record.fields()? {
            match field {
                crate::raw::RawField::Control { tag, .. } | crate::raw::RawField::Data { tag, .. } => {
                    if &tag == b"200" {
                        has_200 = true;
                    } else if &tag == b"245" {
                        has_245 = true;
                    }
                }
            }
        }
        let format = Ok(if has_200 && !has_245 {
            MarcFormat::Unimarc(encoding_override.unwrap_or(unimarc::detect_encoding(record)?))
        } else {
            MarcFormat::Marc21(encoding_override.unwrap_or(marc21::detect_encoding(record)?))
        });
        format
    }

    pub fn encoding(&self) -> &Encoding {
        match self {
            MarcFormat::Marc21(encoding) => encoding,
            MarcFormat::Unimarc(encoding) => encoding,
        }
    }

    pub fn effective_encoding(&self, override_enc: Option<Encoding>) -> Encoding {
        override_enc.unwrap_or(*self.encoding())
    }

    pub fn to_record(&self, record: &RawRecord<'_>) -> Result<Record, MarcError> {
        match self {
            MarcFormat::Marc21(encoding) => marc21::to_record(encoding, record),
            MarcFormat::Unimarc(encoding) => unimarc::to_record(encoding, record),
        }
    }

    pub fn to_raw(&self, record: &Record) -> Result<OwnedRawRecord, MarcError> {
        match self {
            MarcFormat::Marc21(encoding) => marc21::to_raw(encoding, record),
            MarcFormat::Unimarc(encoding) => unimarc::to_raw(encoding, record),
        }
    }
}

pub enum ReverseFieldData {
    Control(String),
    Data { ind1: u8, ind2: u8, subfields: Vec<(u8, String)> },
}

/// Leader layout parameters passed from the compiled config.
pub(crate) struct LeaderLayout<'a> {
    pub reverse_positions: &'a [(usize, usize, String)],
    pub directory_map: &'a [u8; 4],
}

pub(crate) fn build_iso2709(encoding: &Encoding, fields: &[([u8; 3], ReverseFieldData)], layout: &LeaderLayout<'_>) -> Result<OwnedRawRecord, MarcError> {
    let mut directory: Vec<u8> = Vec::new();
    let mut field_data: Vec<u8> = Vec::new();
    let mut offset: usize = 0;

    for (tag, data) in fields {
        let mut field_bytes: Vec<u8> = Vec::new();
        match data {
            ReverseFieldData::Control(text) => {
                field_bytes.extend_from_slice(&encoding.encode(text)?);
                field_bytes.push(0x1E);
            }
            ReverseFieldData::Data { ind1, ind2, subfields } => {
                field_bytes.push(*ind1);
                field_bytes.push(*ind2);
                for (code, value) in subfields {
                    field_bytes.push(0x1F);
                    field_bytes.push(*code);
                    field_bytes.extend_from_slice(&encoding.encode(value)?);
                }
                field_bytes.push(0x1E);
            }
        }

        let length = field_bytes.len();
        directory.extend_from_slice(tag);
        directory.extend_from_slice(format!("{:0>4}", length).as_bytes());
        directory.extend_from_slice(format!("{:0>5}", offset).as_bytes());

        field_data.extend_from_slice(&field_bytes);
        offset += length;
    }

    directory.push(0x1E);

    let base_address = 24 + directory.len();

    let mut leader_bytes = [b' '; 24];
    leader_bytes[0..5].copy_from_slice(b"00000");
    leader_bytes[10] = b'2';
    leader_bytes[11] = b'2';
    leader_bytes[12..17].copy_from_slice(format!("{:0>5}", base_address).as_bytes());
    leader_bytes[20..24].copy_from_slice(layout.directory_map);

    for (pos, len, raw_value) in layout.reverse_positions {
        let bytes = raw_value.as_bytes();
        let copy_len = bytes.len().min(*len).min(24 - pos);
        leader_bytes[*pos..*pos + copy_len].copy_from_slice(&bytes[..copy_len]);
    }

    let record_length = base_address + field_data.len() + 1;
    if record_length > 99999 {
        return Err(MarcError::InvalidRecord("record too long for ISO2709 leader"));
    }
    leader_bytes[0..5].copy_from_slice(format!("{:0>5}", record_length).as_bytes());

    let mut out = Vec::with_capacity(record_length);
    out.extend_from_slice(&leader_bytes);
    out.extend_from_slice(&directory);
    out.extend_from_slice(&field_data);
    out.push(0x1D);

    Ok(OwnedRawRecord::new(out))
}

pub mod config;
pub mod engine;
pub mod marc21;
pub mod unimarc;
