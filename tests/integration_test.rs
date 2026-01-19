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

    let record = Record {
        leader,
        control_fields: vec![ControlField {
            tag: "001".to_string(),
            value: "12345".to_string(),
        }],
        data_fields: vec![DataField {
            tag: "245".to_string(),
            ind1: '1',
            ind2: '0',
            subfields: vec![Subfield {
                code: 'a',
                value: "Test title".to_string(),
            }],
        }],
    };

    assert_eq!(record.control_fields.len(), 1);
    assert_eq!(record.data_fields.len(), 1);
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
fn test_field_enums() {
    use marc_rs::MarcFormat;

    // Test MARC21 tags
    assert_eq!(MainEntry::PersonalName.tag(MarcFormat::Marc21), "100");
    assert_eq!(Title::TitleStatement.tag(MarcFormat::Marc21), "245");
    assert_eq!(Edition::EditionStatement.tag(MarcFormat::Marc21), Some("250"));
    assert_eq!(Physical::PhysicalDescription.tag(MarcFormat::Marc21), Some("300"));
    assert_eq!(Series::SeriesStatement.tag(MarcFormat::Marc21), "490");
    assert_eq!(Note::GeneralNote.tag(MarcFormat::Marc21), "500");
    assert_eq!(Subject::SubjectTopicalTerm.tag(MarcFormat::Marc21), Some("650"));
    assert_eq!(AddedEntry::PersonalName.tag(MarcFormat::Marc21), "700");
    assert_eq!(Linking::MainSeriesEntry.tag(MarcFormat::Marc21), Some("760"));
    assert_eq!(Control::ControlNumber.tag(MarcFormat::Marc21), Some("001"));

    // Test UNIMARC tags
    assert_eq!(MainEntry::PersonalName.tag(MarcFormat::Unimarc), "700");
    assert_eq!(Title::TitleStatement.tag(MarcFormat::Unimarc), "200");
    assert_eq!(Edition::EditionStatement.tag(MarcFormat::Unimarc), Some("205"));
    assert_eq!(Physical::PhysicalDescription.tag(MarcFormat::Unimarc), Some("215"));
    assert_eq!(Series::SeriesStatement.tag(MarcFormat::Unimarc), "225");
    assert_eq!(Subject::SubjectTopicalTerm.tag(MarcFormat::Unimarc), Some("606"));
}

#[test]
fn test_encoding_conversion() {
    let text = "Hello, World!";
    let utf8_bytes = text.as_bytes().to_vec();

    // UTF-8 to UTF-8 should be identity
    let result = convert_from_encoding(text, Encoding::Utf8).unwrap();
    assert_eq!(result, utf8_bytes);

    // UTF-8 to UTF-8 conversion should be identity
    let converted = convert_to_utf8(&utf8_bytes, Encoding::Utf8).unwrap();
    assert_eq!(converted, text);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_serialization() {
    use serde_json;

    let record = Record {
        leader: Leader {
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
        },
        control_fields: vec![ControlField {
            tag: "001".to_string(),
            value: "12345".to_string(),
        }],
        data_fields: vec![],
    };

    let json = serde_json::to_string(&record).unwrap();
    let deserialized: Record = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.control_fields[0].tag, record.control_fields[0].tag);
}
