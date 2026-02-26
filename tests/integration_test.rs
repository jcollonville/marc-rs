use marc_rs::*;

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
        record_status: 'n',
        record_type: 'a',
        bibliographic_level: 'm',
        type_of_control: ' ',
        character_coding_scheme: ' ',
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 24,
        encoding_level: ' ',
        descriptive_cataloging_form: ' ',
        multipart_resource_record_level: ' ',
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: ' ',
    };

    let mut record = Record::new(leader);
    record.push_control(Control::ControlNumber("12345".to_string()));
    record.push_title(Title::TitleStatement(TitleStatementData {
        ind1: '1',
        ind2: '0',
        title: "Test title".to_string(),
        remainder: None,
        responsibility: None,
        medium: None,
        number_of_part: None,
        name_of_part: None,
        other_subfields: vec![],
    }));

    assert_eq!(record.control().len(), 1);
    assert_eq!(record.titles().len(), 1);
}

#[test]
fn test_leader_to_from_bytes() {
    let leader = Leader {
        record_length: 12345,
        record_status: 'n',
        record_type: 'a',
        bibliographic_level: 'm',
        type_of_control: ' ',
        character_coding_scheme: ' ',
        indicator_count: 2,
        subfield_code_count: 2,
        base_address_of_data: 6789,
        encoding_level: ' ',
        descriptive_cataloging_form: ' ',
        multipart_resource_record_level: ' ',
        length_of_length_of_field_portion: 4,
        length_of_starting_character_position_portion: 5,
        length_of_implementation_defined_portion: 0,
        undefined: ' ',
    };

    let bytes = leader.to_bytes();
    assert_eq!(bytes.len(), 24);

    let parsed = Leader::from_bytes(&bytes).unwrap();
    assert_eq!(parsed.record_length, leader.record_length);
    assert_eq!(parsed.base_address_of_data, leader.base_address_of_data);
}

#[test]
fn test_field_tag_mappings() {
    // MARC21
    let me = MainEntry::PersonalName(PersonalNameData {
        ind1: '1',
        ind2: ' ',
        name: "Test".to_string(),
        numeration: None,
        titles: None,
        dates: None,
        relator_term: None,
        fuller_form: None,
        relator_code: None,
        other_subfields: vec![],
    });
    assert_eq!(me.tag(MarcFormat::Marc21), "100");
    assert_eq!(me.tag(MarcFormat::Unimarc), "700");

    let ts = Title::TitleStatement(TitleStatementData {
        ind1: '1',
        ind2: '0',
        title: "Test".to_string(),
        remainder: None,
        responsibility: None,
        medium: None,
        number_of_part: None,
        name_of_part: None,
        other_subfields: vec![],
    });
    assert_eq!(ts.tag(MarcFormat::Marc21), "245");
    assert_eq!(ts.tag(MarcFormat::Unimarc), "200");

    assert_eq!(
        Control::ControlNumber("x".to_string()).tag(MarcFormat::Marc21),
        Some("001")
    );
}

#[test]
fn test_title_try_parse_and_to_raw() {
    let subfields = vec![
        ('a', "Main title".to_string()),
        ('b', "subtitle".to_string()),
        ('c', "Author".to_string()),
    ];

    let title =
        Title::try_parse("245", '1', '0', &subfields, MarcFormat::Marc21).expect("should parse");
    match &title {
        Title::TitleStatement(d) => {
            assert_eq!(d.title, "Main title");
            assert_eq!(d.remainder.as_deref(), Some("subtitle"));
            assert_eq!(d.responsibility.as_deref(), Some("Author"));
        }
        _ => panic!("Expected TitleStatement"),
    }

    let raw = title.to_raw(MarcFormat::Marc21);
    assert_eq!(raw.tag, "245");
    assert_eq!(raw.ind1, '1');
    assert_eq!(raw.ind2, '0');
    assert_eq!(raw.subfields.len(), 3);
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
