use marc_rs::*;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <marc-file> [output-format]", args[0]);
        eprintln!("  output-format: plain, json, marc-xml, marc, or unimarc (default: plain). All output is UTF-8.");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_format = args.get(2).map(|s| s.as_str()).unwrap_or("plain");

    match view_marc_file(file_path, output_format) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn view_marc_file(
    file_path: &str,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let format_encoding = detect_format_encoding(&buffer)?;

    let records = parse(&buffer, format_encoding)?;

    if records.is_empty() {
        eprintln!("No records found in file.");
        return Ok(());
    }

    match output_format.to_lowercase().as_str() {
        "plain" => {
            println!("File: {}", file_path);
            println!("Format: {:?} (output UTF-8)", format_encoding.format);
            println!("{}", "=".repeat(80));
            println!("Found {} record(s)\n", records.len());

            for (idx, record) in records.iter().enumerate() {
                if records.len() > 1 {
                    println!("{}", "─".repeat(80));
                    println!("Record #{}", idx + 1);
                    println!("{}", "─".repeat(80));
                }
                display_record(record, format_encoding.format);
                if idx < records.len() - 1 {
                    println!();
                }
            }
        }
        "json" => {
            let json = serde_json::to_string_pretty(&records)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
            println!("{}", json);
        }
        "marc-xml" => {
            let xml_format = FormatEncoding::marc_xml();
            let xml = helpers::to_string_many(&records, xml_format)
                .map_err(|e| format!("Failed to serialize to XML: {}", e))?;
            println!("{}", xml);
        }
        "marc" | "marc21" => {
            let marc_format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Utf8);
            let bytes = helpers::to_vec_many(&records, marc_format)
                .map_err(|e| format!("Failed to serialize to MARC21: {}", e))?;
            std::io::stdout()
                .write_all(&bytes)
                .map_err(|e| format!("Failed to write MARC21 output: {}", e))?;
        }
        "unimarc" => {
            let unimarc_format = FormatEncoding::new(MarcFormat::Unimarc, Encoding::Utf8);
            let bytes = helpers::to_vec_many(&records, unimarc_format)
                .map_err(|e| format!("Failed to serialize to UNIMARC: {}", e))?;
            std::io::stdout()
                .write_all(&bytes)
                .map_err(|e| format!("Failed to write UNIMARC output: {}", e))?;
        }
        _ => {
            return Err(format!(
                "Unknown output format: {}. Use: plain, json, marc-xml, marc, or unimarc",
                output_format
            )
            .into());
        }
    }

    Ok(())
}

fn detect_format_encoding(buffer: &[u8]) -> Result<FormatEncoding, String> {
    let format = if buffer.starts_with(b"<?xml")
        || buffer.starts_with(b"<record")
        || buffer.starts_with(b"<collection")
    {
        MarcFormat::MarcXml
    } else if buffer.len() >= 24 {
        MarcFormat::Marc21
    } else {
        return Err("Cannot detect format from file content.".to_string());
    };
    Ok(FormatEncoding::new(format, Encoding::Utf8))
}

fn display_record(record: &Record, format: MarcFormat) {
    println!("LEADER");
    println!("  Record Length: {}", record.leader.record_length);
    println!("  Status: {}", record.leader.record_status);
    println!("  Type: {}", record.leader.record_type);
    println!(
        "  Bibliographic Level: {}",
        record.leader.bibliographic_level
    );
    println!("  Type of Control: {}", record.leader.type_of_control);
    println!(
        "  Character Coding Scheme: {}",
        record.leader.character_coding_scheme
    );
    println!("  Indicator Count: {}", record.leader.indicator_count);
    println!(
        "  Subfield Code Count: {}",
        record.leader.subfield_code_count
    );
    println!("  Base Address: {}", record.leader.base_address_of_data);
    println!("  Encoding Level: {}", record.leader.encoding_level);
    println!(
        "  Descriptive Cataloging Form: {}",
        record.leader.descriptive_cataloging_form
    );
    println!();

    // Control fields
    let has_control = !record.control.is_empty() || !record.other_control.is_empty();
    if has_control {
        println!("CONTROL FIELDS");
        for c in &record.control {
            if let Some(tag) = c.tag(format) {
                println!("  {}: {}", tag, c.value());
            }
        }
        for c in &record.other_control {
            println!("  {}: {}", c.tag, c.value);
        }
        println!();
    }

    // Data fields: collect all into raw and display
    let (_, data_fields) = crate::writer::collect_raw_fields(record, format);

    if !data_fields.is_empty() {
        println!("DATA FIELDS");
        for field in &data_fields {
            print!("  {} ", field.tag);
            print!(
                "{}",
                if field.ind1 != ' ' {
                    field.ind1
                } else {
                    '_'
                }
            );
            print!(
                "{}",
                if field.ind2 != ' ' {
                    field.ind2
                } else {
                    '_'
                }
            );
            print!(" ");

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
