use serde::{Deserialize, Serialize};

use crate::fields::{
    AddedEntry, Control, DeweyClassification, Edition, Isbn, LanguageData, Linking, MainEntry,
    Note, Physical, Series, Specimen, Subject, Title,
};
use crate::fields::common::{CorporateNameData, MeetingNameData, PersonalNameData};

/// MARC record structure with typed fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    leader: Leader,
    control: Vec<Control>,
    isbns: Vec<Isbn>,
    titles: Vec<Title>,
    main_entries: Vec<MainEntry>,
    editions: Vec<Edition>,
    physical: Vec<Physical>,
    series: Vec<Series>,
    notes: Vec<Note>,
    subjects: Vec<Subject>,
    added_entries: Vec<AddedEntry>,
    linking: Vec<Linking>,
    specimens: Vec<Specimen>,
    classifications: Vec<DeweyClassification>,
    languages: Vec<LanguageData>,
    other_control: Vec<ControlField>,
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
    pub relator_term: Option<String>,
    pub relator_code: Option<String>,
}

/// Aggregated edition and publication info (250/205 + 260/264/210).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditionInfo {
    /// Edition statement (250 MARC21, 205 UNIMARC).
    pub edition_statement: Option<String>,
    /// Place of publication (first from 260/264/210).
    pub place: Option<String>,
    /// Publisher (first from 260/264/210).
    pub publisher: Option<String>,
    /// Date of publication (first from 260/264/210).
    pub date: Option<String>,
    /// All publication statements (260, 264, 210) for full detail.
    pub publication_statements: Vec<PublicationStatementInfo>,
}

/// One publication/imprint statement (place, publisher, date).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationStatementInfo {
    pub tag: String,
    pub place: Option<String>,
    pub publisher: Option<String>,
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

/// MARC leader (24 bytes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leader {
    pub record_length: u16,
    pub record_status: char,
    pub record_type: char,
    pub bibliographic_level: char,
    pub type_of_control: char,
    pub character_coding_scheme: char,
    pub indicator_count: u8,
    pub subfield_code_count: u8,
    pub base_address_of_data: u16,
    pub encoding_level: char,
    pub descriptive_cataloging_form: char,
    pub multipart_resource_record_level: char,
    pub length_of_length_of_field_portion: u8,
    pub length_of_starting_character_position_portion: u8,
    pub length_of_implementation_defined_portion: u8,
    pub undefined: char,
}

impl Leader {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 24 {
            return Err(format!("Leader must be 24 bytes, got {}", data.len()));
        }

        let record_length = parse_u16(&data[0..5])?;
        let base_address = parse_u16(&data[12..17])?;

        Ok(Leader {
            record_length,
            record_status: data[5] as char,
            record_type: data[6] as char,
            bibliographic_level: data[7] as char,
            type_of_control: data[8] as char,
            character_coding_scheme: data[9] as char,
            indicator_count: data[10] - b'0',
            subfield_code_count: data[11] - b'0',
            base_address_of_data: base_address,
            encoding_level: data[17] as char,
            descriptive_cataloging_form: data[18] as char,
            multipart_resource_record_level: data[19] as char,
            length_of_length_of_field_portion: data[20] - b'0',
            length_of_starting_character_position_portion: data[21] - b'0',
            length_of_implementation_defined_portion: data[22] - b'0',
            undefined: data[23] as char,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 24];
        let record_length_str = format!("{:05}", self.record_length);
        let base_address_str = format!("{:05}", self.base_address_of_data);

        bytes[0..5].copy_from_slice(record_length_str.as_bytes());
        bytes[5] = self.record_status as u8;
        bytes[6] = self.record_type as u8;
        bytes[7] = self.bibliographic_level as u8;
        bytes[8] = self.type_of_control as u8;
        bytes[9] = self.character_coding_scheme as u8;
        bytes[10] = b'0' + self.indicator_count;
        bytes[11] = b'0' + self.subfield_code_count;
        bytes[12..17].copy_from_slice(base_address_str.as_bytes());
        bytes[17] = self.encoding_level as u8;
        bytes[18] = self.descriptive_cataloging_form as u8;
        bytes[19] = self.multipart_resource_record_level as u8;
        bytes[20] = b'0' + self.length_of_length_of_field_portion;
        bytes[21] = b'0' + self.length_of_starting_character_position_portion;
        bytes[22] = b'0' + self.length_of_implementation_defined_portion;
        bytes[23] = self.undefined as u8;

        bytes
    }
}

fn parse_u16(bytes: &[u8]) -> Result<u16, String> {
    let s = std::str::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))?;
    s.parse::<u16>()
        .map_err(|e| format!("Invalid number: {}", e))
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
    pub subfields: Vec<Subfield>,
}

/// Subfield within a data field
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subfield {
    pub code: char,
    pub value: String,
}
