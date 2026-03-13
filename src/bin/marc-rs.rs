use std::io;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use marc_rs::{BinaryWriter, Encoding, MarcError, MarcFormat, MarcReader, XmlWriter};

/// MARC21 / UNIMARC reader and converter.
///
/// Input format (binary ISO2709 or MARC-XML) is auto-detected.
#[derive(Parser)]
#[command(name = "marc-rs", version, about, long_about = None)]
struct Args {
    /// Path to a MARC file (binary ISO2709 or MARC-XML)
    #[arg(value_name = "MARC-FILE")]
    path: PathBuf,

    /// Output format: json | fields | xml | marc21-<enc> | unimarc-<enc>
    #[arg(value_name = "FORMAT", default_value = "fields")]
    format: String,

    /// Force input encoding (overrides the encoding declared in the record)
    #[arg(long, value_name = "ENC")]
    encoding: Option<EncodingArg>,
}

#[derive(Clone, ValueEnum)]
enum EncodingArg {
    Utf8,
    Marc8,
    Iso5426,
    #[value(name = "iso8859-2")]
    Iso8859_2,
    #[value(name = "iso8859-3")]
    Iso8859_3,
    #[value(name = "iso8859-5")]
    Iso8859_5,
}

impl From<EncodingArg> for Encoding {
    fn from(arg: EncodingArg) -> Self {
        match arg {
            EncodingArg::Utf8 => Encoding::Utf8,
            EncodingArg::Marc8 => Encoding::Marc8,
            EncodingArg::Iso5426 => Encoding::Iso5426,
            EncodingArg::Iso8859_2 => Encoding::Other(encoding_rs::ISO_8859_2),
            EncodingArg::Iso8859_3 => Encoding::Other(encoding_rs::ISO_8859_3),
            EncodingArg::Iso8859_5 => Encoding::Other(encoding_rs::ISO_8859_5),
        }
    }
}

fn main() -> Result<(), MarcError> {
    let args = Args::parse();

    let mut reader = MarcReader::from_file(&args.path)?;
    if let Some(enc) = args.encoding {
        reader = reader.with_encoding(enc.into());
    }

    match args.format.as_str() {
        "json" => dump_as_json(reader),
        "fields" => dump_fields(&reader),
        "xml" => dump_as_xml(&reader),
        other if other.starts_with("marc21-") || other.starts_with("unimarc-") => {
            dump_as_iso2709(reader, other)
        }
        _ => dump_fields(&reader),
    }
}

fn dump_as_json(reader: MarcReader) -> Result<(), MarcError> {
    let records = reader.into_records()?;
    let mut first = true;
    print!("[");
    for record in &records {
        if !first {
            print!(",\n");
        }
        first = false;
        let json = serde_json::to_string_pretty(record)
            .map_err(|_| MarcError::InvalidRecord("serde error"))?;
        print!("{json}");
    }
    println!("]");
    Ok(())
}

fn dump_fields(reader: &MarcReader) -> Result<(), MarcError> {
    for (idx, view) in reader.iter().enumerate() {
        let view = view?;
        println!("Record {} ({:?}):", idx + 1, MarcFormat::detect(view.as_raw(), reader.encoding_override())?);
        println!("{view}");
        println!();
    }
    Ok(())
}

fn dump_as_xml(reader: &MarcReader) -> Result<(), MarcError> {
    let stdout = io::stdout();
    let mut writer = XmlWriter::new(stdout.lock());
    writer.start_collection()?;

    for view in reader.iter() {
        let view = view?;
        writer.write_raw(view.as_raw(), reader.encoding_override())?;
    }

    writer.end_collection()?;
    writer.flush()?;
    Ok(())
}

fn dump_as_iso2709(reader: MarcReader, spec: &str) -> Result<(), MarcError> {
    let target = parse_target_format(spec)?;
    let mut records = reader.into_records()?;
    let stdout = io::stdout();
    let mut writer = BinaryWriter::new(stdout.lock());
    for mut record in records {
        writer.write_record(&target, &mut record)?;
    }
    writer.flush()?;
    Ok(())
}

fn parse_target_format(spec: &str) -> Result<MarcFormat, MarcError> {
    if let Some(enc) = spec.strip_prefix("marc21-") {
        return Ok(MarcFormat::Marc21(parse_marc21_encoding(enc)?));
    }
    if let Some(enc) = spec.strip_prefix("unimarc-") {
        return Ok(MarcFormat::Unimarc(parse_unimarc_encoding(enc)?));
    }
    Err(MarcError::InvalidRecord("invalid output-format"))
}

fn parse_marc21_encoding(enc: &str) -> Result<Encoding, MarcError> {
    match enc {
        "utf8" => Ok(Encoding::Utf8),
        "marc8" => Ok(Encoding::Marc8),
        _ => Err(MarcError::InvalidRecord("unsupported marc21 encoding")),
    }
}

fn parse_unimarc_encoding(enc: &str) -> Result<Encoding, MarcError> {
    match enc {
        "utf8" => Ok(Encoding::Utf8),
        "iso5426" => Ok(Encoding::Iso5426),
        "iso8859-2" => Ok(Encoding::Other(encoding_rs::ISO_8859_2)),
        "iso8859-3" => Ok(Encoding::Other(encoding_rs::ISO_8859_3)),
        "iso8859-5" => Ok(Encoding::Other(encoding_rs::ISO_8859_5)),
        _ => Err(MarcError::InvalidRecord("unsupported unimarc encoding")),
    }
}
