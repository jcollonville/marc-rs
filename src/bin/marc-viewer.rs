use marc_rs::*;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <marc-file> [output-format] [input-format]",
            args[0]
        );
        eprintln!("  output-format: plain, json, marc-xml, marc, or unimarc (default: plain). All output is UTF-8.");
        eprintln!("  input-format:  marc21, unimarc, or marcxml (default: auto-detect).");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_format = args.get(2).map(|s| s.as_str()).unwrap_or("plain");
    let forced_input: Option<MarcFormat> = args.get(3).map(|s| match s.to_lowercase().as_str() {
        "marc21" | "marc" => MarcFormat::Marc21,
        "unimarc" => MarcFormat::Unimarc,
        "marcxml" | "marc-xml" | "xml" => MarcFormat::MarcXml,
        _ => MarcFormat::from(s.as_str()),
    });

    match view_marc_file(file_path, output_format, forced_input) {
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
    forced_input: Option<MarcFormat>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_size = buffer.len();
    let result = parse_auto(&buffer, forced_input)?;
    let records = result.records;
    let detected_format = result.format;
    let semantic_format = result.semantic_format;

    if records.is_empty() {
        eprintln!("No records found in file.");
        return Ok(());
    }

    match output_format.to_lowercase().as_str() {
        "plain" => {
            println!("{}", "=".repeat(80));
            println!("FILE INFO");
            println!("{}", "=".repeat(80));
            println!("  File:             {}", file_path);
            println!("  Size:             {}", format_size(file_size));
            println!("  Container format: {:?}", detected_format);
            if detected_format == MarcFormat::MarcXml {
                println!("  Semantic format:  {:?}", semantic_format);
            }
            let encoding = records.first().map(|r| r.leader().character_coding_scheme);
            if let Some(enc) = encoding {
                let enc_label = match enc {
                    marc_rs::CharacterCodingScheme::Utf8 => "UTF-8",
                    marc_rs::CharacterCodingScheme::Marc8OrUnspecified => "MARC-8 / unspecified",
                    marc_rs::CharacterCodingScheme::Unknown(c) => {
                        // leaking is fine for a CLI one-shot display
                        Box::leak(format!("Unknown ('{}')", c).into_boxed_str())
                    }
                };
                println!("  Encoding (leader): {}", enc_label);
            }
            println!("  Records:          {}", records.len());
            println!("{}", "=".repeat(80));
            println!();

            for (idx, record) in records.iter().enumerate() {
                if records.len() > 1 {
                    println!("{}", "─".repeat(80));
                    println!("Record #{}", idx + 1);
                    println!("{}", "─".repeat(80));
                }
                display_record(record, semantic_format);
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

fn display_record(record: &Record, format: MarcFormat) {
    let leader = record.leader();
    println!("LEADER");
    println!("  Record Length: {}", leader.record_length);
    println!("  Status: {}", leader.record_status);
    println!("  Type: {}", leader.record_type);
    println!(
        "  Bibliographic Level: {}",
        leader.bibliographic_level
    );
    println!("  Type of Control: {}", leader.type_of_control);
    println!(
        "  Character Coding Scheme: {}",
        leader.character_coding_scheme
    );
    println!("  Indicator Count: {}", leader.indicator_count);
    println!(
        "  Subfield Code Count: {}",
        leader.subfield_code_count
    );
    println!("  Base Address: {}", leader.base_address_of_data);
    println!("  Encoding Level: {}", leader.encoding_level);
    println!(
        "  Descriptive Cataloging Form: {}",
        leader.descriptive_cataloging_form
    );
    println!();

    let has_control = !record.control().is_empty() || !record.other_control().is_empty();
    if has_control {
        println!("CONTROL FIELDS");
        for c in record.control() {
            if let Some(tag) = c.tag(format) {
                println!("  {}: {}", tag, c.value());
            }
        }
        for c in record.other_control() {
            println!("  {}: {}", c.tag, c.value);
        }
        println!();
    }

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

fn format_size(bytes: usize) -> String {
    const KIB: usize = 1024;
    const MIB: usize = 1024 * KIB;
    const GIB: usize = 1024 * MIB;
    if bytes >= GIB {
        format!("{:.2} GiB ({} bytes)", bytes as f64 / GIB as f64, bytes)
    } else if bytes >= MIB {
        format!("{:.2} MiB ({} bytes)", bytes as f64 / MIB as f64, bytes)
    } else if bytes >= KIB {
        format!("{:.2} KiB ({} bytes)", bytes as f64 / KIB as f64, bytes)
    } else {
        format!("{} bytes", bytes)
    }
}
