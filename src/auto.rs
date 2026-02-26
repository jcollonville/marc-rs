use crate::format::MarcFormat;
use crate::parser::{ParseError, ParseResult};

/// Parse MARC records with automatic format detection (convenience wrapper).
/// See [`crate::parser::parse_auto`] for full documentation.
pub fn parse_binary_auto(
    data: &[u8],
    _forced_format: Option<MarcFormat>,
) -> Result<ParseResult, ParseError> {
    crate::parser::parse_auto(data, _forced_format)
}
