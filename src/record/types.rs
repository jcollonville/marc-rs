use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use marc_rs_derive::MarcPaths;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leader {
    pub status: RecordStatus,
    pub record_type: RecordType,
    pub bibliographic_level: BibliographicLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_cataloging_form: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordStatus {
    #[default]
    New,
    Corrected,
    Deleted,
    Other(char),
}



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordType {
    #[default]
    LanguageMaterial,
    PrintedText,
    ManuscriptText,
    NotatedMusic,
    ManuscriptMusic,
    PrintedCartographic,
    ManuscriptCartographic,
    ProjectedOrVideo,
    NonMusicalSound,
    MusicalSound,
    GraphicTwoDimensional,
    ElectronicResource,
    MultimediaKit,
    MixedMaterials,
    ObjectThreeDimensional,
    Other(char),
}



#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BibliographicLevel {
    #[default]
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
#[serde(rename_all = "camelCase")]
pub enum Language {
    French,
    English,
    German,
    Spanish,
    Italian,
    Portuguese,
    Japanese,
    Chinese,
    Russian,
    Arabic,
    Dutch,
    Swedish,
    Norwegian,
    Danish,
    Finnish,
    Polish,
    Czech,
    Hungarian,
    Romanian,
    Turkish,
    Korean,
    Latin,
    Greek,
    Croatian,
    Hindi,
    Hebrew,
    Persian,
    Catalan,
    Thai,
    Vietnamese,
    Indonesian,
    Malay,
    Other(String),
}

impl Default for Language {
    fn default() -> Self {
        Language::Other(String::new())
    }
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
            "jpn" => Language::Japanese,
            "chi" | "zho" => Language::Chinese,
            "rus" => Language::Russian,
            "ara" => Language::Arabic,
            "dut" | "nld" => Language::Dutch,
            "swe" => Language::Swedish,
            "nor" => Language::Norwegian,
            "dan" => Language::Danish,
            "fin" => Language::Finnish,
            "pol" => Language::Polish,
            "cze" | "ces" => Language::Czech,
            "hun" => Language::Hungarian,
            "rum" | "ron" => Language::Romanian,
            "tur" => Language::Turkish,
            "kor" => Language::Korean,
            "lat" => Language::Latin,
            "gre" | "ell" => Language::Greek,
            "hrv" | "scr" => Language::Croatian,
            "hin" => Language::Hindi,
            "heb" => Language::Hebrew,
            "per" | "fas" => Language::Persian,
            "cat" => Language::Catalan,
            "tha" => Language::Thai,
            "vie" => Language::Vietnamese,
            "ind" => Language::Indonesian,
            "may" | "msa" => Language::Malay,
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
            Language::Japanese => "jpn",
            Language::Chinese => "chi",
            Language::Russian => "rus",
            Language::Arabic => "ara",
            Language::Dutch => "dut",
            Language::Swedish => "swe",
            Language::Norwegian => "nor",
            Language::Danish => "dan",
            Language::Finnish => "fin",
            Language::Polish => "pol",
            Language::Czech => "cze",
            Language::Hungarian => "hun",
            Language::Romanian => "rum",
            Language::Turkish => "tur",
            Language::Korean => "kor",
            Language::Latin => "lat",
            Language::Greek => "gre",
            Language::Croatian => "hrv",
            Language::Hindi => "hin",
            Language::Hebrew => "heb",
            Language::Persian => "per",
            Language::Catalan => "cat",
            Language::Thai => "tha",
            Language::Vietnamese => "vie",
            Language::Indonesian => "ind",
            Language::Malay => "may",
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
    Japan,
    China,
    Russia,
    Brazil,
    Canada,
    Australia,
    Netherlands,
    Belgium,
    Switzerland,
    Austria,
    Portugal,
    Poland,
    India,
    Mexico,
    Argentina,
    SouthKorea,
    Sweden,
    Norway,
    Denmark,
    Finland,
    Other(String),
}

impl Default for Country {
    fn default() -> Self {
        Country::Other(String::new())
    }
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
            "JP" | "ja " => Country::Japan,
            "CN" | "cc " => Country::China,
            "RU" | "ru " => Country::Russia,
            "BR" | "bl " => Country::Brazil,
            "CA" | "xxc" => Country::Canada,
            "AU" | "at " => Country::Australia,
            "NL" | "ne " => Country::Netherlands,
            "BE" | "be " => Country::Belgium,
            "CH" | "sz " => Country::Switzerland,
            "AT" | "au " => Country::Austria,
            "PT" | "po " => Country::Portugal,
            "PL" | "pl " => Country::Poland,
            "IN" | "ii " => Country::India,
            "MX" | "mx " => Country::Mexico,
            "AR" | "ag " => Country::Argentina,
            "KR" | "ko " => Country::SouthKorea,
            "SE" | "sw " => Country::Sweden,
            "NO" | "no " => Country::Norway,
            "DK" | "dk " => Country::Denmark,
            "FI" | "fi " => Country::Finland,
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
            Country::Japan => "JP",
            Country::China => "CN",
            Country::Russia => "RU",
            Country::Brazil => "BR",
            Country::Canada => "CA",
            Country::Australia => "AU",
            Country::Netherlands => "NL",
            Country::Belgium => "BE",
            Country::Switzerland => "CH",
            Country::Austria => "AT",
            Country::Portugal => "PT",
            Country::Poland => "PL",
            Country::India => "IN",
            Country::Mexico => "MX",
            Country::Argentina => "AR",
            Country::SouthKorea => "KR",
            Country::Sweden => "SE",
            Country::Norway => "NO",
            Country::Denmark => "DK",
            Country::Finland => "FI",
            Country::Other(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TargetAudience {
    Juvenile,
    Preschool,
    Primary,
    Children,
    YoungAdult,
    AdultSerious,
    Adult,
    General,
    Specialized,
    #[default]
    Unknown,
    Other(String),
}


#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Isbn {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifying: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parallel: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_part: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_of_part: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[marc(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<PublicationFunction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacture_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacture_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
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
    Dissertation,
    SystemDetails,
    LanguageNote,
    Awards,
    Reproduction,
    Frequency,
    Credits,
    Other(String),
}

impl Default for NoteType {
    fn default() -> Self {
        NoteType::Other(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Relator {
    Author,
    Illustrator,
    Translator,
    Editor,
    PrefaceWriter,
    Photographer,
    Publisher,
    Composer,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Agent {
    Person(Person),
    CorporateBody(CorporateBody),
    Meeting(Meeting),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles_associated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuller_form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator: Option<Relator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorporateBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subordinate_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct LinkedRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<LinkType>,
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkType {
    IsPartOf,
    HasPart,
    Series,
    Supplement,
    SupplementParent,
    IssuedWith,
    Continues,
    ContinuedBy,
    Supersedes,
    SupersededBy,
    Translation,
    TranslationOf,
    OtherEdition,
    OtherEditionDiffLang,
    Preceding,
    Succeeding,
    SetLevel,
    SubsetLevel,
    PieceLevel,
    Other(String),
}

impl Default for LinkType {
    fn default() -> Self {
        LinkType::Other(String::new())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
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
    Meeting,
    UniformTitle,
    Topical,
    Geographic,
    Genre,
    Uncontrolled,
    Other(String),
}

impl Default for SubjectType {
    fn default() -> Self {
        SubjectType::Other(String::new())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
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

impl Default for ClassificationScheme {
    fn default() -> Self {
        ClassificationScheme::Other(String::new())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct CatalogingSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_conventions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribing_agency: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifying_agencies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cataloging_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub describing_conventions: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct LocationCallNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shelving_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_part: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub piece_designation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_note: Option<String>,
}

/// Item barcode: no whitespace allowed (invalid values fail MARC mapping and serde).
/// Default is empty (used by `MarcPaths` codegen only; prefer `Option<Barcode>` for absence).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Barcode(String);

impl Barcode {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    fn validate(s: &str) -> Result<(), &'static str> {
        if s.chars().any(|c| c.is_whitespace()) {
            Err("barcode must not contain whitespace")
        } else {
            Ok(())
        }
    }
}

impl Deref for Barcode {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Barcode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Barcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Barcode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Barcode(s.to_string()))
    }
}

impl TryFrom<&str> for Barcode {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for Barcode {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::validate(&s)?;
        Ok(Barcode(s))
    }
}

impl From<Barcode> for String {
    fn from(b: Barcode) -> Self {
        b.0
    }
}

impl Serialize for Barcode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Barcode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Barcode::try_from(s).map_err(serde::de::Error::custom)
    }
}

impl super::MarcPaths for Barcode {
    const IS_LEAF: bool = true;

    fn from_marc_str(s: &str) -> Result<Self, &'static str> {
        s.parse()
    }

    fn to_marc_str(&self) -> String {
        self.0.clone()
    }

    fn marc_set(&mut self, _: &str, _: &str) -> Result<bool, &'static str> {
        Ok(false)
    }

    fn marc_get_option(&self, _: &str) -> Option<String> {
        None
    }

    fn marc_get_vec(&self, _: &str) -> Option<Vec<String>> {
        None
    }

    fn marc_path_kind(_: &str) -> Option<super::PathKind> {
        None
    }

    fn marc_has_path(_: &str) -> bool {
        false
    }

    fn marc_is_vec_leaf(_: &str) -> bool {
        false
    }

    fn marc_creator_field() -> &'static str {
        ""
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Item {
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
    pub barcode: Option<Barcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<String>,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct PatentNumber {
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<String>,
}

/// Standard technical report number (MARC21 027).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalReportNumber {
    pub value: String,
}

/// Publisher number (MARC21 028).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct PublisherNumber {
    pub value: String,
    pub source: Option<String>,
}

/// CODEN designation (MARC21 030).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct Coden {
    pub value: String,
}

/// Original study number for computer files (MARC21 036).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct OriginalStudyNumber {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Government document call number (MARC21 086).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct GovernmentDocumentNumber {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Generic report number (MARC21 088).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct ReportNumber {
    pub value: String,
}

/// Series statement (UNIMARC 225 / MARC21 490).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatement {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
}

/// Varying form of title (UNIMARC 517 / MARC21 246).
#[derive(Debug, Clone, Default, Serialize, Deserialize, MarcPaths)]
#[serde(rename_all = "camelCase")]
pub struct VaryingTitle {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
}
