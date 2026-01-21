/// MARC format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarcFormat {
    /// MARC21 bibliographic format
    Marc21,
    /// UNIMARC bibliographic format
    Unimarc,
    /// MARC XML format
    MarcXml,
}

impl From<&str> for MarcFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "marc21" | "marc" => MarcFormat::Marc21,
            "unimarc" => MarcFormat::Unimarc,
            "xml" => MarcFormat::MarcXml,
            _ => MarcFormat::Marc21,
        }
    }
}

/// Character encodings supported by MARC formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// UTF-8 encoding
    Utf8,
    /// MARC-8 encoding (MARC21's default)
    Marc8,
    /// ISO 8859-1 (Latin-1)
    Iso8859_1,
    /// ISO 8859-2 (Latin-2)
    Iso8859_2,
    /// ISO 8859-5 (Cyrillic)
    Iso8859_5,
    /// ISO 8859-7 (Greek)
    Iso8859_7,
    /// ISO 8859-15 (Latin-9)
    Iso8859_15,
    /// ISO 5426 (Extension of the Latin alphabet for bibliographic information interchange)
    Iso5426,
}

impl From<&str> for Encoding {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "utf8" | "utf-8" => Encoding::Utf8,
            "marc8" | "marc-8" => Encoding::Marc8,
            "iso8859-1" | "latin1" | "latin-1" => Encoding::Iso8859_1,
            "iso8859-2" | "latin2" | "latin-2" => Encoding::Iso8859_2,
            "iso8859-5" => Encoding::Iso8859_5,
            "iso8859-7" => Encoding::Iso8859_7,
            "iso8859-15" | "latin9" | "latin-9" => Encoding::Iso8859_15,
            "iso5426" | "iso-5426" => Encoding::Iso5426,
            _ => Encoding::Utf8,
        }
    }
}

/// Combination of format and encoding
#[derive(Debug, Clone, Copy)]
pub struct FormatEncoding {
    pub format: MarcFormat,
    pub encoding: Encoding,
}

impl FormatEncoding {
    /// Create a new FormatEncoding
    pub fn new(format: MarcFormat, encoding: Encoding) -> Self {
        Self { format, encoding }
    }

    /// Default MARC21 with MARC-8 encoding
    pub fn marc21_default() -> Self {
        Self {
            format: MarcFormat::Marc21,
            encoding: Encoding::Marc8,
        }
    }

    /// Default UNIMARC with UTF-8 encoding
    pub fn unimarc_default() -> Self {
        Self {
            format: MarcFormat::Unimarc,
            encoding: Encoding::Utf8,
        }
    }

    /// MARC XML with UTF-8 encoding
    pub fn marc_xml() -> Self {
        Self {
            format: MarcFormat::MarcXml,
            encoding: Encoding::Utf8,
        }
    }
}


