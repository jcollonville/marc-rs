use crate::datatypes::{PersonalNameType, OrganizationNameType};
use crate::record::{ControlField, DataField};

/// Identifies which block a tag belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockId {
    Identification,              // 0xx
    CodedInformation,            // 1xx
    Description,                 // 2xx
    Notes,                       // 3xx
    Links,                       // 4xx
    AssociatedTitles,            // 5xx
    SubjectAnalysis,             // 6xx
    IntellectualResponsibility,  // 7xx
    InternationalUse,            // 8xx
    LocalUse,                    // 9xx
}

/// How to extract data from subfields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    // Simple types
    SimpleString, // extract $a as string
    StringList,   // collect all $a into Vec<String>
    // Object types (parsed by dedicated datatype)
    Isbn,
    TitleStatement,
    PhysicalDescription,
    EditionStatement,
    Publication,
    SeriesStatement,
    PersonalName,
    CorporateName,
    MeetingName,
    UniformTitle,
    DeweyClassification,
    Language,
    LinkingEntry,
    SubjectEntry,
    NoteEntry,
    Specimen,
    // Generic fallback
    GenericDataField,
}

/// Maps a subfield code to a named property.
#[derive(Debug, Clone)]
pub struct SubfieldMapping {
    pub code: char,
    pub name: &'static str,
    pub repeatable: bool,
}

/// Describes one MARC tag in a format.
#[derive(Debug, Clone)]
pub struct TagDescriptor {
    pub tag: &'static str,
    pub block: BlockId,
    /// Field name in the target block struct (snake_case).
    pub field: &'static str,
    pub field_type: FieldType,
    /// True for 00X control fields.
    pub is_control: bool,
    pub subfield_map: &'static [SubfieldMapping],
}

/// Trait implemented by each semantic format (UNIMARC, MARC21, ...).
pub trait FormatDescriptor: Send + Sync {
    fn name(&self) -> &str;
    fn tag_descriptor(&self, tag: &str) -> Option<&'static TagDescriptor>;
    fn field_type_to_tag(&self, field_type: FieldType) -> Vec<&'static TagDescriptor>;
    fn field_to_tag(&self, block: BlockId, field: &str) -> Option<&'static TagDescriptor>;
    fn all_descriptors(&self) -> &'static [TagDescriptor];

    fn personal_name_type_from_ind1(&self, ind1: char) -> PersonalNameType;
    fn personal_name_type_to_ind1(&self, name_type: &PersonalNameType) -> char;
    fn organization_name_type_from_ind1(&self, ind1: char) -> OrganizationNameType;
    fn organization_name_type_to_ind1(&self, name_type: &OrganizationNameType) -> char;

    /// Optional hook: map an unknown control field into a typed structure.
    #[allow(unused_variables)]
    fn decode_control(
        &self,
        tag: &str,
        value: &str,
    ) -> Option<ControlField> {
        None
    }

    /// Optional hook: map an unknown data field into a typed structure.
    #[allow(unused_variables)]
    fn decode_data(
        &self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
    ) -> Option<DataField> {
        None
    }
}

pub mod unimarc;
pub mod marc21;

pub use marc21::MARC21_FORMAT;
pub use unimarc::UNIMARC_FORMAT;

