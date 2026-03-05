use marc_rs::*;
use marc_rs::datatypes::title::{Title, TitleStatementData};
use marc_rs::datatypes::{PersonalNameData, PersonalNameType};

#[test]
fn test_parse_empty() {
    let data = b"";
    let format_encoding = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = parse(data, format_encoding);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_format_encoding_defaults() {
    let marc21 = FormatEncoding::marc21_default();
    assert_eq!(marc21.format, MarcFormat::Marc21);
    assert_eq!(marc21.encoding, Encoding::Marc8);

    let unimarc = FormatEncoding::unimarc_default();
    assert_eq!(unimarc.format, MarcFormat::Unimarc);
    assert_eq!(unimarc.encoding, Encoding::Utf8);

    let xml = FormatEncoding::marc_xml();
    assert_eq!(xml.format, MarcFormat::MarcXml);
    assert_eq!(xml.encoding, Encoding::Utf8);
}

#[test]
fn test_record_creation() {
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

    let mut record = Record::new(None, leader);
    record.identification.record_identifier = Some("12345".to_string());
    record.description.title_statement = Some(Title::TitleStatement(TitleStatementData {
        title_added_entry: true,
        nonfiling_chars: 0,
        title: "Test title".to_string(),
        remainder: None,
        responsibility: None,
        other_title_info: None,
        first_responsibility: None,
        other_responsibility: None,
        medium: None,
        number_of_part: None,
        name_of_part: None,
        other_subfields: vec![],
    }));

    assert_eq!(record.record_id(), Some("12345"));
    assert!(record.title().is_some());
    assert_eq!(record.title(), Some("Test title"));
}

#[test]
fn test_leader_to_from_bytes() {
    let leader = Leader {
        record_length: 12345,
        record_status: RecordStatus::New,
        record_type: RecordType::LanguageMaterial,
        bibliographic_level: BibliographicLevel::Monograph,
        type_of_control: TypeOfControl::NoSpecifiedType,
        character_coding_scheme: CharacterCodingScheme::Marc8OrUnspecified,
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 6789,
        encoding_level: EncodingLevel::Unknown(' '),
        descriptive_cataloging_form: DescriptiveCatalogingForm::NonIsbd,
        multipart_resource_record_level: MultipartResourceRecordLevel::NotSpecifiedOrNotApplicable,
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: LeaderUndefined::Blank,
    };

    let bytes = leader.to_bytes();
    assert_eq!(bytes.len(), 24);

    let parsed = Leader::from_bytes(&bytes).unwrap();
    assert_eq!(parsed.record_length, leader.record_length);
    assert_eq!(parsed.base_address_of_data, leader.base_address_of_data);
}

#[test]
fn test_field_tag_mappings() {
    // Test MARC tag dispatch via FormatDescriptor
    use marc_rs::formats::{MARC21_FORMAT, UNIMARC_FORMAT, FormatDescriptor};

    let marc21_title = MARC21_FORMAT.tag_descriptor("245");
    assert!(marc21_title.is_some());
    assert_eq!(marc21_title.unwrap().field, "title_statement");

    let unimarc_title = UNIMARC_FORMAT.tag_descriptor("200");
    assert!(unimarc_title.is_some());
    assert_eq!(unimarc_title.unwrap().field, "title_statement");

    let marc21_001 = MARC21_FORMAT.tag_descriptor("001");
    assert!(marc21_001.is_some());
    assert_eq!(marc21_001.unwrap().field, "record_identifier");
}

#[test]
fn test_title_dispatch_and_read() {
    let mut record = Record::default();
    let descriptor = &marc_rs::formats::MARC21_FORMAT;

    let subfields = vec![
        ('a', "Main title".to_string()),
        ('b', "subtitle".to_string()),
        ('c', "Author".to_string()),
    ];

    record.dispatch_data_field("245", '1', '0', &subfields, descriptor);

    assert_eq!(record.title(), Some("Main title"));
}

#[test]
fn test_encoding_conversion() {
    let text = "Hello, World!";
    let utf8_bytes = text.as_bytes().to_vec();

    let result = convert_from_encoding(text, Encoding::Utf8).unwrap();
    assert_eq!(result, utf8_bytes);

    let converted = convert_to_utf8(&utf8_bytes, Encoding::Utf8).unwrap();
    assert_eq!(converted, text);
}
