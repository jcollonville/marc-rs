use crate::format::Encoding as MarcEncoding;
use encoding_rs::Encoding;
use unicode_normalization::UnicodeNormalization;


/// Try UTF-8 decode, then ISO-8859-1 on invalid sequence (per-record heuristic fallback).
pub fn convert_to_utf8_heuristic(data: &[u8]) -> Result<String, String> {
    let utf8 = Encoding::for_label(b"utf-8").unwrap_or(encoding_rs::UTF_8);
    let (cow, _, had_errors) = utf8.decode(data);
    if !had_errors {
        return Ok(cow.to_string());
    }
    let latin1 = Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252);
    let (cow, _, _) = latin1.decode(data);
    Ok(cow.to_string())
}

/// Convert bytes from a specific encoding to UTF-8
pub fn convert_to_utf8(data: &[u8], encoding: MarcEncoding) -> Result<String, String> {
    if encoding == MarcEncoding::Iso5426 {
        return decode_iso5426(data);
    }

    let enc = get_encoding(encoding);
    let (cow, _, had_errors) = enc.decode(data);

    if had_errors {
        return Err("Encoding conversion had errors".to_string());
    }

    Ok(cow.to_string())
}

/// Convert UTF-8 string to a specific encoding
pub fn convert_from_encoding(text: &str, encoding: MarcEncoding) -> Result<Vec<u8>, String> {
    if encoding == MarcEncoding::Iso5426 {
        return encode_iso5426(text);
    }

    let enc = get_encoding(encoding);
    let (cow, _, had_errors) = enc.encode(text);

    if had_errors {
        return Err("Encoding conversion had errors".to_string());
    }

    Ok(cow.to_vec())
}

/// Get encoding_rs::Encoding for our Encoding enum
fn get_encoding(encoding: MarcEncoding) -> &'static Encoding {
    match encoding {
        MarcEncoding::Utf8 => Encoding::for_label(b"utf-8").unwrap_or(encoding_rs::UTF_8),
        MarcEncoding::Marc8 => {
            // MARC-8 is a variant, use ISO-8859-1 as fallback
            // In a full implementation, you'd need a MARC-8 specific decoder
            Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252)
        }
        MarcEncoding::Iso8859_1 => Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252),
        MarcEncoding::Iso8859_2 => Encoding::for_label(b"iso-8859-2").unwrap(),
        MarcEncoding::Iso8859_5 => Encoding::for_label(b"iso-8859-5").unwrap(),
        MarcEncoding::Iso8859_7 => Encoding::for_label(b"iso-8859-7").unwrap(),
        MarcEncoding::Iso8859_15 => Encoding::for_label(b"iso-8859-15").unwrap(),
        MarcEncoding::Iso5426 => {
            // ISO-5426 is handled by custom functions decode_iso5426/encode_iso5426
            Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252)
        }
        MarcEncoding::Iso6937 => {
            // encoding_rs has no ISO-6937; use ISO-8859-1 as practical fallback
            Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252)
        }
        MarcEncoding::Iso5427 => {
            // encoding_rs has no ISO-5427; use ISO-8859-5 (Cyrillic) as practical fallback
            Encoding::for_label(b"iso-8859-5").unwrap()
        }
        MarcEncoding::Iso5428 => {
            // encoding_rs has no ISO-5428; use ISO-8859-7 (Greek) as practical fallback
            Encoding::for_label(b"iso-8859-7").unwrap()
        }
    }
}


use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Mapping ISO 5426 (bytes > 0x7F) to Unicode code points.
    /// Keys are raw bytes, values are Unicode codes.
    pub static ref ISO5426_CORRECT: HashMap<u32, u32> = {
        let mut m = HashMap::new();

        // --- SPECIAL CHARACTERS (0xA1 - 0xBF) ---
        m.insert(0xA1, 0x0141); // Ł
        m.insert(0xA2, 0x00D8); // Ø
        m.insert(0xA3, 0x0110); // Đ
        m.insert(0xA4, 0x00DE); // Þ
        m.insert(0xA5, 0x00C6); // Æ
        m.insert(0xA6, 0x0152); // Œ
        m.insert(0xA8, 0x00B7); // · (middle dot)
        m.insert(0xB1, 0x0142); // ł
        m.insert(0xB2, 0x00F8); // ø
        m.insert(0xB3, 0x0111); // đ
        m.insert(0xB4, 0x00FE); // þ
        m.insert(0xB5, 0x00E6); // æ
        m.insert(0xB6, 0x0153); // œ
        m.insert(0xB8, 0x0131); // ı (dotless i)
        m.insert(0xB9, 0x00A3); // £
        m.insert(0xBA, 0x00F0); // ð

        // --- SIMPLE COMBINING DIACRITICS (0xC1 - 0xCF) ---
        // Note: In Unicode they must be placed AFTER the base letter
        m.insert(0xC1, 0x0300); // Grave `
        m.insert(0xC2, 0x0301); // Acute ´
        m.insert(0xC3, 0x0302); // Circumflex ^
        m.insert(0xC4, 0x0303); // Tilde ~
        m.insert(0xC5, 0x0304); // Macron ¯
        m.insert(0xC6, 0x0306); // Breve ˘
        m.insert(0xC7, 0x0307); // Dot above ˙
        m.insert(0xC8, 0x0308); // Diaeresis ¨
        m.insert(0xC9, 0x030C); // Caron ˇ
        m.insert(0xCA, 0x030A); // Ring above ˚
        m.insert(0xCB, 0x0327); // Cedilla ¸
        m.insert(0xCC, 0x0328); // Ogonek ̨
        m.insert(0xCD, 0x0323); // Dot below ̣
        m.insert(0xCE, 0x0324); // Diaeresis below ̤
        m.insert(0xCF, 0x0313); // Comma above

        // --- ADDITIONAL DIACRITICS (0xD0 - 0xDF) ---
        m.insert(0xD0, 0x030B); // Double acute
        m.insert(0xD1, 0x0332); // Low line (macron below)
        m.insert(0xD2, 0x0325); // Ring below
        m.insert(0xD6, 0x0326); // Comma below
        
        // --- DOUBLE DIACRITICS (span two letters) ---
        m.insert(0xE1, 0x0361); // Double inverted breve (t͡s)
        m.insert(0xE2, 0x0360); // Double tilde

        m
    };

    static ref UNICODE_TO_ISO5426: HashMap<u32, u8> = {
        let mut m = HashMap::new();
        // Inverse of ISO5426_CORRECT table
        // Note: Only bytes > 0x7F are mapped here
        for (iso_byte, unicode_cp) in ISO5426_CORRECT.iter() {
            m.insert(*unicode_cp, *iso_byte as u8);
        }
        m
    };
}

fn decode_iso5426(data: &[u8]) -> Result<String, String> {
    let mut out_codes: Vec<u32> = Vec::with_capacity(data.len());
    let mut i = 0;

    while i < data.len() {
        let b = data[i];

        match b {
            // ASCII standard (0x20-0x7E)
            0x20..=0x7E => {
                out_codes.push(b as u32);
                i += 1;
            }
            // DETECT COMBINING ACCENTS (ISO-5426 combining range)
            // In ISO-5426, the accent comes BEFORE the letter.
            0xC1..=0xCF | 0xD0..=0xDF | 0xE1..=0xE8 => {
                if let Some(&accent_unicode) = ISO5426_CORRECT.get(&(b as u32)) {
                    if i + 1 < data.len() {
                        let next_byte = data[i + 1];
                        
                        // 1. Push base letter first (Unicode order)
                        let base_char = if next_byte > 0x7F {
                            *ISO5426_CORRECT.get(&(next_byte as u32)).unwrap_or(&(next_byte as u32))
                        } else {
                            next_byte as u32
                        };
                        out_codes.push(base_char);

                        // 2. Push accent after
                        out_codes.push(accent_unicode);
                        i += 2; // Consumed accent and letter
                    } else {
                        // Lone accent at end of string
                        out_codes.push(accent_unicode);
                        i += 1;
                    }
                } else {
                    out_codes.push(b as u32);
                    i += 1;
                }
            }
            // G1 special characters (Æ, Œ, ł, etc.) non-combining
            0xA1..=0xBF => {
                let cp = *ISO5426_CORRECT.get(&(b as u32)).unwrap_or(&(b as u32));
                out_codes.push(cp);
                i += 1;
            }
            // Control characters (0x00-0x1F) and other
            _ => {
                if b == 0x09 || b == 0x0A || b == 0x0D {
                    out_codes.push(b as u32);
                }
                i += 1;
            }
        }
    }

    // Convert to String and NFC normalization (crucial to merge base + accent)
    let raw_string: String = out_codes
        .into_iter()
        .filter_map(std::char::from_u32)
        .collect();

    Ok(raw_string.nfc().collect())
}

/// Encode UTF-8 string to ISO-5426 bytes


fn encode_iso5426(text: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(text.len());

    // 1. Use NFD to separate accents from base letters (e.g. 'é' -> 'e' + '\u0301')
    let nfd_text: Vec<char> = text.nfd().collect();
    let mut i = 0;

    while i < nfd_text.len() {
        let ch = nfd_text[i];
        let cp = ch as u32;

        // 2. Check if next character is a combining diacritic
        if i + 1 < nfd_text.len() {
            let next_ch = nfd_text[i + 1];
            let next_cp = next_ch as u32;

            // If next is an accent handled by ISO-5426
            if (0x0300..=0x036F).contains(&next_cp) {
                if let Some(&accent_byte) = UNICODE_TO_ISO5426.get(&next_cp) {
                    // ISO-5426 rule: write ACCENT first
                    result.push(accent_byte);

                    // Then write base letter
                    if cp <= 0x7E {
                        result.push(cp as u8);
                    } else if let Some(&base_byte) = UNICODE_TO_ISO5426.get(&cp) {
                        result.push(base_byte);
                    } else {
                        return Err(format!("Base character not supported: {}", ch));
                    }
                    
                    i += 2; // Consumed letter and its accent
                    continue;
                }
            }
        }

        // 3. Standalone characters (ASCII or special like Æ, Œ)
        if cp <= 0x7E {
            result.push(cp as u8);
        } else if let Some(&byte) = UNICODE_TO_ISO5426.get(&cp) {
            result.push(byte);
        } else {
            // Optional: replace with '?' or space instead of error
            return Err(format!("Character not supported in ISO-5426: {}", ch));
        }

        i += 1;
    }

    Ok(result)
}

/// Map ISO-5426 special characters (0x80-0x9F) to Unicode
/// This is a partial mapping - a full implementation would include all 76 characters
fn map_iso5426_special(byte: u8) -> Option<char> {
    match byte {
        // Common ISO-5426 characters mapped to Unicode
        // This is a simplified mapping - extend as needed
        0x80..=0x9F => {
            // For now, use ISO-8859-1 as fallback for most characters
            // A full implementation would have a complete mapping table
            let iso8859_1_enc = Encoding::for_label(b"iso-8859-1").unwrap();
            let byte_array = [byte];
            let (cow, _, _) = iso8859_1_enc.decode(&byte_array);
            let decoded_str = cow.to_string();
            decoded_str.chars().next()
        }
        _ => None,
    }
}

/// Map Unicode character to ISO-5426 byte
fn map_unicode_to_iso5426(ch: char) -> Option<u8> {
    // Simplified mapping - extend with full ISO-5426 table as needed
    // For now, try ISO-8859-1 encoding first
    let iso8859_1_enc = Encoding::for_label(b"iso-8859-1").unwrap();
    let ch_str = ch.to_string();
    let (cow, _, had_errors) = iso8859_1_enc.encode(&ch_str);
    let encoded_bytes = cow.to_vec();
    if !had_errors && encoded_bytes.len() == 1 {
        Some(encoded_bytes[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_keeps_valid_utf8() {
        let data = "Bonjour été".as_bytes();
        let decoded = convert_to_utf8_heuristic(data).expect("valid UTF-8 should decode");
        assert_eq!(decoded, "Bonjour été");
    }

    #[test]
    fn heuristic_falls_back_to_latin1() {
        let data = [0xE9u8];
        let decoded = convert_to_utf8_heuristic(&data).expect("latin1 fallback should decode");
        assert_eq!(decoded, "é");
    }

    #[test]
    fn convert_to_utf8_utf8_reports_invalid_data() {
        let data = [0xFFu8];
        let result = convert_to_utf8(&data, MarcEncoding::Utf8);
        assert!(result.is_err());
    }

    #[test]
    fn decode_iso5426_reorders_combining_accent() {
        let data = [0xC2u8, b'e'];
        let decoded = convert_to_utf8(&data, MarcEncoding::Iso5426).expect("ISO-5426 should decode");
        assert_eq!(decoded, "é");
    }

    #[test]
    fn encode_iso5426_places_accent_before_base_letter() {
        let encoded = convert_from_encoding("é", MarcEncoding::Iso5426).expect("ISO-5426 should encode");
        assert_eq!(encoded, vec![0xC2, b'e']);
    }

    #[test]
    fn iso5426_roundtrip_with_mixed_characters() {
        let source = "Łódź";
        let encoded = convert_from_encoding(source, MarcEncoding::Iso5426).expect("should encode");
        let decoded = convert_to_utf8(&encoded, MarcEncoding::Iso5426).expect("should decode");
        assert_eq!(decoded, source);
    }

    #[test]
    fn encode_iso5426_rejects_unsupported_character() {
        let result = convert_from_encoding("☃", MarcEncoding::Iso5426);
        assert!(result.is_err());
    }

    #[test]
    fn helper_maps_are_consistent_for_basic_values() {
        assert_eq!(map_unicode_to_iso5426('é'), Some(0xE9));
        assert_eq!(map_iso5426_special(0x80), Some('€'));
        assert_eq!(map_iso5426_special(0x7F), None);
    }

    #[test]
    fn iso5427_roundtrip_uses_cyrillic_fallback() {
        let source = "Ж";
        let encoded = convert_from_encoding(source, MarcEncoding::Iso5427).expect("should encode");
        let decoded = convert_to_utf8(&encoded, MarcEncoding::Iso5427).expect("should decode");
        assert_eq!(decoded, source);
    }

    #[test]
    fn iso5428_roundtrip_uses_greek_fallback() {
        let source = "Ω";
        let encoded = convert_from_encoding(source, MarcEncoding::Iso5428).expect("should encode");
        let decoded = convert_to_utf8(&encoded, MarcEncoding::Iso5428).expect("should decode");
        assert_eq!(decoded, source);
    }
}
