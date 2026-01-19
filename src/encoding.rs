use crate::format::Encoding as MarcEncoding;
use encoding_rs::Encoding;

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
            // This should never be called, but kept for consistency
            Encoding::for_label(b"iso-8859-1").unwrap_or(encoding_rs::WINDOWS_1252)
        }
    }
}

/// Decode ISO-5426 bytes to UTF-8 string
/// ISO-5426 is compatible with ISO-8859-1 for most characters (0x20-0x7E, 0xA0-0xFF)
/// Some special characters in the 0x80-0x9F range need special handling
fn decode_iso5426(data: &[u8]) -> Result<String, String> {
    let mut result = String::with_capacity(data.len());

    for &byte in data {
        match byte {
            // ASCII printable characters (0x20-0x7E) - same as ISO-8859-1
            0x20..=0x7E => {
                result.push(byte as char);
            }
            // Control characters (0x00-0x1F) - keep as is or skip
            0x00..=0x1F => {
                // Skip control characters or convert to space
                if byte == 0x09 || byte == 0x0A || byte == 0x0D {
                    result.push(byte as char);
                }
            }
            // DEL character (0x7F)
            0x7F => {
                // Skip or replace with space
            }
            // ISO-5426 special range (0x80-0x9F) - map to Unicode equivalents
            0x80..=0x9F => {
                if let Some(ch) = map_iso5426_special(byte) {
                    result.push(ch);
                } else {
                    // Fallback: use replacement character
                    result.push('\u{FFFD}');
                }
            }
            // High range (0xA0-0xFF) - same as ISO-8859-1
            0xA0..=0xFF => {
                // Use ISO-8859-1 mapping for this range
                let iso8859_1_enc = Encoding::for_label(b"iso-8859-1").unwrap();
                let byte_array = [byte];
                let (cow, _, _) = iso8859_1_enc.decode(&byte_array);
                let decoded_str = cow.to_string();
                result.push_str(&decoded_str);
            }
        }
    }

    Ok(result)
}

/// Encode UTF-8 string to ISO-5426 bytes
fn encode_iso5426(text: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(text.len());

    for ch in text.chars() {
        let code_point = ch as u32;

        match code_point {
            // ASCII printable (0x20-0x7E)
            0x20..=0x7E => {
                result.push(code_point as u8);
            }
            // Control characters
            0x00..=0x1F => {
                if code_point == 0x09 || code_point == 0x0A || code_point == 0x0D {
                    result.push(code_point as u8);
                }
            }
            // Try to map to ISO-5426 special range first
            _ => {
                if let Some(byte) = map_unicode_to_iso5426(ch) {
                    result.push(byte);
                } else {
                    // Fallback: use ISO-8859-1 encoding
                    let iso8859_1_enc = Encoding::for_label(b"iso-8859-1").unwrap();
                    let ch_str = ch.to_string();
                    let (cow, _, had_errors) = iso8859_1_enc.encode(&ch_str);
                    let encoded_bytes = cow.to_vec();
                    if had_errors || encoded_bytes.is_empty() {
                        return Err(format!("Cannot encode character '{}' to ISO-5426", ch));
                    }
                    result.extend_from_slice(&encoded_bytes);
                }
            }
        }
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
