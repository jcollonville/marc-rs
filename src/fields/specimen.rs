//! Specimen (item/copy) fields — 995 (French norm) and 952 (Koha).
//! Each physical copy is one repetition of the field; the parser collects all 995/952 into `record.specimens`.

use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// One specimen (item/copy) — one 995 or 952 field.
/// Common subfields: $a library, $b section, $f barcode, $k call number, $v document type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Specimen {
    /// Original tag: "995" (French norm) or "952" (Koha).
    pub tag: String,
    pub ind1: char,
    pub ind2: char,
    /// $a — Library (site code)
    pub library: Option<String>,
    /// $b — Section / Location
    pub section: Option<String>,
    /// $f — Barcode
    pub barcode: Option<String>,
    /// $k — Call number
    pub call_number: Option<String>,
    /// $v — Document type (e.g. loanable, reference)
    pub document_type: Option<String>,
    pub other_subfields: Vec<(char, String)>,
}

impl Specimen {
    const KNOWN_CODES: [char; 5] = ['a', 'b', 'f', 'k', 'v'];

    /// Parse one 995 or 952 field into a Specimen. Every occurrence is one specimen.
    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        _format: MarcFormat,
    ) -> Option<Self> {
        match tag {
            "995" | "952" => Some(Self {
                tag: tag.to_string(),
                ind1,
                ind2,
                library: get_subfield(subfields, 'a'),
                section: get_subfield(subfields, 'b'),
                barcode: get_subfield(subfields, 'f'),
                call_number: get_subfield(subfields, 'k'),
                document_type: get_subfield(subfields, 'v'),
                other_subfields: get_remaining_subfields(subfields, &Self::KNOWN_CODES),
            }),
            _ => None,
        }
    }

    fn to_subfields(&self) -> Vec<(char, String)> {
        let mut out = Vec::new();
        push_subfield(&mut out, 'a', &self.library);
        push_subfield(&mut out, 'b', &self.section);
        push_subfield(&mut out, 'f', &self.barcode);
        push_subfield(&mut out, 'k', &self.call_number);
        push_subfield(&mut out, 'v', &self.document_type);
        out.extend(self.other_subfields.clone());
        out
    }

    pub fn to_raw(&self, _format: MarcFormat) -> DataField {
        to_data_field(&self.tag, self.ind1, self.ind2, self.to_subfields())
    }
}
