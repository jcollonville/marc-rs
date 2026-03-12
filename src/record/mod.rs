use serde::{Deserialize, Serialize};

mod types;
pub use types::*;

/// High-level semantic representation of a MARC bibliographic record,
/// organized following the standard block numbering (0XX-9XX).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub leader: Leader,

    /// 0XX - Identification
    #[serde(default)]
    pub identification: Identification,
    /// 1XX - Coded information
    #[serde(default)]
    pub coded: Coded,
    /// 2XX - Descriptive information
    #[serde(default)]
    pub description: Description,
    /// 3XX - Notes
    #[serde(default)]
    pub notes: Notes,
    /// 4XX - Links to other bibliographic records
    #[serde(default)]
    pub links: Links,
    /// 5XX - Associated titles
    #[serde(default)]
    pub associated_titles: AssociatedTitles,
    /// 6XX - Subject indexing
    #[serde(default)]
    pub indexing: Indexing,
    /// 7XX - Responsibility
    #[serde(default)]
    pub responsibility: Responsibility,
    /// 8XX - International data
    #[serde(default)]
    pub international: International,
    /// 9XX - National and local data
    #[serde(default)]
    pub local: Local,
}

impl Record {
    pub fn authors(&self) -> impl Iterator<Item = &Agent> {
        self.responsibility
            .main_entry
            .iter()
            .chain(self.responsibility.added_entries.iter())
    }

    pub fn languages(&self) -> &[Language] {
        &self.coded.languages
    }

    pub fn titles(&self) -> Vec<&Title> {
        let mut out = Vec::new();
        if let Some(t) = &self.description.title {
            out.push(t);
        }
        if let Some(t) = &self.associated_titles.uniform_title {
            out.push(t);
        }
        out
    }

    pub fn audience(&self) -> Option<&TargetAudience> {
        self.coded.target_audience.as_ref()
    }

    pub fn isbn(&self) -> &[Isbn] {
        &self.identification.isbn
    }

    pub fn specimens(&self) -> &[Specimen] {
        &self.local.specimens
    }

  
}

/// 0XX - Identification block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_version_date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isbn: Vec<Isbn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issn: Vec<Issn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_bibliography_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub national_library_record_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal_deposit_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lccn: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub system_control_numbers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patent_numbers: Vec<PatentNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technical_report_numbers: Vec<TechnicalReportNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publisher_numbers: Vec<PublisherNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codens: Vec<Coden>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub original_study_numbers: Vec<OriginalStudyNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub government_document_numbers: Vec<GovernmentDocumentNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_numbers: Vec<ReportNumber>,
}

/// 1XX - Coded information block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coded {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_dates: Option<(MarcDate, Option<MarcDate>)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<TargetAudience>,
    /// MARC21 043 - Geographic area codes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub geographic_area_codes: Vec<String>,
    /// MARC21 045 - Time period codes (raw)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_period_codes: Vec<String>,
}

/// 2XX - Descriptive information block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publication: Vec<Publication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_description: Option<PhysicalDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_statement: Option<String>,
}

/// 3XX - Notes block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Note>,
}

/// 4XX - Links to other bibliographic records
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<LinkedRecord>,
}

/// 5XX - Associated titles block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedTitles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_title: Option<Title>,
}

/// 6XX - Subject indexing block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indexing {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<Classification>,
}

/// 7XX - Responsibility block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Responsibility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_entry: Option<Agent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entries: Vec<Agent>,
}

/// 8XX - International data block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct International {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cataloging_sources: Vec<CatalogingSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location_call_numbers: Vec<LocationCallNumber>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub electronic_locations: Vec<ElectronicLocation>,
    /// MARC21 850 - Holding institution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding_institutions: Vec<String>,
}

/// 9XX - National and local data block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimens: Vec<Specimen>,
}
