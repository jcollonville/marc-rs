use serde::{Deserialize, Serialize};

use crate::author::{Author, collect_authors};
use crate::fields::{
    AddedEntry, Control, DeweyClassification, Edition, Isbn, LanguageData, Linking,
    MainEntry, Note, Physical, Series, Specimen, Subject, Title,
};
use crate::fields::language::LanguageCode;
use crate::leader::*;

/// MARC record structure with typed fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub leader: Leader,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control: Vec<Control>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub isbns: Vec<Isbn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub titles: Vec<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_entries: Vec<MainEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editions: Vec<Edition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub physical: Vec<Physical>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<Series>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_entries: Vec<AddedEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linking: Vec<Linking>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimens: Vec<Specimen>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<DeweyClassification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<LanguageData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_control: Vec<ControlField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_data: Vec<DataField>,
}

/// Aggregated edition and publication info (250/205 + 260/264/210).
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

/// One publication/imprint statement (place, publisher, date).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationStatementInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl Record {
    pub fn new(leader: Leader) -> Self {
        Self {
            leader,
            control: Vec::new(),
            isbns: Vec::new(),
            titles: Vec::new(),
            main_entries: Vec::new(),
            editions: Vec::new(),
            physical: Vec::new(),
            series: Vec::new(),
            notes: Vec::new(),
            subjects: Vec::new(),
            added_entries: Vec::new(),
            linking: Vec::new(),
            specimens: Vec::new(),
            classifications: Vec::new(),
            languages: Vec::new(),
            other_control: Vec::new(),
            other_data: Vec::new(),
        }
    }

    pub fn leader(&self) -> &Leader {
        &self.leader
    }
    pub fn set_leader(&mut self, leader: Leader) {
        self.leader = leader;
    }
    pub fn control(&self) -> &[Control] {
        &self.control
    }
    pub fn isbns(&self) -> &[Isbn] {
        &self.isbns
    }
    pub fn titles(&self) -> &[Title] {
        &self.titles
    }
    pub fn main_entries(&self) -> &[MainEntry] {
        &self.main_entries
    }
    pub fn editions(&self) -> &[Edition] {
        &self.editions
    }
    pub fn physical(&self) -> &[Physical] {
        &self.physical
    }
    pub fn series(&self) -> &[Series] {
        &self.series
    }
    pub fn notes(&self) -> &[Note] {
        &self.notes
    }
    pub fn subjects(&self) -> &[Subject] {
        &self.subjects
    }
    pub fn added_entries(&self) -> &[AddedEntry] {
        &self.added_entries
    }
    pub fn linking(&self) -> &[Linking] {
        &self.linking
    }
    pub fn specimens(&self) -> &[Specimen] {
        &self.specimens
    }
    pub fn classifications(&self) -> &[DeweyClassification] {
        &self.classifications
    }
    pub fn languages(&self) -> &[LanguageData] {
        &self.languages
    }
    pub fn other_control(&self) -> &[ControlField] {
        &self.other_control
    }
    pub fn other_data(&self) -> &[DataField] {
        &self.other_data
    }

    pub fn push_control(&mut self, c: Control) {
        self.control.push(c);
    }
    pub fn push_other_control(&mut self, c: ControlField) {
        self.other_control.push(c);
    }
    pub fn push_isbn(&mut self, isbn: Isbn) {
        self.isbns.push(isbn);
    }
    pub fn push_title(&mut self, t: Title) {
        self.titles.push(t);
    }
    pub fn push_main_entry(&mut self, me: MainEntry) {
        self.main_entries.push(me);
    }
    pub fn push_edition(&mut self, ed: Edition) {
        self.editions.push(ed);
    }
    pub fn push_physical(&mut self, ph: Physical) {
        self.physical.push(ph);
    }
    pub fn push_series(&mut self, se: Series) {
        self.series.push(se);
    }
    pub fn push_note(&mut self, no: Note) {
        self.notes.push(no);
    }
    pub fn push_subject(&mut self, su: Subject) {
        self.subjects.push(su);
    }
    pub fn push_added_entry(&mut self, ae: AddedEntry) {
        self.added_entries.push(ae);
    }
    pub fn push_linking(&mut self, li: Linking) {
        self.linking.push(li);
    }
    pub fn push_specimen(&mut self, sp: Specimen) {
        self.specimens.push(sp);
    }
    pub fn push_classification(&mut self, dc: DeweyClassification) {
        self.classifications.push(dc);
    }
    pub fn push_language(&mut self, lang: LanguageData) {
        self.languages.push(lang);
    }
    pub fn push_other_data(&mut self, df: DataField) {
        self.other_data.push(df);
    }

    /// Collect all authors from main entries (1XX) and added entries (70X–71X).
    /// Order: main entry first, then added entries. Uniform titles are skipped.
    pub fn authors(&self) -> Vec<Author> {
        collect_authors(&self.main_entries, &self.added_entries)
    }

    /// Edition and publication info: statement (250/205), place, publisher, date (260/264/210).
    pub fn edition_info(&self) -> EditionInfo {
        let mut edition_statement = None;
        let mut place = None;
        let mut publisher = None;
        let mut date = None;
        let mut publication_statements = Vec::new();

        for e in &self.editions {
            match e {
                Edition::EditionStatement(d) => {
                    if edition_statement.is_none() {
                        edition_statement = Some(d.edition.clone());
                    }
                }
                Edition::Publication(d) => {
                    if place.is_none() {
                        place = d.place().map(String::from);
                    }
                    if publisher.is_none() {
                        publisher = d.publisher().map(String::from);
                    }
                    if date.is_none() {
                        date = d.date().map(String::from);
                    }
                    publication_statements.push(PublicationStatementInfo {
                        place: d.place().map(String::from),
                        publisher: d.publisher().map(String::from),
                        date: d.date().map(String::from),
                    });
                }
                _ => {}
            }
        }

        EditionInfo {
            edition_statement,
            place,
            publisher,
            date,
            publication_statements,
        }
    }

    /// Collection/series names (440, 490, 225 — title or statement).
    pub fn collections(&self) -> Vec<String> {
        let mut out = Vec::new();
        for s in &self.series {
            match s {
                Series::SeriesTitle(d) | Series::SeriesStatement(d) => {
                    out.push(d.statement.clone());
                }
                _ => {}
            }
        }
        out
    }

    /// All language codes (041 $a + UNIMARC 101 from Physical::AssociatedLanguage).
    pub fn language_codes(&self) -> Vec<LanguageCode> {
        let mut out = Vec::new();
        for lang in &self.languages {
            out.extend(lang.codes.clone());
        }
        for ph in &self.physical {
            if let Physical::AssociatedLanguage(d) = ph {
                if !d.text.is_empty() {
                    out.push(LanguageCode::from_code(&d.text));
                }
            }
        }
        out
    }

    /// First audience/public type (385 MARC21, 330 UNIMARC).
    pub fn audience(&self) -> Option<String> {
        for ph in &self.physical {
            if let Physical::AudienceCharacteristics(d) = ph {
                return Some(d.text.clone());
            }
        }
        None
    }

    /// First Dewey number if present (082/083/676).
    pub fn dewey(&self) -> Option<&str> {
        self.classifications.first().and_then(DeweyClassification::first_number)
    }
}

/// Builder for constructing `Record` instances step by step.
#[derive(Debug, Clone)]
pub struct RecordBuilder {
    record: Record,
}

impl RecordBuilder {
    pub fn new(leader: Leader) -> Self {
        Self {
            record: Record::new(leader),
        }
    }

    pub fn leader(mut self, leader: Leader) -> Self {
        self.record.leader = leader;
        self
    }

    pub fn control(mut self, c: Control) -> Self {
        self.record.control.push(c);
        self
    }

    pub fn isbn(mut self, isbn: Isbn) -> Self {
        self.record.isbns.push(isbn);
        self
    }

    pub fn title(mut self, t: Title) -> Self {
        self.record.titles.push(t);
        self
    }

    pub fn main_entry(mut self, me: MainEntry) -> Self {
        self.record.main_entries.push(me);
        self
    }

    pub fn edition(mut self, ed: Edition) -> Self {
        self.record.editions.push(ed);
        self
    }

    pub fn physical(mut self, ph: Physical) -> Self {
        self.record.physical.push(ph);
        self
    }

    pub fn series(mut self, se: Series) -> Self {
        self.record.series.push(se);
        self
    }

    pub fn note(mut self, no: Note) -> Self {
        self.record.notes.push(no);
        self
    }

    pub fn subject(mut self, su: Subject) -> Self {
        self.record.subjects.push(su);
        self
    }

    pub fn added_entry(mut self, ae: AddedEntry) -> Self {
        self.record.added_entries.push(ae);
        self
    }

    pub fn linking(mut self, li: Linking) -> Self {
        self.record.linking.push(li);
        self
    }

    pub fn specimen(mut self, sp: Specimen) -> Self {
        self.record.specimens.push(sp);
        self
    }

    pub fn classification(mut self, dc: DeweyClassification) -> Self {
        self.record.classifications.push(dc);
        self
    }

    pub fn language(mut self, lang: LanguageData) -> Self {
        self.record.languages.push(lang);
        self
    }

    pub fn other_control(mut self, c: ControlField) -> Self {
        self.record.other_control.push(c);
        self
    }

    pub fn other_data(mut self, df: DataField) -> Self {
        self.record.other_data.push(df);
        self
    }

    pub fn build(self) -> Record {
        self.record
    }
}

impl Record {
    pub fn builder(leader: Leader) -> RecordBuilder {
        RecordBuilder::new(leader)
    }
}

/// Raw control field (001-009) — used for the "other" bucket and writing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlField {
    pub tag: String,
    pub value: String,
}

/// Raw data field (010-999) — used for the "other" bucket and writing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataField {
    pub tag: String,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind1: char,
    #[serde(
        default = "crate::fields::common::default_indicator",
        skip_serializing_if = "crate::fields::common::is_default_indicator"
    )]
    pub ind2: char,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subfields: Vec<Subfield>,
}

/// Subfield within a data field
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subfield {
    pub code: char,
    pub value: String,
}
