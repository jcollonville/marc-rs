use marc_rs::*;
use marc_rs::datatypes::title::{Title, TitleStatementData};
use marc_rs::datatypes::{PersonalNameData, PersonalNameType, SubjectData, SubjectThesaurus};

fn make_leader() -> Leader {
    Leader {
        record_length: 100,
        record_status: RecordStatus::New,
        record_type: RecordType::LanguageMaterial,
        bibliographic_level: BibliographicLevel::Monograph,
        type_of_control: TypeOfControl::NoSpecifiedType,
        character_coding_scheme: CharacterCodingScheme::Marc8OrUnspecified,
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 24,
        encoding_level: EncodingLevel::Unknown(' '),
        descriptive_cataloging_form: DescriptiveCatalogingForm::NonIsbd,
        multipart_resource_record_level: MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: LeaderUndefined::Blank,
    }
}

fn record_with_id(id: &str) -> Record {
    let mut record = Record::new(None, make_leader());
    record.identification.record_identifier = Some(id.to_string());
    record
}

#[test]
fn test_helpers_from_slice() {
    let data = b"";
    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = helpers::from_slice(data, format);
    assert!(result.is_err());
}

#[test]
fn test_helpers_to_vec() {
    let record = record_with_id("12345");
    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = helpers::to_vec(&record, format);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_helpers_to_string_xml() {
    let record = record_with_id("12345");
    let format = FormatEncoding::marc_xml();
    let result = helpers::to_string(&record, format);
    assert!(result.is_ok());
    let xml = result.unwrap();
    assert!(xml.contains("<record"));
    assert!(xml.contains("001"));
}

#[test]
fn test_helpers_from_str_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<record xmlns="http://www.loc.gov/MARC21/slim">
  <leader>00000nam a2200000 a 4500</leader>
  <controlfield tag="001">12345</controlfield>
</record>"#;

    let format = FormatEncoding::marc_xml();
    let result = helpers::from_str(xml, format);
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.record_id(), Some("12345"));
}

#[test]
fn test_helpers_to_writer() {
    let record = record_with_id("12345");
    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let mut buffer = Vec::new();
    let result = helpers::to_writer(&record, format, &mut buffer);
    assert!(result.is_ok());
    assert!(!buffer.is_empty());
}

#[test]
fn test_helpers_to_records() {
    let r1 = record_with_id("12345");
    let r2 = record_with_id("67890");
    let records = vec![r1, r2];

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = helpers::to_records(&records, format);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_json_round_trip() {
    let leader = Leader {
        record_length: 100,
        record_status: RecordStatus::New,
        record_type: RecordType::LanguageMaterial,
        bibliographic_level: BibliographicLevel::Monograph,
        type_of_control: TypeOfControl::NoSpecifiedType,
        character_coding_scheme: CharacterCodingScheme::Utf8,
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 24,
        encoding_level: EncodingLevel::Full,
        descriptive_cataloging_form: DescriptiveCatalogingForm::NonIsbd,
        multipart_resource_record_level: MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: LeaderUndefined::Unknown('0'),
    };

    let mut record = Record::new(None, leader);

    // Set record identifier
    record.identification.record_identifier = Some("ocm12345".to_string());

    // Set title statement
    record.description.title_statement = Some(Title::TitleStatement(TitleStatementData {
        title_added_entry: false,
        nonfiling_chars: 4,
        title: "The Rust Programming Language".to_string(),
        remainder: Some("a comprehensive guide".to_string()),
        responsibility: Some("Steve Klabnik and Carol Nichols".to_string()),
        other_title_info: None,
        first_responsibility: None,
        other_responsibility: None,
        medium: None,
        number_of_part: None,
        name_of_part: None,
        other_subfields: vec![],
    }));

    // Set main entry
    record.intellectual_responsibility.main_entry_personal_name = Some(PersonalNameData {
        name_type: PersonalNameType::Surname,
        name: "Klabnik, Steve".to_string(),
        numeration: None,
        titles: None,
        dates: Some("1988-".to_string()),
        relator_term: Some("author".to_string()),
        fuller_form: None,
        relator_code: None,
        authority_number: None,
        dates_of_work: None,
        other_subfields: vec![],
    });

    // Add subject
    use marc_rs::blocks::subject_analysis::TaggedSubject;
    record.subject_analysis.subjects.push(TaggedSubject {
        tag: "650".to_string(),
        data: SubjectData {
            thesaurus: SubjectThesaurus::Lcsh,
            term: "Rust (Computer program language)".to_string(),
            name_subdivision: None,
            form_subdivision: None,
            general_subdivision: None,
            chronological_subdivision: None,
            geographic_subdivision: None,
            source: None,
            authority_number: None,
            other_subfields: vec![('x', "Handbooks, manuals, etc.".to_string())],
        },
    });

    let json = serde_json::to_string(&record).expect("Serialize to JSON");
    let deserialized: Record = serde_json::from_str(&json).expect("Deserialize from JSON");

    assert_eq!(record, deserialized);
}
