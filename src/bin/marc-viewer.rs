use marc_rs::*;
use serde_json;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <marc-file> [format] [encoding] [output-format]", args[0]);
        eprintln!("  format: marc21, unimarc, or xml (default: auto-detect)");
        eprintln!("  encoding: utf8, marc8, iso8859-1, etc. (default: auto-detect)");
        eprintln!("  output-format: plain, json, json_pretty, xml, marc, or unimarc (default: plain)");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let format = args.get(2).map(|s| s.as_str());
    let encoding = args.get(3).map(|s| s.as_str());
    let output_format = args.get(4).map(|s| s.as_str()).unwrap_or("plain");

    match view_marc_file(file_path, format, encoding, output_format) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn view_marc_file(
    file_path: &str,
    format: Option<&str>,
    encoding: Option<&str>,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    // Read file
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Detect or use specified format
    let format_encoding = if let Some(fmt) = format {
        parse_format_encoding(fmt, encoding)?
    } else {
        detect_format_encoding(&buffer, encoding)?
    };

    // Parse records
    let records = parse(&buffer, format_encoding)?;

    if records.is_empty() {
        eprintln!("No records found in file.");
        return Ok(());
    }

    // Output based on format
    match output_format.to_lowercase().as_str() {
        "plain" => {
            println!("File: {}", file_path);
            println!("Format: {:?}, Encoding: {:?}", format_encoding.format, format_encoding.encoding);
            println!("{}", "=".repeat(80));
            println!("Found {} record(s)\n", records.len());

            for (idx, record) in records.iter().enumerate() {
                if records.len() > 1 {
                    println!("{}", "─".repeat(80));
                    println!("Record #{}", idx + 1);
                    println!("{}", "─".repeat(80));
                }
                display_record(record);
                if idx < records.len() - 1 {
                    println!();
                }
            }
        }
        "json" => {
            let json = serde_json::to_string(&records)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
            println!("{}", json);
        }
        "json_pretty" => {
            let json = serde_json::to_string_pretty(&records)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
            println!("{}", json);
        }
        "xml" => {
            let xml_format = FormatEncoding::marc_xml();
            let xml = serde_marc::to_string_many(&records, xml_format)
                .map_err(|e| format!("Failed to serialize to XML: {}", e))?;
            println!("{}", xml);
        }
        "marc" | "marc21" => {
            let marc_format = FormatEncoding::marc21_default();
            let bytes = serde_marc::to_vec_many(&records, marc_format)
                .map_err(|e| format!("Failed to serialize to MARC21: {}", e))?;
            std::io::stdout()
                .write_all(&bytes)
                .map_err(|e| format!("Failed to write MARC21 output: {}", e))?;
        }
        "unimarc" => {
            let unimarc_format = FormatEncoding::unimarc_default();
            let bytes = serde_marc::to_vec_many(&records, unimarc_format)
                .map_err(|e| format!("Failed to serialize to UNIMARC: {}", e))?;
            std::io::stdout()
                .write_all(&bytes)
                .map_err(|e| format!("Failed to write UNIMARC output: {}", e))?;
        }
        _ => {
            return Err(format!("Unknown output format: {}. Use: plain, json, json_pretty, xml, marc, or unimarc", output_format).into());
        }
    }

    Ok(())
}

fn parse_format_encoding(
    format: &str,
    encoding: Option<&str>,
) -> Result<FormatEncoding, String> {
    let fmt = match format.to_lowercase().as_str() {
        "marc21" | "marc" => MarcFormat::Marc21,
        "unimarc" => MarcFormat::Unimarc,
        "xml" => MarcFormat::MarcXml,
        _ => return Err(format!("Unknown format: {}. Use: marc21, unimarc, or xml", format)),
    };

    let enc = if let Some(enc_str) = encoding {
        parse_encoding(enc_str)?
    } else {
        match fmt {
            MarcFormat::Marc21 => Encoding::Marc8,
            MarcFormat::Unimarc => Encoding::Utf8,
            MarcFormat::MarcXml => Encoding::Utf8,
        }
    };

    Ok(FormatEncoding::new(fmt, enc))
}

fn parse_encoding(enc_str: &str) -> Result<Encoding, String> {
    match enc_str.to_lowercase().as_str() {
        "utf8" | "utf-8" => Ok(Encoding::Utf8),
        "marc8" | "marc-8" => Ok(Encoding::Marc8),
        "iso8859-1" | "latin1" | "latin-1" => Ok(Encoding::Iso8859_1),
        "iso8859-2" | "latin2" | "latin-2" => Ok(Encoding::Iso8859_2),
        "iso8859-5" => Ok(Encoding::Iso8859_5),
        "iso8859-7" => Ok(Encoding::Iso8859_7),
        "iso8859-15" | "latin9" | "latin-9" => Ok(Encoding::Iso8859_15),
        "iso5426" | "iso-5426" => Ok(Encoding::Iso5426),
        _ => Err(format!("Unknown encoding: {}", enc_str)),
    }
}

fn detect_format_encoding(
    buffer: &[u8],
    encoding: Option<&str>,
) -> Result<FormatEncoding, String> {
    // Try to detect format
    let format = if buffer.starts_with(b"<?xml") || buffer.starts_with(b"<record") || buffer.starts_with(b"<collection") {
        MarcFormat::MarcXml
    } else if buffer.len() >= 24 {
        // Check if it looks like binary MARC
        // Try MARC21 first (most common)
        MarcFormat::Marc21
    } else {
        return Err("Cannot detect format. Please specify format explicitly.".to_string());
    };

    let enc = if let Some(enc_str) = encoding {
        parse_encoding(enc_str)?
    } else {
        match format {
            MarcFormat::Marc21 => Encoding::Marc8,
            MarcFormat::Unimarc => Encoding::Utf8,
            MarcFormat::MarcXml => Encoding::Utf8,
        }
    };

    Ok(FormatEncoding::new(format, enc))
}

fn display_record(record: &Record) {
    // Display Leader
    println!("LEADER");
    println!("  Record Length: {}", record.leader.record_length);
    println!("  Status: {}", record.leader.record_status);
    println!("  Type: {}", record.leader.record_type);
    println!("  Bibliographic Level: {}", record.leader.bibliographic_level);
    println!("  Type of Control: {}", record.leader.type_of_control);
    println!("  Character Coding Scheme: {}", record.leader.character_coding_scheme);
    println!("  Indicator Count: {}", record.leader.indicator_count);
    println!("  Subfield Code Count: {}", record.leader.subfield_code_count);
    println!("  Base Address: {}", record.leader.base_address_of_data);
    println!("  Encoding Level: {}", record.leader.encoding_level);
    println!("  Descriptive Cataloging Form: {}", record.leader.descriptive_cataloging_form);
    println!();

    // Display Control Fields
    if !record.control_fields.is_empty() {
        println!("CONTROL FIELDS");
        for field in &record.control_fields {
            println!("  {}: {}", field.tag, field.value);
        }
        println!();
    }

    // Display Data Fields
    if !record.data_fields.is_empty() {
        println!("DATA FIELDS");
        for field in &record.data_fields {
            print!("  {} ", field.tag);
            if field.ind1 != ' ' {
                print!("{}", field.ind1);
            } else {
                print!("_");
            }
            if field.ind2 != ' ' {
                print!("{}", field.ind2);
            } else {
                print!("_");
            }
            print!(" ");

            // Display subfields
            let mut first = true;
            for subfield in &field.subfields {
                if !first {
                    print!(" ");
                }
                print!("${}{}", subfield.code, subfield.value);
                first = false;
            }
            println!();
        }
    } else {
        println!("DATA FIELDS: (none)");
    }
}
