use marc_rs::*;

#[test]
fn test_helpers_from_slice() {
    let data = b"";
    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = helpers::from_slice(data, format);
    assert!(result.is_err());
}

#[test]
fn test_helpers_to_vec() {
    let leader = Leader {
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
    };

    let mut record = Record::new(leader);
    record.push_control(Control::ControlNumber("12345".to_string()));

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = helpers::to_vec(&record, format);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn test_helpers_to_string_xml() {
    let leader = Leader {
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
    };

    let mut record = Record::new(leader);
    record.push_control(Control::ControlNumber("12345".to_string()));

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
    assert_eq!(record.control().len(), 1);
    match &record.control()[0] {
        Control::ControlNumber(v) => assert_eq!(v, "12345"),
        _ => panic!("Expected ControlNumber"),
    }
}

#[test]
fn test_helpers_to_writer() {
    let leader = Leader {
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
    };

    let mut record = Record::new(leader);
    record.push_control(Control::ControlNumber("12345".to_string()));

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let mut buffer = Vec::new();
    let result = helpers::to_writer(&record, format, &mut buffer);
    assert!(result.is_ok());
    assert!(!buffer.is_empty());
}

#[test]
fn test_helpers_to_records() {
    let leader = Leader {
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
    };

    let mut r1 = Record::new(leader.clone());
    r1.push_control(Control::ControlNumber("12345".to_string()));
    let mut r2 = Record::new(leader);
    r2.push_control(Control::ControlNumber("67890".to_string()));
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

    let mut record = Record::new(leader);
    record.push_control(Control::ControlNumber("ocm12345".to_string()));
    record.push_control(Control::FixedLengthDataElements("some-fixed-data".to_string()));
    record.push_title(Title::TitleStatement(
        marc_rs::fields::title::TitleStatementData {
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
        },
    ));
    record.push_main_entry(MainEntry::PersonalName(
        marc_rs::fields::common::PersonalNameData {
            name_type: marc_rs::fields::common::PersonalNameType::Surname,
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
        },
    ));
    record.push_subject(Subject::SubjectTopicalTerm(
        marc_rs::fields::common::SubjectData {
            thesaurus: marc_rs::fields::common::SubjectThesaurus::Lcsh,
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
    ));
    record.push_other_data(DataField {
        tag: "999".to_string(),
        ind1: ' ',
        ind2: ' ',
        subfields: vec![Subfield {
            code: 'a',
            value: "local-data".to_string(),
        }],
    });

    let json = serde_json::to_string(&record).expect("Serialize to JSON");
    let deserialized: Record = serde_json::from_str(&json).expect("Deserialize from JSON");

    assert_eq!(record, deserialized);
}
