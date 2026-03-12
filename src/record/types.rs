use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader {
    pub status: RecordStatus,
    pub record_type: RecordType,
    pub bibliographic_level: BibliographicLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordStatus {
    New,
    Corrected,
    Deleted,
    Other(char),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordType {
    LanguageMaterial,
    NotatedMusic,
    CartographicMaterial,
    Manuscript,
    ProjectedMedium,
    Sound,
    Visual,
    ComputerFile,
    MixedMaterials,
    Other(char),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BibliographicLevel {
    Monograph,
    Serial,
    MonographicComponent,
    SerialComponent,
    Collection,
    Subunit,
    IntegratingResource,
    Other(char),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum MarcDate {
    Exact(NaiveDate),
    YearMonth { year: u16, month: u8 },
    Year(u16),
    Range { start: u16, end: u16 },
    Approximate(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    French,
    English,
    German,
    Spanish,
    Italian,
    Portuguese,
    Other(String),
}

impl From<&str> for Language {
    fn from(code: &str) -> Self {
        match code {
            "fre" | "fra" => Language::French,
            "eng" => Language::English,
            "ger" | "deu" => Language::German,
            "spa" => Language::Spanish,
            "ita" => Language::Italian,
            "por" => Language::Portuguese,
            other => Language::Other(other.to_string()),
        }
    }
}

impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::French => "fre",
            Language::English => "eng",
            Language::German => "ger",
            Language::Spanish => "spa",
            Language::Italian => "ita",
            Language::Portuguese => "por",
            Language::Other(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Country {
    France,
    UnitedStates,
    UnitedKingdom,
    Germany,
    Spain,
    Italy,
    Other(String),
}

impl From<&str> for Country {
    fn from(code: &str) -> Self {
        match code {
            "FR" | "fr " => Country::France,
            "US" | "xxu" => Country::UnitedStates,
            "UK" | "xxk" => Country::UnitedKingdom,
            "DE" | "gw " => Country::Germany,
            "ES" | "sp " => Country::Spain,
            "IT" | "it " => Country::Italy,
            other => Country::Other(other.to_string()),
        }
    }
}

impl Country {
    pub fn code(&self) -> &str {
        match self {
            Country::France => "FR",
            Country::UnitedStates => "US",
            Country::UnitedKingdom => "UK",
            Country::Germany => "DE",
            Country::Spain => "ES",
            Country::Italy => "IT",
            Country::Other(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetAudience {
    General,
    Juvenile,
    YoungAdult,
    Specialized,
    Unknown,
    Other(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Isbn {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifying: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issn {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifying: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicationFunction {
    Production,
    Publication,
    Distribution,
    Manufacture,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parallel: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<PublicationFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_type: Option<NoteType>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteType {
    General,
    Bibliography,
    Contents,
    Summary,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Agent {
    Person(Person),
    CorporateBody(CorporateBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorporateBody {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<LinkType>,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkType {
    IsPartOf,
    HasPart,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub heading_type: SubjectType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubjectType {
    Personal,
    Corporate,
    Topical,
    Geographic,
    Genre,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Classification {
    pub scheme: ClassificationScheme,
    pub number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClassificationScheme {
    Dewey,
    Lcc,
    Udc,
    Other(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_physical_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accompanying_material: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogingSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_conventions: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCallNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_library: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_control_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circulation_status: Option<String>,
}

/// Patent control number (MARC21 013).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatentNumber {
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<MarcDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<String>,
}

/// Standard technical report number (MARC21 027).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalReportNumber {
    pub value: String,
    pub canceled_or_invalid: bool,
}

/// Publisher number (MARC21 028).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublisherNumber {
    pub value: String,
    pub source: Option<String>,
    pub canceled_or_invalid: bool,
}

/// CODEN designation (MARC21 030).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coden {
    pub value: String,
    pub canceled_or_invalid: bool,
}

/// Original study number for computer files (MARC21 036).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalStudyNumber {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Government document call number (MARC21 086).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernmentDocumentNumber {
    pub value: String,
    pub canceled_or_invalid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Generic report number (MARC21 088).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportNumber {
    pub value: String,
    pub canceled_or_invalid: bool,
}

