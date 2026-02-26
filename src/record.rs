use serde::{Deserialize, Serialize};

use crate::fields::{
    AddedEntry, Control, DeweyClassification, Edition, Isbn, LanguageData, Linking, MainEntry,
    Note, Physical, Series, Specimen, Subject, Title,
};
use crate::fields::common::{CorporateNameData, MeetingNameData, PersonalNameData};
use crate::leader::*;

/// MARC record structure with typed fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    leader: Leader,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    control: Vec<Control>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    isbns: Vec<Isbn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    titles: Vec<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    main_entries: Vec<MainEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    editions: Vec<Edition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    physical: Vec<Physical>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    series: Vec<Series>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    notes: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    subjects: Vec<Subject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    added_entries: Vec<AddedEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    linking: Vec<Linking>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    specimens: Vec<Specimen>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    classifications: Vec<DeweyClassification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    languages: Vec<LanguageData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    other_control: Vec<ControlField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    other_data: Vec<DataField>,
}

/// Author kind (personal, corporate, meeting).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorKind {
    Personal,
    Corporate,
    Meeting,
}

/// Extracted author from main or added entry (100/700, 110/710, 111/711).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub kind: AuthorKind,
    /// Display form: "Name, dates" for personal; full form for corporate/meeting.
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relator_code: Option<String>,
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
    pub tag: String,
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
        let mut out = Vec::new();
        for e in &self.main_entries {
            if let Some(a) = author_from_main_entry(e) {
                out.push(a);
            }
        }
        for e in &self.added_entries {
            if let Some(a) = author_from_added_entry(e) {
                out.push(a);
            }
        }
        out
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
                        tag: d.tag.clone(),
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
    pub fn language_codes(&self) -> Vec<String> {
        let mut out = Vec::new();
        for lang in &self.languages {
            out.extend(lang.codes.clone());
        }
        for ph in &self.physical {
            if let Physical::AssociatedLanguage(d) = ph {
                if !d.text.is_empty() {
                    out.push(d.text.clone());
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

fn author_from_main_entry(e: &MainEntry) -> Option<Author> {
    match e {
        MainEntry::PersonalName(d) => Some(author_personal(d)),
        MainEntry::CorporateName(d) => Some(author_corporate(d)),
        MainEntry::MeetingName(d) => Some(author_meeting(d)),
        MainEntry::UniformTitle(_) => None,
    }
}

fn author_from_added_entry(e: &AddedEntry) -> Option<Author> {
    match e {
        AddedEntry::PersonalName(d) => Some(author_personal(d)),
        AddedEntry::CorporateName(d) => Some(author_corporate(d)),
        AddedEntry::MeetingName(d) => Some(author_meeting(d)),
        _ => None,
    }
}

fn author_personal(d: &PersonalNameData) -> Author {
    let display_name = personal_display_name(d);
    Author {
        kind: AuthorKind::Personal,
        display_name,
        relator_term: d.relator_term.clone(),
        relator_code: d.relator_code.clone(),
    }
}

fn author_corporate(d: &CorporateNameData) -> Author {
    let display_name = corporate_display_name(d);
    Author {
        kind: AuthorKind::Corporate,
        display_name,
        relator_term: d.relator_term.clone(),
        relator_code: d.relator_code.clone(),
    }
}

fn author_meeting(d: &MeetingNameData) -> Author {
    let display_name = meeting_display_name(d);
    Author {
        kind: AuthorKind::Meeting,
        display_name,
        relator_term: None,
        relator_code: None,
    }
}

fn personal_display_name(d: &PersonalNameData) -> String {
    let mut s = d.name.clone();
    if let Some(ref b) = d.numeration {
        s.push_str(" ");
        s.push_str(b);
    }
    if let Some(ref c) = d.titles {
        s.push_str(" ");
        s.push_str(c);
    }
    if let Some(ref d) = d.dates {
        s.push_str(", ");
        s.push_str(d);
    }
    s
}

fn corporate_display_name(d: &CorporateNameData) -> String {
    let mut s = d.name.clone();
    if let Some(ref b) = d.subordinate_unit {
        s.push_str(". ");
        s.push_str(b);
    }
    if let Some(ref c) = d.location {
        s.push_str(" ");
        s.push_str(c);
    }
    if let Some(ref d) = d.date {
        s.push_str(" ");
        s.push_str(d);
    }
    s
}

fn meeting_display_name(d: &MeetingNameData) -> String {
    let mut s = d.name.clone();
    if let Some(ref c) = d.location {
        s.push_str(" ");
        s.push_str(c);
    }
    if let Some(ref d) = d.date {
        s.push_str(" ");
        s.push_str(d);
    }
    if let Some(ref e) = d.subordinate_unit {
        s.push_str(". ");
        s.push_str(e);
    }
    if let Some(ref n) = d.number {
        s.push_str(" ");
        s.push_str(n);
    }
    s
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
    pub ind1: char,
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
