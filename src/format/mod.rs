use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::raw::{RawField, RawRecord};
use crate::record::*;

#[derive(Debug, Clone, Copy)]
pub enum MarcFormat {
    Marc21(Encoding),
    Unimarc(Encoding),
}

impl MarcFormat {
    pub fn detect(record: &RawRecord<'_>) -> Result<Self, MarcError> {
        // Very rough heuristic: presence of UNIMARC-only tags like 200 suggests UNIMARC.
        let mut has_200 = false;
        let mut has_245 = false;
        for field in record.fields()? {
            match field {
                RawField::Control { tag, .. } | RawField::Data { tag, .. } => {
                    if &tag == b"200" {
                        has_200 = true;
                    } else if &tag == b"245" {
                        has_245 = true;
                    }
                }
            }
        }
        Ok(if has_200 && !has_245 {
            MarcFormat::Unimarc(unimarc::detect_encoding(record)?)
        } else {
            MarcFormat::Marc21(marc21::detect_encoding(record)?)
        })
    }


    pub fn encoding(&self) -> &Encoding {
        match self {
            MarcFormat::Marc21(encoding) => encoding,
            MarcFormat::Unimarc(encoding) => encoding,
        }
    }

    /// Encoding to use when decoding raw data; uses override when provided.
    pub fn effective_encoding(&self, override_enc: Option<Encoding>) -> Encoding {
        override_enc.unwrap_or(*self.encoding())
    }
   

    pub fn to_record(&self, record: &RawRecord<'_>) -> Result<Record, MarcError> {
        match self {
            MarcFormat::Marc21(encoding) => marc21::to_record(encoding, record),
            MarcFormat::Unimarc(encoding) => unimarc::to_record(encoding, record),
        }
    }

    pub fn to_raw(&self, record: &Record) -> Result<Vec<u8>, MarcError> {
        match self {
            MarcFormat::Marc21(encoding) => marc21::to_raw(encoding, record),
            MarcFormat::Unimarc(encoding) => unimarc::to_raw(encoding, record),
        }
    }
}

/// How to extract a value from a raw field.
pub enum ExtractionRule {
    WholeSubfield { code: u8 },
    SubfieldSlice { code: u8, offset: usize, length: usize },
    WholeControl,
    ControlSlice { offset: usize, length: usize },
}

pub struct FieldMappingEntry {
    pub tag: [u8; 3],
    pub extraction: ExtractionRule,
    pub apply: fn(&str, &mut Record),
}

/// Helper macro to declare `FieldMappingEntry` values in a compact and readable way.
/// It supports common patterns (`set`, `push`, `init`, `last`, `last_date`) and
/// falls back to custom closures for complex logic.
#[macro_export]
macro_rules! marc_mapping {
    // Control field -> set Option<String> on a path (if non-empty after trim).
    ($tag:expr, control => set $($path:ident).+) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeControl,
            apply: |data, rec| {
                let v = data.trim();
                if !v.is_empty() {
                    rec.$($path).+ = Some(v.to_string());
                }
            },
        }
    };

    // Subfield -> set Option<String> on a path (if non-empty after trim).
    ($tag:expr, $code:expr => set $($path:ident).+) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: |data, rec| {
                let v = data.trim();
                if !v.is_empty() {
                    rec.$($path).+ = Some(v.to_string());
                }
            },
        }
    };

    // Subfield -> push String into a Vec<String> (if non-empty after trim).
    ($tag:expr, $code:expr => push $($path:ident).+) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: |data, rec| {
                let v = data.trim();
                if !v.is_empty() {
                    rec.$($path).+.push(v.to_string());
                }
            },
        }
    };

    // Subfield -> get_or_insert_with(Default::default) on an Option<T> then set a field (N-segment path).
    ($tag:expr, $code:expr => init $($opt:ident).+ -> $field:ident) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: |data, rec| {
                let v = data.trim();
                if !v.is_empty() {
                    let target = rec.$($opt).+.get_or_insert_with(Default::default);
                    target.$field = Some(v.to_string());
                }
            },
        }
    };

    // Subfield -> set String on the last element of a Vec<T> (ensure Default if empty).
    ($tag:expr, $code:expr => last $($vec:ident).+ -> $field:ident) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: |data, rec| {
                let v = data.trim();
                if v.is_empty() {
                    return;
                }
                if rec.$($vec).+.is_empty() {
                    rec.$($vec).+.push(Default::default());
                }
                if let Some(last) = rec.$($vec).+.last_mut() {
                    last.$field = Some(v.to_string());
                }
            },
        }
    };

    // Subfield -> set date (MarcDate) on the last element of a Vec<T>.
    // `parse_marc_date` must be in scope at the call site.
    ($tag:expr, $code:expr => last_date $($vec:ident).+ -> $field:ident) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: |data, rec| {
                let v = data.trim();
                if v.is_empty() {
                    return;
                }
                if rec.$($vec).+.is_empty() {
                    rec.$($vec).+.push(Default::default());
                }
                if let Some(last) = rec.$($vec).+.last_mut() {
                    last.$field = Some(parse_marc_date(v));
                }
            },
        }
    };

    // ControlSlice -> custom closure (must be before SubfieldSlice).
    ($tag:expr, control, slice $offset:expr, $len:expr => $closure:expr) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::ControlSlice {
                offset: $offset,
                length: $len,
            },
            apply: $closure,
        }
    };

    // SubfieldSlice -> custom closure.
    ($tag:expr, $code:expr, slice $offset:expr, $len:expr => $closure:expr) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::SubfieldSlice {
                code: $code,
                offset: $offset,
                length: $len,
            },
            apply: $closure,
        }
    };

    // Subfield -> custom closure.
    ($tag:expr, $code:expr => $closure:expr) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeSubfield { code: $code },
            apply: $closure,
        }
    };

    // Control -> custom closure.
    ($tag:expr, control => $closure:expr) => {
        $crate::format::FieldMappingEntry {
            tag: *$tag,
            extraction: $crate::format::ExtractionRule::WholeControl,
            apply: $closure,
        }
    };
}

/// Unified bidirectional mapping macro using push-down accumulation.
///
/// Every tag must declare both forward (import) and reverse (export).
///
/// **Auto** (auto-generates both directions from a simple accessor):
/// - `$tag, control => set $path;`
/// - `$tag, $code => set $path;`
/// - `$tag, $code => push $path;`
/// - `$tag => init $seg1.$seg2 { $code => $field, ... };`
///
/// **Paired** (single forward subfield + custom reverse):
/// - `$tag, $code => $fwd, reverse $rev;`
/// - `$tag, control => $fwd, reverse $rev;`
/// - `$tag, $code, slice $off, $len => $fwd, reverse $rev;`
/// - `$tag, control, slice $off, $len => $fwd, reverse $rev;`
///
/// **Paired group** (multiple forward subfields + custom reverse):
/// ```text
/// $tag => {
///     $code => $closure;
///     $code => last $vec -> $field;
/// } reverse $rev;
/// ```
#[macro_export]
macro_rules! marc_bimap {
    // --- terminal ---
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]) => {
        const MAPPINGS: &[$crate::format::FieldMappingEntry] = &[$($fwd,)*];
        const REVERSE_MAPPINGS: &[$crate::format::ReverseFieldMapping] = &[$($rev,)*];
    };

    // ==================== AUTO ====================

    // control => set $path
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, control => set $($path:ident).+ ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, control => set $($path).+),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping {
                tag: *$tag,
                build: |rec| rec.$($path).+.as_ref()
                    .filter(|s| !s.trim().is_empty())
                    .map(|v| vec![$crate::format::ReverseFieldData::Control(v.clone())])
                    .unwrap_or_default(),
            },]
            $($rest)*
        );
    };

    // $code => set $path
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, $code:expr => set $($path:ident).+ ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, $code => set $($path).+),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping {
                tag: *$tag,
                build: |rec| rec.$($path).+.as_ref()
                    .filter(|s| !s.trim().is_empty())
                    .map(|v| vec![$crate::format::ReverseFieldData::Data {
                        ind1: b' ', ind2: b' ',
                        subfields: vec![($code, v.clone())],
                    }])
                    .unwrap_or_default(),
            },]
            $($rest)*
        );
    };

    // $code => push $path
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, $code:expr => push $($path:ident).+ ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, $code => push $($path).+),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping {
                tag: *$tag,
                build: |rec| rec.$($path).+.iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|v| $crate::format::ReverseFieldData::Data {
                        ind1: b' ', ind2: b' ',
                        subfields: vec![($code, v.clone())],
                    })
                    .collect(),
            },]
            $($rest)*
        );
    };

    // => init seg { $code => $field, ... } (1-segment path: flat Option<T> on Record)
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr => init $seg:ident { $( $code:expr => $field:ident ),+ $(,)? } ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $( $crate::marc_mapping!($tag, $code => init $seg -> $field), )+]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping {
                tag: *$tag,
                build: |rec| {
                    let Some(target) = &rec.$seg else { return Vec::new(); };
                    let mut subfields: Vec<(u8, String)> = Vec::new();
                    $(
                        if let Some(v) = &target.$field {
                            if !v.trim().is_empty() {
                                subfields.push(($code, v.clone()));
                            }
                        }
                    )+
                    if subfields.is_empty() { Vec::new() }
                    else { vec![$crate::format::ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }] }
                },
            },]
            $($rest)*
        );
    };

    // => init seg1.seg2 { $code => $field, ... } (2-segment path)
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr => init $seg1:ident . $seg2:ident { $( $code:expr => $field:ident ),+ $(,)? } ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $( $crate::marc_mapping!($tag, $code => init $seg1.$seg2 -> $field), )+]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping {
                tag: *$tag,
                build: |rec| {
                    let Some(target) = &rec.$seg1.$seg2 else { return Vec::new(); };
                    let mut subfields: Vec<(u8, String)> = Vec::new();
                    $(
                        if let Some(v) = &target.$field {
                            if !v.trim().is_empty() {
                                subfields.push(($code, v.clone()));
                            }
                        }
                    )+
                    if subfields.is_empty() { Vec::new() }
                    else { vec![$crate::format::ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }] }
                },
            },]
            $($rest)*
        );
    };

    // ==================== PAIRED ====================

    // control, slice => $fwd, reverse $rev
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, control, slice $offset:expr, $len:expr => $fwd_fn:expr, reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, control, slice $offset, $len => $fwd_fn),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping { tag: *$tag, build: $rev_fn },]
            $($rest)*
        );
    };

    // control => $fwd, reverse $rev
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, control => $fwd_fn:expr, reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, control => $fwd_fn),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping { tag: *$tag, build: $rev_fn },]
            $($rest)*
        );
    };

    // $code, slice => $fwd, reverse $rev
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, $code:expr, slice $offset:expr, $len:expr => $fwd_fn:expr, reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, $code, slice $offset, $len => $fwd_fn),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping { tag: *$tag, build: $rev_fn },]
            $($rest)*
        );
    };

    // $code => $fwd, reverse $rev (must be AFTER set/push)
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr, $code:expr => $fwd_fn:expr, reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $crate::marc_mapping!($tag, $code => $fwd_fn),]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping { tag: *$tag, build: $rev_fn },]
            $($rest)*
        );
    };

    // ==================== PAIRED GROUP ====================

    // $tag => { forward entries... } reverse $rev ;
    (@entries fwd = [$($fwd:expr,)*] rev = [$($rev:expr,)*]
        $tag:expr => { $($group:tt)* } reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@group
            tag = $tag, gfwd = [],
            fwd = [$($fwd,)*], rev = [$($rev,)*],
            $($group)* @ end reverse $rev_fn ; $($rest)*
        );
    };

    // @group terminal
    (@group
        tag = $tag:expr, gfwd = [$($gfwd:expr,)*],
        fwd = [$($fwd:expr,)*], rev = [$($rev:expr,)*],
        @ end reverse $rev_fn:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@entries
            fwd = [$($fwd,)* $($gfwd,)*]
            rev = [$($rev,)* $crate::format::ReverseFieldMapping { tag: *$tag, build: $rev_fn },]
            $($rest)*
        );
    };

    // @group: last
    (@group
        tag = $tag:expr, gfwd = [$($gfwd:expr,)*],
        fwd = [$($fwd:expr,)*], rev = [$($rev:expr,)*],
        $code:expr => last $($vec:ident).+ -> $field:ident ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@group
            tag = $tag, gfwd = [$($gfwd,)* $crate::marc_mapping!($tag, $code => last $($vec).+ -> $field),],
            fwd = [$($fwd,)*], rev = [$($rev,)*],
            $($rest)*
        );
    };

    // @group: last_date
    (@group
        tag = $tag:expr, gfwd = [$($gfwd:expr,)*],
        fwd = [$($fwd:expr,)*], rev = [$($rev:expr,)*],
        $code:expr => last_date $($vec:ident).+ -> $field:ident ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@group
            tag = $tag, gfwd = [$($gfwd,)* $crate::marc_mapping!($tag, $code => last_date $($vec).+ -> $field),],
            fwd = [$($fwd,)*], rev = [$($rev,)*],
            $($rest)*
        );
    };

    // @group: closure (must be last @group content arm)
    (@group
        tag = $tag:expr, gfwd = [$($gfwd:expr,)*],
        fwd = [$($fwd:expr,)*], rev = [$($rev:expr,)*],
        $code:expr => $closure:expr ; $($rest:tt)*
    ) => {
        $crate::marc_bimap!(@group
            tag = $tag, gfwd = [$($gfwd,)* $crate::marc_mapping!($tag, $code => $closure),],
            fwd = [$($fwd,)*], rev = [$($rev,)*],
            $($rest)*
        );
    };

    // --- entry point (must be last) ---
    ($($all:tt)*) => {
        $crate::marc_bimap!(@entries fwd = [] rev = [] $($all)*);
    };
}

pub enum ReverseFieldData {
    Control(String),
    Data {
        ind1: u8,
        ind2: u8,
        subfields: Vec<(u8, String)>,
    },
}

pub struct ReverseFieldMapping {
    pub tag: [u8; 3],
    pub build: fn(&Record) -> Vec<ReverseFieldData>,
}

fn apply_mappings(
    encoding: &Encoding,
    record: &RawRecord<'_>,
    mappings: &[FieldMappingEntry],
) -> Result<Record, MarcError> {
    let mut out = Record {
        leader: Leader {
            status: RecordStatus::Other('n'),
            record_type: RecordType::Other('a'),
            bibliographic_level: BibliographicLevel::Other('m'),
        },
        identification: Identification::default(),
        coded: Coded::default(),
        description: Description::default(),
        notes: Notes::default(),
        links: Links::default(),
        associated_titles: AssociatedTitles::default(),
        indexing: Indexing::default(),
        responsibility: Responsibility::default(),
        international: International::default(),
        local: Local::default(),
    };

    // Leader minimal parsing for core enums.
    if let Ok(leader_bytes) = record.leader() {
        let status = leader_bytes[5] as char;
        let rtype = leader_bytes[6] as char;
        let level = leader_bytes[7] as char;
        out.leader.status = match status {
            'n' => RecordStatus::New,
            'c' => RecordStatus::Corrected,
            'd' => RecordStatus::Deleted,
            other => RecordStatus::Other(other),
        };
        out.leader.record_type = match rtype {
            'a' => RecordType::LanguageMaterial,
            'm' => RecordType::ComputerFile,
            other => RecordType::Other(other),
        };
        out.leader.bibliographic_level = match level {
            'm' => BibliographicLevel::Monograph,
            's' => BibliographicLevel::Serial,
            other => BibliographicLevel::Other(other),
        };
    }

    for raw_field in record.fields()? {
        let tag = match raw_field {
            RawField::Control { tag, .. } | RawField::Data { tag, .. } => tag,
        };
        for entry in mappings.iter().filter(|e| e.tag == tag) {
            match (&entry.extraction, &raw_field) {
                (ExtractionRule::WholeControl, RawField::Control { data, .. }) => {
                    if let Ok(text) = encoding.decode(data) {
                        (entry.apply)(&text, &mut out);
                    }
                }
                (ExtractionRule::ControlSlice { offset, length }, RawField::Control { data, .. }) => {
                    if *offset + *length <= data.len() {
                        if let Ok(text) = encoding.decode(&data[*offset..*offset + *length]) {
                            (entry.apply)(&text, &mut out);
                        }
                    }
                }
                (ExtractionRule::WholeSubfield { code }, RawField::Data { body, .. })
                | (ExtractionRule::SubfieldSlice { code, .. }, RawField::Data { body, .. }) => {
                    let mut pos = 0;
                    while pos < body.len() {
                        if body[pos] == 0x1F {
                            if pos + 1 >= body.len() {
                                break;
                            }
                            let c = body[pos + 1];
                            let start = pos + 2;
                            let mut end = start;
                            while end < body.len() && body[end] != 0x1F && body[end] != 0x1E {
                                end += 1;
                            }
                            if c == *code {
                                let slice = match &entry.extraction {
                                    ExtractionRule::WholeSubfield { .. } => &body[start..end],
                                    ExtractionRule::SubfieldSlice { offset, length, .. } => {
                                        if start + *offset + *length <= end {
                                            &body[start + *offset..start + *offset + *length]
                                        } else {
                                            &[]
                                        }
                                    }
                                    _ => &[],
                                };
                                if !slice.is_empty() {
                                    if let Ok(text) = encoding.decode(slice) {
                                        (entry.apply)(&text, &mut out);
                                    }
                                }
                            }
                            pos = end;
                        } else if body[pos] == 0x1E {
                            break;
                        } else {
                            pos += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(out)
}

fn leader_char_status(status: &RecordStatus) -> u8 {
    match status {
        RecordStatus::New => b'n',
        RecordStatus::Corrected => b'c',
        RecordStatus::Deleted => b'd',
        RecordStatus::Other(c) => *c as u8,
    }
}

fn leader_char_type(record_type: &RecordType) -> u8 {
    match record_type {
        RecordType::LanguageMaterial => b'a',
        RecordType::NotatedMusic => b'c',
        RecordType::CartographicMaterial => b'e',
        RecordType::Manuscript => b't',
        RecordType::ProjectedMedium => b'g',
        RecordType::Sound => b'i',
        RecordType::Visual => b'k',
        RecordType::ComputerFile => b'm',
        RecordType::MixedMaterials => b'p',
        RecordType::Other(c) => *c as u8,
    }
}

fn leader_char_level(level: &BibliographicLevel) -> u8 {
    match level {
        BibliographicLevel::Monograph => b'm',
        BibliographicLevel::Serial => b's',
        BibliographicLevel::MonographicComponent => b'a',
        BibliographicLevel::SerialComponent => b'b',
        BibliographicLevel::Collection => b'c',
        BibliographicLevel::Subunit => b'd',
        BibliographicLevel::IntegratingResource => b'i',
        BibliographicLevel::Other(c) => *c as u8,
    }
}

pub(crate) fn build_iso2709(
    encoding: &Encoding,
    leader: &Leader,
    fields: &[( [u8; 3], ReverseFieldData )],
) -> Result<Vec<u8>, MarcError> {
    // Build field data and directory in order.
    let mut directory: Vec<u8> = Vec::new();
    let mut field_data: Vec<u8> = Vec::new();
    let mut offset: usize = 0;

    for (tag, data) in fields {
        let mut field_bytes: Vec<u8> = Vec::new();
        match data {
            ReverseFieldData::Control(text) => {
                field_bytes.extend_from_slice(&encoding.encode(text)?);
                field_bytes.push(0x1E);
            }
            ReverseFieldData::Data { ind1, ind2, subfields } => {
                field_bytes.push(*ind1);
                field_bytes.push(*ind2);
                for (code, value) in subfields {
                    field_bytes.push(0x1F);
                    field_bytes.push(*code);
                    field_bytes.extend_from_slice(&encoding.encode(value)?);
                }
                field_bytes.push(0x1E);
            }
        }

        let length = field_bytes.len();
        directory.extend_from_slice(tag);
        directory.extend_from_slice(format!("{:0>4}", length).as_bytes());
        directory.extend_from_slice(format!("{:0>5}", offset).as_bytes());

        field_data.extend_from_slice(&field_bytes);
        offset += length;
    }

    // Directory ends with field terminator.
    directory.push(0x1E);

    let base_address = 24 + directory.len();

    // Leader template (24 bytes).
    let mut leader_bytes = [b' '; 24];
    leader_bytes[0..5].copy_from_slice(b"00000");
    leader_bytes[5] = leader_char_status(&leader.status);
    leader_bytes[6] = leader_char_type(&leader.record_type);
    leader_bytes[7] = leader_char_level(&leader.bibliographic_level);
    leader_bytes[8] = b' ';
    leader_bytes[9] = match encoding {
        Encoding::Utf8 => b'a',
        _ => b' ',
    };
    leader_bytes[10] = b'2';
    leader_bytes[11] = b'2';
    leader_bytes[12..17].copy_from_slice(format!("{:0>5}", base_address).as_bytes());
    leader_bytes[17] = b' ';
    leader_bytes[18] = b' ';
    leader_bytes[19] = b' ';
    leader_bytes[20..24].copy_from_slice(b"4500");

    let record_length = base_address + field_data.len() + 1; // + record terminator 0x1D
    if record_length > 99999 {
        return Err(MarcError::InvalidRecord("record too long for ISO2709 leader"));
    }
    leader_bytes[0..5].copy_from_slice(format!("{:0>5}", record_length).as_bytes());

    let mut out = Vec::with_capacity(record_length);
    out.extend_from_slice(&leader_bytes);
    out.extend_from_slice(&directory);
    out.extend_from_slice(&field_data);
    out.push(0x1D);
    Ok(out)
}

pub mod marc21;
pub mod unimarc;

