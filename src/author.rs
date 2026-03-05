use serde::{Deserialize, Serialize};

use crate::datatypes::{CorporateNameData, MeetingNameData, PersonalNameData, PersonalNameType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorKind {
    Personal,
    Corporate,
    Meeting,
}

/// Semantic author extracted from main entries (1XX) and added entries (70X–71X).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub kind: AuthorKind,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_code: Option<String>,
}


pub fn author_from_personal_name(d: &PersonalNameData) -> Author {
    author_personal(d)
}

pub fn author_from_corporate_name(d: &CorporateNameData) -> Author {
    author_corporate(d)
}

pub fn author_from_meeting_name(d: &MeetingNameData) -> Author {
    author_meeting(d)
}

fn author_personal(d: &PersonalNameData) -> Author {
    let (last_name, first_name) = split_personal_name(&d.name, &d.name_type);
    Author {
        kind: AuthorKind::Personal,
        display_name: personal_display_name(d),
        last_name,
        first_name,
        dates: d.dates.clone(),
        function: d.relator_term.clone(),
        function_code: d.relator_code.clone(),
    }
}

fn author_corporate(d: &CorporateNameData) -> Author {
    Author {
        kind: AuthorKind::Corporate,
        display_name: corporate_display_name(d),
        last_name: None,
        first_name: None,
        dates: d.date.clone(),
        function: d.relator_term.clone(),
        function_code: d.relator_code.clone(),
    }
}

fn author_meeting(d: &MeetingNameData) -> Author {
    Author {
        kind: AuthorKind::Meeting,
        display_name: meeting_display_name(d),
        last_name: None,
        first_name: None,
        dates: d.date.clone(),
        function: None,
        function_code: None,
    }
}

// ── Name helpers ─────────────────────────────────────────────────────

fn trim_isbd(s: &str) -> String {
    s.trim_end_matches([',', '.', ';', ':', ' ']).to_string()
}

/// Split $a into (last_name, first_name) based on name type.
///   Surname  (ind1=1): "Dupont, Jean" -> ("Dupont", "Jean")
///   Forename (ind1=0): "Jean"         -> (None, "Jean")
///   Family   (ind1=3): "La Rochefoucauld" -> ("La Rochefoucauld", None)
fn split_personal_name(
    raw: &str,
    name_type: &PersonalNameType,
) -> (Option<String>, Option<String>) {
    let cleaned = trim_isbd(raw);
    match name_type {
        PersonalNameType::Surname => {
            if let Some((last, first)) = cleaned.split_once(", ") {
                (Some(last.to_string()), Some(trim_isbd(first)))
            } else {
                (Some(cleaned), None)
            }
        }
        PersonalNameType::Forename => (None, Some(cleaned)),
        PersonalNameType::FamilyName => (Some(cleaned), None),
    }
}

fn personal_display_name(d: &PersonalNameData) -> String {
    let mut s = d.name.clone();
    if let Some(ref b) = d.numeration {
        s.push(' ');
        s.push_str(b);
    }
    if let Some(ref c) = d.titles {
        s.push(' ');
        s.push_str(c);
    }
    if let Some(ref dates) = d.dates {
        s.push_str(", ");
        s.push_str(dates);
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
        s.push(' ');
        s.push_str(c);
    }
    if let Some(ref date) = d.date {
        s.push(' ');
        s.push_str(date);
    }
    s
}

fn meeting_display_name(d: &MeetingNameData) -> String {
    let mut s = d.name.clone();
    if let Some(ref c) = d.location {
        s.push(' ');
        s.push_str(c);
    }
    if let Some(ref date) = d.date {
        s.push(' ');
        s.push_str(date);
    }
    if let Some(ref e) = d.subordinate_unit {
        s.push_str(". ");
        s.push_str(e);
    }
    if let Some(ref n) = d.number {
        s.push(' ');
        s.push_str(n);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_surname_comma_forename() {
        let (last, first) = split_personal_name("Dupont, Jean,", &PersonalNameType::Surname);
        assert_eq!(last.as_deref(), Some("Dupont"));
        assert_eq!(first.as_deref(), Some("Jean"));
    }

    #[test]
    fn split_surname_no_comma() {
        let (last, first) = split_personal_name("Voltaire", &PersonalNameType::Surname);
        assert_eq!(last.as_deref(), Some("Voltaire"));
        assert_eq!(first, None);
    }

    #[test]
    fn split_forename_only() {
        let (last, first) = split_personal_name("Jean,", &PersonalNameType::Forename);
        assert_eq!(last, None);
        assert_eq!(first.as_deref(), Some("Jean"));
    }

    #[test]
    fn split_family_name() {
        let (last, first) =
            split_personal_name("La Rochefoucauld.", &PersonalNameType::FamilyName);
        assert_eq!(last.as_deref(), Some("La Rochefoucauld"));
        assert_eq!(first, None);
    }

    #[test]
    fn personal_author_with_function() {
        let d = PersonalNameData {
            name_type: PersonalNameType::Surname,
            name: "Martin, Pierre,".to_string(),
            numeration: None,
            titles: None,
            dates: Some("1980-".to_string()),
            relator_term: Some("auteur".to_string()),
            fuller_form: None,
            relator_code: Some("070".to_string()),
            authority_number: None,
            dates_of_work: None,
            other_subfields: vec![],
        };
        let a = author_personal(&d);
        assert_eq!(a.kind, AuthorKind::Personal);
        assert_eq!(a.last_name.as_deref(), Some("Martin"));
        assert_eq!(a.first_name.as_deref(), Some("Pierre"));
        assert_eq!(a.dates.as_deref(), Some("1980-"));
        assert_eq!(a.function.as_deref(), Some("auteur"));
        assert_eq!(a.function_code.as_deref(), Some("070"));
    }

    #[test]
    fn corporate_author() {
        let d = CorporateNameData {
            name_type: crate::datatypes::OrganizationNameType::DirectOrder,
            name: "Acme Corp.".to_string(),
            subordinate_unit: Some("Research Division".to_string()),
            location: None,
            date: None,
            relator_term: Some("éditeur".to_string()),
            relator_code: None,
            other_subfields: vec![],
        };
        let a = author_corporate(&d);
        assert_eq!(a.kind, AuthorKind::Corporate);
        assert_eq!(a.display_name, "Acme Corp.. Research Division");
        assert_eq!(a.last_name, None);
        assert_eq!(a.first_name, None);
        assert_eq!(a.function.as_deref(), Some("éditeur"));
    }
}
