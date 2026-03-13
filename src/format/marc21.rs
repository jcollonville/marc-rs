use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::format::engine::{load_config, CompiledConfig};
use crate::raw::{OwnedRawRecord, RawRecord};
use crate::record::Record;

use lazy_static::lazy_static;

const MARC21_JSON: &str = include_str!("../../resources/marc21.json");

lazy_static! {
    static ref COMPILED: CompiledConfig = {
        let (_, compiled) = load_config(MARC21_JSON)
            .expect("invalid marc21.json config");
        compiled
    };
}

pub fn detect_encoding(record: &RawRecord<'_>) -> Result<Encoding, MarcError> {
    let leader = record.leader()?;
    Ok(match leader[9] as char {
        'a' => Encoding::Utf8,
        _ => Encoding::Marc8,
    })
}

pub fn to_record(encoding: &Encoding, record: &RawRecord<'_>) -> Result<Record, MarcError> {
    COMPILED.to_record(encoding, record)
}

pub fn to_raw(encoding: &Encoding, record: &Record) -> Result<OwnedRawRecord, MarcError> {
    COMPILED.to_raw(encoding, record)
}
