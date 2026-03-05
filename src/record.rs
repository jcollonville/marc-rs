use serde::{Deserialize, Serialize};

use crate::blocks::{
    AssociatedTitlesBlock, CodedInformationBlock, DescriptionBlock, IdentificationBlock,
    IntellectualResponsibilityBlock, InternationalUseBlock, LinksBlock, LocalUseBlock,
    MarcBlock, NotesBlock, SubjectAnalysisBlock,
};
use crate::datatypes::{Specimen, default_indicator, is_default_indicator};
use crate::datatypes::language::LanguageCode;
use crate::formats::{FieldType, FormatDescriptor};
use crate::{MarcFormat, leader::*};

/// Raw control field (001-009).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlField {
    pub tag: String,
    pub value: String,
}

/// Raw data field (010-999).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataField {
    pub tag: String,
    #[serde(
        default = "default_indicator",
        skip_serializing_if = "is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "default_indicator",
        skip_serializing_if = "is_default_indicator"
    )]
    pub ind2: char,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subfields: Vec<Subfield>,
}

/// Subfield within a data field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subfield {
    pub code: char,
    pub value: String,
}

/// Aggregated edition and publication info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_statement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publication_statements: Vec<PublicationStatementInfo>,
}

/// Collection/series info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    pub traced: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CollectionInfoKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionInfoKind {
    Mention,
    Uniform,
}

/// One publication/imprint statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationStatementInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// MARC record structured into 10 semantic blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_format: Option<MarcFormat>,
    pub leader: Leader,
    /// Bloc 0 – Identification (0xx)
    #[serde(default)]
    pub identification: IdentificationBlock,
    /// Bloc 1 – Informations codées (1xx)
    #[serde(default)]
    pub coded_information: CodedInformationBlock,
    /// Bloc 2 – Description (2xx/3xx)
    #[serde(default)]
    pub description: DescriptionBlock,
    /// Bloc 3 – Notes (3xx/5xx)
    #[serde(default)]
    pub notes: NotesBlock,
    /// Bloc 4 – Liens (4xx/76x-78x)
    #[serde(default)]
    pub links: LinksBlock,
    /// Bloc 5 – Titres associés (5xx)
    #[serde(default)]
    pub associated_titles: AssociatedTitlesBlock,
    /// Bloc 6 – Analyse matière (6xx)
    #[serde(default)]
    pub subject_analysis: SubjectAnalysisBlock,
    /// Bloc 7 – Responsabilité intellectuelle (7xx)
    #[serde(default)]
    pub intellectual_responsibility: IntellectualResponsibilityBlock,
    /// Bloc 8 – Usage international (8xx)
    #[serde(default)]
    pub international_use: InternationalUseBlock,
    /// Bloc 9 – Usage local (9xx)
    #[serde(default)]
    pub local_use: LocalUseBlock,
}

impl Default for Record {
    fn default() -> Self {
        Self::new(None, Leader::default())
    }
}

impl Record {
    pub fn new(original_format: Option<MarcFormat>, leader: Leader) -> Self {
        Self {
            original_format,
            leader,
            identification: IdentificationBlock::default(),
            coded_information: CodedInformationBlock::default(),
            description: DescriptionBlock::default(),
            notes: NotesBlock::default(),
            links: LinksBlock::default(),
            associated_titles: AssociatedTitlesBlock::default(),
            subject_analysis: SubjectAnalysisBlock::default(),
            intellectual_responsibility: IntellectualResponsibilityBlock::default(),
            international_use: InternationalUseBlock::default(),
            local_use: LocalUseBlock::default(),
        }
    }

    pub fn leader(&self) -> &Leader {
        &self.leader
    }

    pub fn set_leader(&mut self, leader: Leader) {
        self.leader = leader;
    }

    /// Dispatch a data field to the correct block using the format descriptor for routing.
    pub fn dispatch_data_field(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        if let Some(desc) = descriptor.tag_descriptor(tag) {
            use crate::formats::BlockId;
            match desc.block {
                BlockId::Identification => {
                    self.identification.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::CodedInformation => {
                    self.coded_information.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::Description => {
                    self.description.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::Notes => {
                    self.notes.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::Links => {
                    self.links.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::AssociatedTitles => {
                    self.associated_titles.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::SubjectAnalysis => {
                    self.subject_analysis.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::IntellectualResponsibility => {
                    self.intellectual_responsibility.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::InternationalUse => {
                    self.international_use.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
                BlockId::LocalUse => {
                    self.local_use.dispatch_data(tag, ind1, ind2, subfields, descriptor);
                }
            }
        } else {
            // Unknown tag – fallback to block routing by prefix
            self.dispatch_data_by_tag_prefix(tag, ind1, ind2, subfields, descriptor);
        }
    }

    fn dispatch_data_by_tag_prefix(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    ) {
        let prefix = tag.chars().next().unwrap_or('0');
        match prefix {
            '0' => self.identification.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '1' => self.coded_information.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '2' | '3' => self.description.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '4' => self.links.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '5' => self.associated_titles.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '6' => self.subject_analysis.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '7' => self.intellectual_responsibility.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '8' => self.international_use.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            '9' => self.local_use.dispatch_data(tag, ind1, ind2, subfields, descriptor),
            _ => {}
        }
    }

    /// Dispatch a control field to the correct block.
    pub fn dispatch_control_field(
        &mut self,
        tag: &str,
        value: &str,
        descriptor: &dyn FormatDescriptor,
    ) {
        // Control fields 00x go to identification or coded_information
        match tag {
            "001" | "003" | "005" | "009" => {
                self.identification.dispatch_control(tag, value, descriptor);
            }
            "006" | "007" | "008" => {
                self.coded_information.dispatch_control(tag, value, descriptor);
            }
            _ => {
                self.identification.dispatch_control(tag, value, descriptor);
            }
        }
    }

    /// Collect all raw fields ordered by block for writing.
    pub fn collect_raw_fields(&self, descriptor: &dyn FormatDescriptor) -> (Vec<ControlField>, Vec<DataField>) {
        let mut controls = Vec::new();
        let mut data = Vec::new();

        controls.extend(self.identification.collect_control_fields(descriptor));
        controls.extend(self.coded_information.collect_control_fields(descriptor));

        data.extend(self.identification.collect_data_fields(descriptor));
        data.extend(self.coded_information.collect_data_fields(descriptor));
        data.extend(self.description.collect_data_fields(descriptor));
        data.extend(self.notes.collect_data_fields(descriptor));
        data.extend(self.links.collect_data_fields(descriptor));
        data.extend(self.associated_titles.collect_data_fields(descriptor));
        data.extend(self.subject_analysis.collect_data_fields(descriptor));
        data.extend(self.intellectual_responsibility.collect_data_fields(descriptor));
        data.extend(self.international_use.collect_data_fields(descriptor));
        data.extend(self.local_use.collect_data_fields(descriptor));

        (controls, data)
    }

    // ── Utility / accessor methods ─────────────────────────────────────

    /// All authors (main + added entries) as flat `Author` list.
    pub fn authors(&self) -> Vec<crate::author::Author> {
        use crate::author::{author_from_personal_name, author_from_corporate_name, author_from_meeting_name};
        let ir = &self.intellectual_responsibility;
        let mut out = Vec::new();

        if let Some(ref p) = ir.main_entry_personal_name {
            out.push(author_from_personal_name(p));
        }
        if let Some(ref c) = ir.main_entry_corporate_name {
            out.push(author_from_corporate_name(c));
        }
        if let Some(ref m) = ir.main_entry_meeting_name {
            out.push(author_from_meeting_name(m));
        }
        for p in &ir.added_entry_personal_names {
            out.push(author_from_personal_name(p));
        }
        for c in &ir.added_entry_corporate_names {
            out.push(author_from_corporate_name(c));
        }
        for m in &ir.added_entry_meeting_names {
            out.push(author_from_meeting_name(m));
        }
        out
    }

    /// Edition and publication info.
    pub fn edition_info(&self) -> EditionInfo {
        let d = &self.description;
        let edition_statement = match &d.edition_statement {
            Some(crate::datatypes::edition::Edition::EditionStatement(ed)) => {
                Some(ed.edition.clone())
            }
            _ => None,
        };

        let (place, publisher, date, publication_statements) =
            if let Some(ref p) = d.publication_distribution_imprint {
                let ps = PublicationStatementInfo {
                    place: p.place().map(String::from),
                    publisher: p.publisher().map(String::from),
                    date: p.date().map(String::from),
                };
                (
                    p.place().map(String::from),
                    p.publisher().map(String::from),
                    p.date().map(String::from),
                    vec![ps],
                )
            } else {
                (None, None, None, Vec::new())
            };

        EditionInfo { edition_statement, place, publisher, date, publication_statements }
    }

    /// Title string (primary title statement).
    pub fn title(&self) -> Option<&str> {
        match &self.description.title_statement {
            Some(crate::datatypes::title::Title::TitleStatement(d)) => Some(&d.title),
            _ => None,
        }
    }

    /// Collection/series titles for display.
    pub fn collections(&self) -> Vec<String> {
        self.description.series_statement.iter().map(|s| s.statement.clone()).collect()
    }

    /// Collection/series details.
    pub fn collection_infos(&self) -> Vec<CollectionInfo> {
        self.description.series_statement.iter().map(|s| CollectionInfo {
            title: s.statement.clone(),
            volume: s.volume.clone(),
            issn: s.issn.clone(),
            traced: s.traced,
            kind: Some(CollectionInfoKind::Mention),
        }).collect()
    }

    /// All language codes (041/101 $a).
    pub fn language_codes(&self) -> Vec<LanguageCode> {
        self.coded_information.language_of_resource
            .as_ref()
            .map(|l| l.codes.clone())
            .unwrap_or_default()
    }

    /// First Dewey number if present (082/083/676).
    pub fn dewey(&self) -> Option<&str> {
        self.subject_analysis.dewey_classification
            .first()
            .and_then(|d| d.first_number())
    }

    /// First electronic access URI (856 $u).
    pub fn electronic_uri(&self) -> Option<&str> {
        self.international_use.electronic_location_access.first().map(String::as_str)
    }

    /// First ISBN number as plain string (sanitized).
    pub fn isbn(&self) -> Option<String> {
        self.identification
            .isbn
            .first()
            .map(|i| i.sanitized_number())
    }

    /// First ISBN price/acquisition information.
    pub fn price(&self) -> Option<String> {
        self.identification
            .isbn
            .first()
            .and_then(|i| i.price_or_acquisition.clone())
    }

    /// Call number from international holdings block (852).
    pub fn call_number(&self) -> Option<&str> {
        self.international_use.location_call_number.as_deref()
    }

    /// Main language of the resource (first 041/101 code).
    pub fn lang(&self) -> Option<LanguageCode> {
        self.coded_information
            .language_of_resource
            .as_ref()
            .and_then(|l| l.codes.first().cloned())
    }

    /// Original language when the record is a translation (second 041/101 code).
    pub fn lang_orig(&self) -> Option<LanguageCode> {
        self.coded_information
            .language_of_resource
            .as_ref()
            .and_then(|l| {
                if l.is_translation == Some(true) && l.codes.len() >= 2 {
                    l.codes.get(1).cloned()
                } else {
                    None
                }
            })
    }

    /// Publication date (first occurrence from publication/imprint statement).
    pub fn publication_date(&self) -> Option<String> {
        self.description
            .publication_distribution_imprint
            .as_ref()
            .and_then(|p| p.date().map(String::from))
    }

    /// Number of pages / extent (300 $a / 215 $a).
    pub fn nb_pages(&self) -> Option<String> {
        self.description
            .physical_description
            .as_ref()
            .map(|p| p.extent.clone())
    }

    /// Physical format / other physical details (300 $b / 215 $d).
    pub fn format(&self) -> Option<String> {
        self.description
            .physical_description
            .as_ref()
            .and_then(|p| p.other_physical_details.clone())
    }

    /// Accompanying material (300 $e / 215 $e).
    pub fn accompanying_material(&self) -> Option<String> {
        self.description
            .physical_description
            .as_ref()
            .and_then(|p| p.accompanying_material.clone())
    }

    /// Media type note(s) (Unimarc 337, MARC21 337, as free text).
    pub fn media_type(&self) -> Vec<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.notes
            .notes
            .iter()
            .filter_map(|n| descriptor.tag_descriptor(&n.tag).map(|d| (d, &n.data)))
            .filter(|(d, _)| d.field == "media_type_note")
            .map(|(_, data)| data.text.clone())
            .collect()
    }

    /// Audience / target audience note(s) (333/521).
    pub fn audience_type(&self) -> Vec<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.notes
            .notes
            .iter()
            .filter_map(|n| descriptor.tag_descriptor(&n.tag).map(|d| (d, &n.data)))
            .filter(|(d, _)| d.field == "audience_note" || d.field == "target_audience_note")
            .map(|(_, data)| data.text.clone())
            .collect()
    }

    /// Table of contents note(s) (327/505).
    pub fn table_of_contents(&self) -> Vec<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.notes
            .notes
            .iter()
            .filter_map(|n| descriptor.tag_descriptor(&n.tag).map(|d| (d, &n.data)))
            .filter(|(d, _)| d.field == "contents_note" || d.field == "formatted_contents_note")
            .map(|(_, data)| data.text.clone())
            .collect()
    }

    /// Abstract / summary note (first 321/330/520).
    pub fn abstract_text(&self) -> Option<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.notes
            .notes
            .iter()
            .filter_map(|n| descriptor.tag_descriptor(&n.tag).map(|d| (d, &n.data)))
            .find(|(d, _)| d.field == "summary_note" || d.field == "native_language_summary_note")
            .map(|(_, data)| data.text.clone())
    }

    /// All free-text notes (3xx/5xx) as strings.
    pub fn notes_texts(&self) -> Vec<String> {
        self.notes
            .notes
            .iter()
            .map(|n| n.data.text.clone())
            .collect()
    }

    /// Keywords / uncontrolled subject terms (653/690/610).
    pub fn keywords(&self) -> Vec<String> {
        self.subject_analysis.local_subject_heading.clone()
    }

    /// Subject headings (excluding genre/form); returns only the main term.
    pub fn subject(&self) -> Vec<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.subject_analysis
            .subjects
            .iter()
            .filter_map(|s| descriptor.tag_descriptor(&s.tag).map(|d| (d, &s.data)))
            .filter(|(d, _)| d.field != "subject_genre_form" && d.field != "index_term_genre_form")
            .map(|(_, data)| data.term.clone())
            .collect()
    }

    /// Genre / form terms (608/655).
    pub fn genre(&self) -> Vec<String> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        self.subject_analysis
            .subjects
            .iter()
            .filter_map(|s| descriptor.tag_descriptor(&s.tag).map(|d| (d, &s.data)))
            .filter(|(d, _)| d.field == "subject_genre_form" || d.field == "index_term_genre_form")
            .map(|(_, data)| data.term.clone())
            .collect()
    }

    /// Collection / series main title (first 490/225).
    pub fn collection(&self) -> Option<String> {
        self.description
            .series_statement
            .first()
            .map(|s| s.statement.clone())
    }

    /// Collection volume number (first 490/225 $v).
    pub fn collection_volume_number(&self) -> Option<String> {
        self.description
            .series_statement
            .first()
            .and_then(|s| s.volume.clone())
    }

    /// Alias for collection volume number, for series volume number usage.
    pub fn series_volume_number(&self) -> Option<String> {
        self.collection_volume_number()
    }

    /// Collection sequence number – not explicitly modeled, always `None` for now.
    pub fn collection_sequence_number(&self) -> Option<String> {
        None
    }

    /// Series titles, same as collection titles list.
    pub fn series(&self) -> Vec<String> {
        self.collections()
    }

    /// Edition statement as plain string, if present.
    pub fn edition(&self) -> Option<String> {
        match &self.description.edition_statement {
            Some(crate::datatypes::edition::Edition::EditionStatement(ed)) => {
                Some(ed.edition.clone())
            }
            _ => None,
        }
    }

    /// Record creation timestamp (not modeled, always None).
    pub fn created_at(&self) -> Option<String> {
        None
    }

    /// Record update timestamp (not modeled, always None).
    pub fn updated_at(&self) -> Option<String> {
        None
    }

    /// Record archival timestamp (not modeled, always None).
    pub fn archived_at(&self) -> Option<String> {
        None
    }

    /// High-level record state derived from specimens (first specimen circulation_status).
    pub fn state(&self) -> Option<String> {
        self.specimens()
            .into_iter()
            .find_map(|s| s.circulation_status)
    }

    /// Record identifier (001).
    pub fn record_id(&self) -> Option<&str> {
        self.identification.record_identifier.as_deref()
    }

    pub fn specimens(&self) -> Vec<Specimen> {
        let descriptor = crate::parser::descriptor_for(self.original_format.unwrap_or_default());
        let tags = descriptor.field_type_to_tag(FieldType::Specimen);

        let mut out = Vec::new();
        for tag in tags {
            let field = self.local_use.data.iter().find(|f| f.tag == tag.tag);
            if let Some(field) = field {
                let subfields: Vec<(char, String)> = field
                    .subfields
                    .iter()
                    .map(|sf| (sf.code, sf.value.clone()))
                    .collect();
                let specimen = Specimen::from_subfields_with_desc(tag, &subfields);
                out.push(specimen);
            }
        }
        out
    }
}

/// Builder for constructing `Record` instances.
pub struct RecordBuilder {
    record: Record,
}

impl RecordBuilder {
    pub fn new(original_format: Option<MarcFormat>, leader: Leader) -> Self {
        Self { record: Record::new(original_format, leader) }
    }

    pub fn leader(mut self, leader: Leader) -> Self {
        self.record.leader = leader;
        self
    }

    /// Apply a closure to the identification block.
    pub fn identification<F: FnOnce(&mut IdentificationBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.identification);
        self
    }

    /// Apply a closure to the coded information block.
    pub fn coded_information<F: FnOnce(&mut CodedInformationBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.coded_information);
        self
    }

    /// Apply a closure to the description block.
    pub fn description<F: FnOnce(&mut DescriptionBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.description);
        self
    }

    /// Apply a closure to the notes block.
    pub fn notes<F: FnOnce(&mut NotesBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.notes);
        self
    }

    /// Apply a closure to the links block.
    pub fn links<F: FnOnce(&mut LinksBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.links);
        self
    }

    /// Apply a closure to the associated titles block.
    pub fn associated_titles<F: FnOnce(&mut AssociatedTitlesBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.associated_titles);
        self
    }

    /// Apply a closure to the subject analysis block.
    pub fn subject_analysis<F: FnOnce(&mut SubjectAnalysisBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.subject_analysis);
        self
    }

    /// Apply a closure to the intellectual responsibility block.
    pub fn intellectual_responsibility<F: FnOnce(&mut IntellectualResponsibilityBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.intellectual_responsibility);
        self
    }

    /// Apply a closure to the international use block.
    pub fn international_use<F: FnOnce(&mut InternationalUseBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.international_use);
        self
    }

    /// Apply a closure to the local use block.
    pub fn local_use<F: FnOnce(&mut LocalUseBlock)>(mut self, f: F) -> Self {
        f(&mut self.record.local_use);
        self
    }

    pub fn build(self) -> Record {
        self.record
    }
}

impl Record {
    pub fn builder(leader: Leader) -> RecordBuilder {
        RecordBuilder::new(None, leader)
    }
}
