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
    pub qualifying: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issn {
    pub value: String,
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
    pub subtitle: Option<String>,
    #[serde(default)]
    pub parallel: Vec<String>,
    pub responsibility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub place: Option<String>,
    pub publisher: Option<String>,
    pub date: Option<MarcDate>,
    pub function: Option<PublicationFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
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
    pub forename: Option<String>,
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
    pub extent: Option<String>,
    pub other_physical_details: Option<String>,
    pub dimensions: Option<String>,
    pub accompanying_material: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogingSource {
    pub country: Option<String>,
    pub agency: Option<String>,
    pub date: Option<MarcDate>,
    pub transcription_conventions: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCallNumber {
    pub location: Option<String>,
    pub sublocation: Option<String>,
    pub call_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicLocation {
    pub uri: Option<String>,
    pub public_note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen {
    pub library: Option<String>,
    pub section: Option<String>,
    pub sub_library: Option<String>,
    pub section_code: Option<String>,
    pub level_code: Option<String>,
    pub barcode: Option<String>,
    pub call_number: Option<String>,
    pub inventory_number: Option<String>,
    pub creation_date: Option<MarcDate>,
    pub modification_date: Option<MarcDate>,
    pub loan_date: Option<MarcDate>,
    pub return_date: Option<MarcDate>,
    pub acquisition_date: Option<MarcDate>,
    pub item_type: Option<String>,
    pub record_control_number: Option<String>,
    pub document_type: Option<String>,
    pub circulation_status: Option<String>,
}

/// Patent control number (MARC21 013).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatentNumber {
    pub number: String,
    pub country: Option<String>,
    pub number_type: Option<String>,
    pub date: Option<MarcDate>,
    pub status: Option<String>,
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
    pub source: Option<String>,
}

/// Government document call number (MARC21 086).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernmentDocumentNumber {
    pub value: String,
    pub canceled_or_invalid: bool,
    pub source: Option<String>,
}

/// Generic report number (MARC21 088).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportNumber {
    pub value: String,
    pub canceled_or_invalid: bool,
}

