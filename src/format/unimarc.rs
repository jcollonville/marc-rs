use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::format::engine::{load_config, CompiledConfig};
use crate::raw::{OwnedRawRecord, RawRecord};
use crate::record::Record;

use lazy_static::lazy_static;

const UNIMARC_JSON: &str = include_str!("../../resources/unimarc.json");

lazy_static! {
    static ref COMPILED: CompiledConfig = {
        let (_, compiled) = load_config(UNIMARC_JSON)
            .expect("invalid unimarc.json config");
        compiled
    };
}

pub fn detect_encoding(record: &RawRecord<'_>) -> Result<Encoding, MarcError> {
    let mut encoding = Encoding::Utf8;
    if let Ok(fields) = record.fields() {
        for f in fields {
            if let crate::raw::RawField::Data { tag, body, .. } = f {
                if &tag == b"100" {
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
                            if code == b'a' && end >= start + 28 {
                                encoding = match &body[start + 26..start + 28] {
                                    b"50" => Encoding::Utf8,
                                    b"01" => Encoding::Iso5426,
                                    b"02" => Encoding::Other(encoding_rs::ISO_8859_2),
                                    b"03" => Encoding::Other(encoding_rs::ISO_8859_3),
                                    b"05" => Encoding::Other(encoding_rs::ISO_8859_5),
                                    _ => Encoding::Utf8,
                                };
                                break;
                            }
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
    }
    Ok(encoding)
}



pub fn to_record(encoding: &Encoding, record: &RawRecord<'_>) -> Result<Record, MarcError> {
    COMPILED.to_record(encoding, record)
}

pub fn to_raw(encoding: &Encoding, record: &Record) -> Result<OwnedRawRecord, MarcError> {
    COMPILED.to_raw(encoding, record)
}
