//! MARC Leader position enums — named codes instead of raw char/u8.

use serde::{Deserialize, Serialize};

/// Leader position 5 — Record status (relationship to file).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordStatus {
    IncreaseInEncodingLevel,
    CorrectedOrRevised,
    Deleted,
    New,
    IncreaseInEncodingLevelPrepublication,
    Obsolete,
    Snapshot,
    Unknown(char),
}

impl From<char> for RecordStatus {
    fn from(c: char) -> Self {
        match c {
            'a' => RecordStatus::IncreaseInEncodingLevel,
            'c' => RecordStatus::CorrectedOrRevised,
            'd' => RecordStatus::Deleted,
            'n' => RecordStatus::New,
            'p' => RecordStatus::IncreaseInEncodingLevelPrepublication,
            'o' => RecordStatus::Obsolete,
            's' => RecordStatus::Snapshot,
            _ => RecordStatus::Unknown(c),
        }
    }
}

impl From<RecordStatus> for char {
    fn from(s: RecordStatus) -> char {
        match s {
            RecordStatus::IncreaseInEncodingLevel => 'a',
            RecordStatus::CorrectedOrRevised => 'c',
            RecordStatus::Deleted => 'd',
            RecordStatus::New => 'n',
            RecordStatus::IncreaseInEncodingLevelPrepublication => 'p',
            RecordStatus::Obsolete => 'o',
            RecordStatus::Snapshot => 's',
            RecordStatus::Unknown(c) => c,
        }
    }
}

/// Leader position 6 — Type of record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
    LanguageMaterial,
    NotatedMusic,
    ManuscriptNotatedMusic,
    CartographicMaterial,
    ManuscriptCartographic,
    ProjectedMedium,
    ComputerFile,
    Kit,
    MixedMaterial,
    NonmusicalSoundRecording,
    MusicalSoundRecording,
    TwoDimensionalNonprojectableGraphic,
    Serial,
    ManuscriptLanguage,
    TactileMaterial,
    Unspecified,
    Unknown(char),
}

impl From<char> for RecordType {
    fn from(c: char) -> Self {
        match c {
            'a' => RecordType::LanguageMaterial,
            'c' => RecordType::NotatedMusic,
            'd' => RecordType::ManuscriptNotatedMusic,
            'e' => RecordType::CartographicMaterial,
            'f' => RecordType::ManuscriptCartographic,
            'g' => RecordType::ProjectedMedium,
            'i' => RecordType::NonmusicalSoundRecording,
            'j' => RecordType::MusicalSoundRecording,
            'k' => RecordType::TwoDimensionalNonprojectableGraphic,
            'm' => RecordType::ComputerFile,
            'o' => RecordType::Kit,
            'p' => RecordType::MixedMaterial,
            'r' => RecordType::TactileMaterial,
            's' => RecordType::Serial,
            't' => RecordType::ManuscriptLanguage,
            'u' => RecordType::Unspecified,
            _ => RecordType::Unknown(c),
        }
    }
}

impl From<RecordType> for char {
    fn from(s: RecordType) -> char {
        match s {
            RecordType::LanguageMaterial => 'a',
            RecordType::NotatedMusic => 'c',
            RecordType::ManuscriptNotatedMusic => 'd',
            RecordType::CartographicMaterial => 'e',
            RecordType::ManuscriptCartographic => 'f',
            RecordType::ProjectedMedium => 'g',
            RecordType::NonmusicalSoundRecording => 'i',
            RecordType::MusicalSoundRecording => 'j',
            RecordType::TwoDimensionalNonprojectableGraphic => 'k',
            RecordType::ComputerFile => 'm',
            RecordType::Kit => 'o',
            RecordType::MixedMaterial => 'p',
            RecordType::TactileMaterial => 'r',
            RecordType::Serial => 's',
            RecordType::ManuscriptLanguage => 't',
            RecordType::Unspecified => 'u',
            RecordType::Unknown(c) => c,
        }
    }
}

/// Leader position 7 — Bibliographic level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BibliographicLevel {
    MonographicComponentPart,
    SerialComponentPart,
    Collection,
    Subunit,
    IntegratingResource,
    Monograph,
    Serial,
    Unknown(char),
}

impl From<char> for BibliographicLevel {
    fn from(c: char) -> Self {
        match c {
            'a' => BibliographicLevel::MonographicComponentPart,
            'b' => BibliographicLevel::SerialComponentPart,
            'c' => BibliographicLevel::Collection,
            'd' => BibliographicLevel::Subunit,
            'i' => BibliographicLevel::IntegratingResource,
            'm' => BibliographicLevel::Monograph,
            's' => BibliographicLevel::Serial,
            _ => BibliographicLevel::Unknown(c),
        }
    }
}

impl From<BibliographicLevel> for char {
    fn from(s: BibliographicLevel) -> char {
        match s {
            BibliographicLevel::MonographicComponentPart => 'a',
            BibliographicLevel::SerialComponentPart => 'b',
            BibliographicLevel::Collection => 'c',
            BibliographicLevel::Subunit => 'd',
            BibliographicLevel::IntegratingResource => 'i',
            BibliographicLevel::Monograph => 'm',
            BibliographicLevel::Serial => 's',
            BibliographicLevel::Unknown(c) => c,
        }
    }
}

/// Leader position 8 — Type of control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeOfControl {
    NoSpecifiedType,
    Archival,
    Unknown(char),
}

impl From<char> for TypeOfControl {
    fn from(c: char) -> Self {
        match c {
            ' ' => TypeOfControl::NoSpecifiedType,
            'a' => TypeOfControl::Archival,
            _ => TypeOfControl::Unknown(c),
        }
    }
}

impl From<TypeOfControl> for char {
    fn from(s: TypeOfControl) -> char {
        match s {
            TypeOfControl::NoSpecifiedType => ' ',
            TypeOfControl::Archival => 'a',
            TypeOfControl::Unknown(c) => c,
        }
    }
}

/// Leader position 9 — Character coding scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterCodingScheme {
    /// MARC-8 or unspecified
    Marc8OrUnspecified,
    /// Unicode UTF-8
    Utf8,
    Unknown(char),
}

impl From<char> for CharacterCodingScheme {
    fn from(c: char) -> Self {
        match c {
            ' ' | '#' => CharacterCodingScheme::Marc8OrUnspecified,
            'a' => CharacterCodingScheme::Utf8,
            _ => CharacterCodingScheme::Unknown(c),
        }
    }
}

impl From<CharacterCodingScheme> for char {
    fn from(s: CharacterCodingScheme) -> char {
        match s {
            CharacterCodingScheme::Marc8OrUnspecified => ' ',
            CharacterCodingScheme::Utf8 => 'a',
            CharacterCodingScheme::Unknown(c) => c,
        }
    }
}

/// Leader position 17 — Encoding level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingLevel {
    Full,
    LessThanFull,
    Abridged,
    Core,
    Partial,
    Minimal,
    Prepublication,
    MinimalLevel,
    PrepublicationLevel,
    Unknown(char),
}

impl From<char> for EncodingLevel {
    fn from(c: char) -> Self {
        match c {
            ' ' => EncodingLevel::Full,
            '1' => EncodingLevel::Full,
            '2' => EncodingLevel::LessThanFull,
            '3' => EncodingLevel::Abridged,
            '4' => EncodingLevel::Core,
            '5' => EncodingLevel::Partial,
            '7' => EncodingLevel::Minimal,
            '8' => EncodingLevel::Prepublication,
            'I' => EncodingLevel::MinimalLevel,
            'J' => EncodingLevel::PrepublicationLevel,
            'K' | 'L' | 'M' => EncodingLevel::Unknown(c),
            'u' => EncodingLevel::Unknown(c),
            'z' => EncodingLevel::Unknown(c),
            _ => EncodingLevel::Unknown(c),
        }
    }
}

impl From<EncodingLevel> for char {
    fn from(s: EncodingLevel) -> char {
        match s {
            EncodingLevel::Full => ' ',
            EncodingLevel::LessThanFull => '2',
            EncodingLevel::Abridged => '3',
            EncodingLevel::Core => '4',
            EncodingLevel::Partial => '5',
            EncodingLevel::Minimal => '7',
            EncodingLevel::Prepublication => '8',
            EncodingLevel::MinimalLevel => 'I',
            EncodingLevel::PrepublicationLevel => 'J',
            EncodingLevel::Unknown(c) => c,
        }
    }
}

/// Leader position 18 — Descriptive cataloging form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptiveCatalogingForm {
    NonIsbd,
    AACR2,
    IsbdPunctuationOmitted,
    IsbdPunctuationIncluded,
    NonIsbdPunctuationOmitted,
    Unknown(char),
}

impl From<char> for DescriptiveCatalogingForm {
    fn from(c: char) -> Self {
        match c {
            ' ' => DescriptiveCatalogingForm::NonIsbd,
            'a' => DescriptiveCatalogingForm::AACR2,
            'c' => DescriptiveCatalogingForm::IsbdPunctuationOmitted,
            'i' => DescriptiveCatalogingForm::IsbdPunctuationIncluded,
            'n' => DescriptiveCatalogingForm::NonIsbdPunctuationOmitted,
            'u' => DescriptiveCatalogingForm::Unknown(c),
            _ => DescriptiveCatalogingForm::Unknown(c),
        }
    }
}

impl From<DescriptiveCatalogingForm> for char {
    fn from(s: DescriptiveCatalogingForm) -> char {
        match s {
            DescriptiveCatalogingForm::NonIsbd => ' ',
            DescriptiveCatalogingForm::AACR2 => 'a',
            DescriptiveCatalogingForm::IsbdPunctuationOmitted => 'c',
            DescriptiveCatalogingForm::IsbdPunctuationIncluded => 'i',
            DescriptiveCatalogingForm::NonIsbdPunctuationOmitted => 'n',
            DescriptiveCatalogingForm::Unknown(c) => c,
        }
    }
}

/// Leader position 19 — Multipart resource record level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultipartResourceRecordLevel {
    NotSpecifiedOrNotApplicable,
    Set,
    PartWithIndependentTitle,
    PartWithDependentTitle,
    Unknown(char),
}

impl From<char> for MultipartResourceRecordLevel {
    fn from(c: char) -> Self {
        match c {
            ' ' | 'a' => MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
            'b' => MultipartResourceRecordLevel::Set,
            'c' => MultipartResourceRecordLevel::PartWithIndependentTitle,
            'd' => MultipartResourceRecordLevel::PartWithDependentTitle,
            'u' => MultipartResourceRecordLevel::Unknown(c),
            _ => MultipartResourceRecordLevel::Unknown(c),
        }
    }
}

impl From<MultipartResourceRecordLevel> for char {
    fn from(s: MultipartResourceRecordLevel) -> char {
        match s {
            MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable => ' ',
            MultipartResourceRecordLevel::Set => 'b',
            MultipartResourceRecordLevel::PartWithIndependentTitle => 'c',
            MultipartResourceRecordLevel::PartWithDependentTitle => 'd',
            MultipartResourceRecordLevel::Unknown(c) => c,
        }
    }
}

/// Leader position 23 — Undefined (reserved).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderUndefined {
    Blank,
    Unknown(char),
}

impl From<char> for LeaderUndefined {
    fn from(c: char) -> Self {
        match c {
            ' ' => LeaderUndefined::Blank,
            _ => LeaderUndefined::Unknown(c),
        }
    }
}

impl From<LeaderUndefined> for char {
    fn from(s: LeaderUndefined) -> char {
        match s {
            LeaderUndefined::Blank => ' ',
            LeaderUndefined::Unknown(c) => c,
        }
    }
}

impl std::fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for BibliographicLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for TypeOfControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for CharacterCodingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for EncodingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for DescriptiveCatalogingForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for MultipartResourceRecordLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for LeaderUndefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// MARC leader (24 bytes) — 24-byte header for each record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leader {
    pub record_length: u16,
    pub record_status: RecordStatus,
    pub record_type: RecordType,
    pub bibliographic_level: BibliographicLevel,
    pub type_of_control: TypeOfControl,
    pub character_coding_scheme: CharacterCodingScheme,
    pub indicator_count: u8,
    pub subfield_code_count: u8,
    pub base_address_of_data: u16,
    pub encoding_level: EncodingLevel,
    pub descriptive_cataloging_form: DescriptiveCatalogingForm,
    pub multipart_resource_record_level: MultipartResourceRecordLevel,
    pub length_of_length_of_field_portion: u8,
    pub length_of_starting_character_position_portion: u8,
    pub length_of_implementation_defined_portion: u8,
    pub undefined: LeaderUndefined,
}

/// Builder for constructing `Leader` instances with sensible defaults.
///
/// Defaults to a new language-material monograph record with UTF-8 encoding.
#[derive(Debug, Clone)]
pub struct LeaderBuilder {
    leader: Leader,
}

impl Default for Leader {
    fn default() -> Self {
        Leader {
            record_length: 0,
            record_status: RecordStatus::New,
            record_type: RecordType::LanguageMaterial,
            bibliographic_level: BibliographicLevel::Monograph,
            type_of_control: TypeOfControl::NoSpecifiedType,
            character_coding_scheme: CharacterCodingScheme::Utf8,
            indicator_count: 2,
            subfield_code_count: 2,
            base_address_of_data: 0,
            encoding_level: EncodingLevel::Full,
            descriptive_cataloging_form: DescriptiveCatalogingForm::NonIsbd,
            multipart_resource_record_level: MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
            length_of_length_of_field_portion: 4,
            length_of_starting_character_position_portion: 5,
            length_of_implementation_defined_portion: 0,
            undefined: LeaderUndefined::Blank,
        }
    }
}

impl LeaderBuilder {
    pub fn new() -> Self {
        Self { leader: Leader::default() }
    }

    pub fn record_status(mut self, v: RecordStatus) -> Self {
        self.leader.record_status = v;
        self
    }

    pub fn record_type(mut self, v: RecordType) -> Self {
        self.leader.record_type = v;
        self
    }

    pub fn bibliographic_level(mut self, v: BibliographicLevel) -> Self {
        self.leader.bibliographic_level = v;
        self
    }

    pub fn type_of_control(mut self, v: TypeOfControl) -> Self {
        self.leader.type_of_control = v;
        self
    }

    pub fn character_coding_scheme(mut self, v: CharacterCodingScheme) -> Self {
        self.leader.character_coding_scheme = v;
        self
    }

    pub fn encoding_level(mut self, v: EncodingLevel) -> Self {
        self.leader.encoding_level = v;
        self
    }

    pub fn descriptive_cataloging_form(mut self, v: DescriptiveCatalogingForm) -> Self {
        self.leader.descriptive_cataloging_form = v;
        self
    }

    pub fn multipart_resource_record_level(mut self, v: MultipartResourceRecordLevel) -> Self {
        self.leader.multipart_resource_record_level = v;
        self
    }

    pub fn build(self) -> Leader {
        self.leader
    }
}

impl Default for LeaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Leader {
    pub fn builder() -> LeaderBuilder {
        LeaderBuilder::new()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 24 {
            return Err(format!("Leader must be 24 bytes, got {}", data.len()));
        }

        let record_length = parse_u16(&data[0..5])?;
        let base_address = parse_u16(&data[12..17])?;

        Ok(Leader {
            record_length,
            record_status: RecordStatus::from(data[5] as char),
            record_type: RecordType::from(data[6] as char),
            bibliographic_level: BibliographicLevel::from(data[7] as char),
            type_of_control: TypeOfControl::from(data[8] as char),
            character_coding_scheme: CharacterCodingScheme::from(data[9] as char),
            indicator_count: data[10].saturating_sub(b'0'),
            subfield_code_count: data[11].saturating_sub(b'0'),
            base_address_of_data: base_address,
            encoding_level: EncodingLevel::from(data[17] as char),
            descriptive_cataloging_form: DescriptiveCatalogingForm::from(data[18] as char),
            multipart_resource_record_level: MultipartResourceRecordLevel::from(data[19] as char),
            length_of_length_of_field_portion: data[20].saturating_sub(b'0'),
            length_of_starting_character_position_portion: data[21].saturating_sub(b'0'),
            length_of_implementation_defined_portion: data[22].saturating_sub(b'0'),
            undefined: LeaderUndefined::from(data[23] as char),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 24];
        let record_length_str = format!("{:05}", self.record_length);
        let base_address_str = format!("{:05}", self.base_address_of_data);

        bytes[0..5].copy_from_slice(record_length_str.as_bytes());
        bytes[5] = char::from(self.record_status) as u8;
        bytes[6] = char::from(self.record_type) as u8;
        bytes[7] = char::from(self.bibliographic_level) as u8;
        bytes[8] = char::from(self.type_of_control) as u8;
        bytes[9] = char::from(self.character_coding_scheme) as u8;
        bytes[10] = b'0' + self.indicator_count;
        bytes[11] = b'0' + self.subfield_code_count;
        bytes[12..17].copy_from_slice(base_address_str.as_bytes());
        bytes[17] = char::from(self.encoding_level) as u8;
        bytes[18] = char::from(self.descriptive_cataloging_form) as u8;
        bytes[19] = char::from(self.multipart_resource_record_level) as u8;
        bytes[20] = b'0' + self.length_of_length_of_field_portion;
        bytes[21] = b'0' + self.length_of_starting_character_position_portion;
        bytes[22] = b'0' + self.length_of_implementation_defined_portion;
        bytes[23] = char::from(self.undefined) as u8;

        bytes
    }
}

fn parse_u16(bytes: &[u8]) -> Result<u16, String> {
    let s = std::str::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))?;
    s.parse::<u16>()
        .map_err(|e| format!("Invalid number: {}", e))
}
