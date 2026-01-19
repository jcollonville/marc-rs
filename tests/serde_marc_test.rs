#[cfg(feature = "serde")]
use marc_rs::*;

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_from_slice() {
    // This is a minimal test - in practice you'd use real MARC data
    let data = b"";
    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = serde_marc::from_slice(data, format);
    // Empty data should return an error
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_to_vec() {
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
        data_fields: vec![],
    };

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = serde_marc::to_vec(&record, format);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_to_string_xml() {
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
        data_fields: vec![],
    };

    let format = FormatEncoding::marc_xml();
    let result = serde_marc::to_string(&record, format);
    assert!(result.is_ok());
    let xml = result.unwrap();
    assert!(xml.contains("<record"));
    assert!(xml.contains("001"));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_from_str_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<record xmlns="http://www.loc.gov/MARC21/slim">
  <leader>00000nam a2200000 a 4500</leader>
  <controlfield tag="001">12345</controlfield>
</record>"#;

    let format = FormatEncoding::marc_xml();
    let result = serde_marc::from_str(xml, format);
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.control_fields.len(), 1);
    assert_eq!(record.control_fields[0].tag, "001");
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_to_writer() {
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
        data_fields: vec![],
    };

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let mut buffer = Vec::new();
    let result = serde_marc::to_writer(&record, format, &mut buffer);
    assert!(result.is_ok());
    assert!(!buffer.is_empty());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_marc_to_records() {
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

    let records = vec![
        Record {
            leader: leader.clone(),
            control_fields: vec![ControlField {
                tag: "001".to_string(),
                value: "12345".to_string(),
            }],
            data_fields: vec![],
        },
        Record {
            leader,
            control_fields: vec![ControlField {
                tag: "001".to_string(),
                value: "67890".to_string(),
            }],
            data_fields: vec![],
        },
    ];

    let format = FormatEncoding::new(MarcFormat::Marc21, Encoding::Marc8);
    let result = serde_marc::to_records(&records, format);
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}
