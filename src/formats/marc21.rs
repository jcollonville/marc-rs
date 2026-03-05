use crate::datatypes::{PersonalNameType, OrganizationNameType};
use crate::formats::{BlockId, FieldType, SubfieldMapping, TagDescriptor, FormatDescriptor};

/// Singleton instance for the MARC 21 semantic format.
pub struct Marc21Format;

pub static MARC21_FORMAT: Marc21Format = Marc21Format;

// ── Shared subfield mappings ──────────────────────────────────────────────

const SF_A: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "value", repeatable: false },
];

const SF_NOTE: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "text", repeatable: false },
    SubfieldMapping { code: 'u', name: "uri", repeatable: false },
];

const SF_LINKING: &[SubfieldMapping] = &[
    SubfieldMapping { code: 't', name: "title", repeatable: false },
    SubfieldMapping { code: 'v', name: "volume", repeatable: false },
    SubfieldMapping { code: 'w', name: "record_control_number", repeatable: false },
    SubfieldMapping { code: 'x', name: "issn", repeatable: false },
    SubfieldMapping { code: 'z', name: "isbn", repeatable: false },
];

const SF_PERSONAL_NAME: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "name", repeatable: false },
    SubfieldMapping { code: 'b', name: "numeration", repeatable: false },
    SubfieldMapping { code: 'c', name: "titles", repeatable: false },
    SubfieldMapping { code: 'd', name: "dates", repeatable: false },
    SubfieldMapping { code: 'e', name: "relator_term", repeatable: false },
    SubfieldMapping { code: 'f', name: "dates_of_work", repeatable: false },
    SubfieldMapping { code: 'q', name: "fuller_form", repeatable: false },
    SubfieldMapping { code: 'u', name: "affiliation", repeatable: false },
    SubfieldMapping { code: '4', name: "relator_code", repeatable: true },
];

const SF_CORPORATE_NAME: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "name", repeatable: false },
    SubfieldMapping { code: 'b', name: "subordinate_unit", repeatable: true },
    SubfieldMapping { code: 'c', name: "location", repeatable: false },
    SubfieldMapping { code: 'd', name: "date", repeatable: true },
    SubfieldMapping { code: 'e', name: "relator_term", repeatable: false },
    SubfieldMapping { code: '4', name: "relator_code", repeatable: true },
];

const SF_UNIFORM_TITLE: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "title", repeatable: false },
    SubfieldMapping { code: 'd', name: "date", repeatable: false },
    SubfieldMapping { code: 'f', name: "date_of_work", repeatable: false },
    SubfieldMapping { code: 'l', name: "language", repeatable: false },
    SubfieldMapping { code: 'n', name: "number", repeatable: false },
    SubfieldMapping { code: 'p', name: "name_of_part", repeatable: false },
];

const SF_SUBJECT: &[SubfieldMapping] = &[
    SubfieldMapping { code: 'a', name: "term", repeatable: false },
    SubfieldMapping { code: 'b', name: "form_subdivision", repeatable: true },
    SubfieldMapping { code: 'v', name: "general_subdivision", repeatable: true },
    SubfieldMapping { code: 'x', name: "general_subdivision_2", repeatable: true },
    SubfieldMapping { code: 'y', name: "chronological_subdivision", repeatable: true },
    SubfieldMapping { code: 'z', name: "geographic_subdivision", repeatable: true },
    SubfieldMapping { code: '2', name: "source", repeatable: false },
];

pub const MARC21_DESCRIPTORS: &[TagDescriptor] = &[
    // ── Bloc 0 – Identification ──────────────────────────────────────────
    TagDescriptor { tag: "001", block: BlockId::Identification, field: "record_identifier",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "003", block: BlockId::Identification, field: "agency_identifier",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "005", block: BlockId::Identification, field: "record_version_date",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "010", block: BlockId::Identification, field: "lccn",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "015", block: BlockId::Identification, field: "national_bibliography_number",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "value", repeatable: true },
            SubfieldMapping { code: '2', name: "source", repeatable: false },
        ] },
    TagDescriptor { tag: "016", block: BlockId::Identification, field: "national_library_record_number",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "017", block: BlockId::Identification, field: "legal_deposit_number",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "value", repeatable: false },
            SubfieldMapping { code: 'b', name: "assigning_agency", repeatable: false },
        ] },
    TagDescriptor { tag: "020", block: BlockId::Identification, field: "isbn",
        field_type: FieldType::Isbn, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "value", repeatable: false },
            SubfieldMapping { code: 'q', name: "qualification", repeatable: false },
            SubfieldMapping { code: 'c', name: "price_or_acquisition", repeatable: false },
            SubfieldMapping { code: 'z', name: "canceled_invalid", repeatable: true },
        ] },
    TagDescriptor { tag: "022", block: BlockId::Identification, field: "issn",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "024", block: BlockId::Identification, field: "other_standard_identifier",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "027", block: BlockId::Identification, field: "strn",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "028", block: BlockId::Identification, field: "publisher_number",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "030", block: BlockId::Identification, field: "coden",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "035", block: BlockId::Identification, field: "system_control_number",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "086", block: BlockId::Identification, field: "government_document_classification",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },

    // ── Bloc 1 – Informations codées ─────────────────────────────────────
    TagDescriptor { tag: "006", block: BlockId::CodedInformation, field: "additional_fixed_length_elements",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "007", block: BlockId::CodedInformation, field: "physical_description_fixed_field",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "008", block: BlockId::CodedInformation, field: "fixed_length_data_elements",
        field_type: FieldType::SimpleString, is_control: true, subfield_map: &[] },
    TagDescriptor { tag: "034", block: BlockId::CodedInformation, field: "coded_cartographic_math_data",
        field_type: FieldType::GenericDataField, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "041", block: BlockId::CodedInformation, field: "language_of_resource",
        field_type: FieldType::Language, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "language_code", repeatable: true },
            SubfieldMapping { code: 'b', name: "language_of_summary", repeatable: true },
        ] },
    TagDescriptor { tag: "044", block: BlockId::CodedInformation, field: "country_of_publication",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "045", block: BlockId::CodedInformation, field: "time_period_of_content",
        field_type: FieldType::GenericDataField, is_control: false, subfield_map: SF_A },

    // ── Bloc 2 – Description ─────────────────────────────────────────────
    TagDescriptor { tag: "240", block: BlockId::Description, field: "uniform_title",
        field_type: FieldType::UniformTitle, is_control: false, subfield_map: SF_UNIFORM_TITLE },
    TagDescriptor { tag: "245", block: BlockId::Description, field: "title_statement",
        field_type: FieldType::TitleStatement, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "title", repeatable: false },
            SubfieldMapping { code: 'b', name: "remainder", repeatable: false },
            SubfieldMapping { code: 'c', name: "responsibility", repeatable: false },
            SubfieldMapping { code: 'h', name: "medium", repeatable: false },
            SubfieldMapping { code: 'n', name: "number_of_part", repeatable: false },
            SubfieldMapping { code: 'p', name: "name_of_part", repeatable: false },
        ] },
    TagDescriptor { tag: "250", block: BlockId::Description, field: "edition_statement",
        field_type: FieldType::EditionStatement, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "edition", repeatable: false },
            SubfieldMapping { code: 'b', name: "remainder", repeatable: false },
        ] },
    TagDescriptor { tag: "254", block: BlockId::Description, field: "musical_presentation_statement",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "255", block: BlockId::Description, field: "cartographic_mathematical_data",
        field_type: FieldType::GenericDataField, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "256", block: BlockId::Description, field: "computer_file_characteristics",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "257", block: BlockId::Description, field: "country_of_producing_entity",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "258", block: BlockId::Description, field: "philatelic_issue_data",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "260", block: BlockId::Description, field: "publication_distribution_imprint",
        field_type: FieldType::Publication, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "place", repeatable: true },
            SubfieldMapping { code: 'b', name: "publisher", repeatable: true },
            SubfieldMapping { code: 'c', name: "date", repeatable: true },
        ] },
    TagDescriptor { tag: "264", block: BlockId::Description, field: "publication_distribution_imprint_rda",
        field_type: FieldType::Publication, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "place", repeatable: true },
            SubfieldMapping { code: 'b', name: "publisher", repeatable: true },
            SubfieldMapping { code: 'c', name: "date", repeatable: true },
        ] },
    TagDescriptor { tag: "300", block: BlockId::Description, field: "physical_description",
        field_type: FieldType::PhysicalDescription, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "extent", repeatable: false },
            SubfieldMapping { code: 'b', name: "other_physical_details", repeatable: false },
            SubfieldMapping { code: 'c', name: "dimensions", repeatable: false },
            SubfieldMapping { code: 'e', name: "accompanying_material", repeatable: false },
        ] },
    TagDescriptor { tag: "362", block: BlockId::Description, field: "dates_of_publication",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "490", block: BlockId::Description, field: "series_statement",
        field_type: FieldType::SeriesStatement, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "statement", repeatable: false },
            SubfieldMapping { code: 'v', name: "volume", repeatable: false },
            SubfieldMapping { code: 'x', name: "issn", repeatable: false },
        ] },

    // ── Bloc 3 – Notes ───────────────────────────────────────────────────
    TagDescriptor { tag: "500", block: BlockId::Notes, field: "general_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "501", block: BlockId::Notes, field: "with_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "502", block: BlockId::Notes, field: "dissertation_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "504", block: BlockId::Notes, field: "bibliography_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "505", block: BlockId::Notes, field: "formatted_contents_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "506", block: BlockId::Notes, field: "restrictions_on_access_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "507", block: BlockId::Notes, field: "scale_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "508", block: BlockId::Notes, field: "creation_production_credits_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "510", block: BlockId::Notes, field: "citation_references_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "511", block: BlockId::Notes, field: "participant_or_performer_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "513", block: BlockId::Notes, field: "type_of_report_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "514", block: BlockId::Notes, field: "data_quality_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "515", block: BlockId::Notes, field: "numbering_peculiarities_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "516", block: BlockId::Notes, field: "type_of_computer_file_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "518", block: BlockId::Notes, field: "date_time_place_of_event_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "520", block: BlockId::Notes, field: "summary_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "521", block: BlockId::Notes, field: "target_audience_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "522", block: BlockId::Notes, field: "geographic_coverage_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "524", block: BlockId::Notes, field: "preferred_citation_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "525", block: BlockId::Notes, field: "supplement_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "526", block: BlockId::Notes, field: "study_program_info_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "530", block: BlockId::Notes, field: "additional_physical_form_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "533", block: BlockId::Notes, field: "reproduction_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "534", block: BlockId::Notes, field: "original_version_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "535", block: BlockId::Notes, field: "location_originals_duplicates_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "536", block: BlockId::Notes, field: "funding_information_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "538", block: BlockId::Notes, field: "system_details_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "540", block: BlockId::Notes, field: "terms_governing_use_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "541", block: BlockId::Notes, field: "immediate_source_acquisition_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "542", block: BlockId::Notes, field: "information_copyright_status",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "544", block: BlockId::Notes, field: "location_other_archival_materials",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "545", block: BlockId::Notes, field: "biographical_historical_data",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "546", block: BlockId::Notes, field: "language_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "547", block: BlockId::Notes, field: "former_title_complexity_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "550", block: BlockId::Notes, field: "issuing_body_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "552", block: BlockId::Notes, field: "entity_attribute_info_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "555", block: BlockId::Notes, field: "cumulative_index_finding_aids_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "556", block: BlockId::Notes, field: "information_about_documentation_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "561", block: BlockId::Notes, field: "ownership_custodial_history",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "562", block: BlockId::Notes, field: "copy_version_identification_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "563", block: BlockId::Notes, field: "binding_information",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "565", block: BlockId::Notes, field: "case_file_characteristics_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "567", block: BlockId::Notes, field: "methodology_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "580", block: BlockId::Notes, field: "linking_entry_complexity_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "581", block: BlockId::Notes, field: "publications_about_described_materials",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "583", block: BlockId::Notes, field: "action_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "584", block: BlockId::Notes, field: "accumulation_frequency_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "585", block: BlockId::Notes, field: "exhibitions_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "586", block: BlockId::Notes, field: "awards_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },
    TagDescriptor { tag: "588", block: BlockId::Notes, field: "source_of_description_note",
        field_type: FieldType::NoteEntry, is_control: false, subfield_map: SF_NOTE },

    // ── Bloc 4 – Liens ───────────────────────────────────────────────────
    TagDescriptor { tag: "440", block: BlockId::Links, field: "link_series_title",
        field_type: FieldType::SeriesStatement, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "statement", repeatable: false },
            SubfieldMapping { code: 'v', name: "volume", repeatable: false },
            SubfieldMapping { code: 'x', name: "issn", repeatable: false },
        ] },
    TagDescriptor { tag: "760", block: BlockId::Links, field: "link_series",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "762", block: BlockId::Links, field: "link_subseries",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "765", block: BlockId::Links, field: "link_original_language",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "767", block: BlockId::Links, field: "link_translation",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "770", block: BlockId::Links, field: "link_supplement_special_issue",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "772", block: BlockId::Links, field: "link_supplement_parent",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "773", block: BlockId::Links, field: "link_host_item",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "774", block: BlockId::Links, field: "link_constituent_unit",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "775", block: BlockId::Links, field: "link_other_edition",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "776", block: BlockId::Links, field: "link_additional_physical_form",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "777", block: BlockId::Links, field: "link_issued_with",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "780", block: BlockId::Links, field: "link_preceding",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "785", block: BlockId::Links, field: "link_succeeding",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "786", block: BlockId::Links, field: "link_data_source",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },
    TagDescriptor { tag: "787", block: BlockId::Links, field: "link_other_relationship",
        field_type: FieldType::LinkingEntry, is_control: false, subfield_map: SF_LINKING },

    // ── Bloc 5 – Titres associés ─────────────────────────────────────────
    TagDescriptor { tag: "210", block: BlockId::AssociatedTitles, field: "abbreviated_title",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "222", block: BlockId::AssociatedTitles, field: "key_title",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "242", block: BlockId::AssociatedTitles, field: "translation_of_title_by_cataloging_agency",
        field_type: FieldType::TitleStatement, is_control: false, subfield_map: &[
            SubfieldMapping { code: 'a', name: "title", repeatable: false },
        ] },
    TagDescriptor { tag: "246", block: BlockId::AssociatedTitles, field: "varying_form_of_title",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "title", repeatable: false },
            SubfieldMapping { code: 'b', name: "remainder", repeatable: false },
        ] },
    TagDescriptor { tag: "247", block: BlockId::AssociatedTitles, field: "former_title",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "title", repeatable: false },
        ] },
    TagDescriptor { tag: "830", block: BlockId::Links, field: "link_series_uniform_title",
        field_type: FieldType::SeriesStatement, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "statement", repeatable: false },
            SubfieldMapping { code: 'v', name: "volume", repeatable: false },
            SubfieldMapping { code: 'x', name: "issn", repeatable: false },
        ] },

    // ── Bloc 6 – Analyse matière ─────────────────────────────────────────
    TagDescriptor { tag: "043", block: BlockId::SubjectAnalysis, field: "geographic_area_code",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "050", block: BlockId::SubjectAnalysis, field: "lc_classification",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "class_number", repeatable: false },
            SubfieldMapping { code: 'b', name: "item_number", repeatable: false },
        ] },
    TagDescriptor { tag: "082", block: BlockId::SubjectAnalysis, field: "dewey_classification",
        field_type: FieldType::DeweyClassification, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "number", repeatable: true },
            SubfieldMapping { code: 'b', name: "item_number", repeatable: false },
            SubfieldMapping { code: '2', name: "edition", repeatable: false },
        ] },
    TagDescriptor { tag: "083", block: BlockId::SubjectAnalysis, field: "dewey_classification_additional",
        field_type: FieldType::DeweyClassification, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "number", repeatable: true },
            SubfieldMapping { code: 'b', name: "item_number", repeatable: false },
            SubfieldMapping { code: '2', name: "edition", repeatable: false },
        ] },
    TagDescriptor { tag: "084", block: BlockId::SubjectAnalysis, field: "other_classification_number",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "600", block: BlockId::SubjectAnalysis, field: "subject_person_name",
        field_type: FieldType::PersonalName, is_control: false, subfield_map: SF_PERSONAL_NAME },
    TagDescriptor { tag: "610", block: BlockId::SubjectAnalysis, field: "subject_corporate_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "611", block: BlockId::SubjectAnalysis, field: "subject_meeting_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "630", block: BlockId::SubjectAnalysis, field: "subject_uniform_title",
        field_type: FieldType::UniformTitle, is_control: false, subfield_map: SF_UNIFORM_TITLE },
    TagDescriptor { tag: "650", block: BlockId::SubjectAnalysis, field: "subject_topical_term",
        field_type: FieldType::SubjectEntry, is_control: false, subfield_map: SF_SUBJECT },
    TagDescriptor { tag: "651", block: BlockId::SubjectAnalysis, field: "subject_geographic_name",
        field_type: FieldType::SubjectEntry, is_control: false, subfield_map: SF_SUBJECT },
    TagDescriptor { tag: "653", block: BlockId::SubjectAnalysis, field: "index_term_uncontrolled",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "654", block: BlockId::SubjectAnalysis, field: "subject_faceted_topical_terms",
        field_type: FieldType::SubjectEntry, is_control: false, subfield_map: SF_SUBJECT },
    TagDescriptor { tag: "655", block: BlockId::SubjectAnalysis, field: "index_term_genre_form",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "656", block: BlockId::SubjectAnalysis, field: "index_term_occupation",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "657", block: BlockId::SubjectAnalysis, field: "index_term_function",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "658", block: BlockId::SubjectAnalysis, field: "index_term_curriculum_objective",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "662", block: BlockId::SubjectAnalysis, field: "subject_hierarchical_place_name",
        field_type: FieldType::SubjectEntry, is_control: false, subfield_map: SF_SUBJECT },
    TagDescriptor { tag: "688", block: BlockId::SubjectAnalysis, field: "subject_type_of_entity_unspecified",
        field_type: FieldType::SubjectEntry, is_control: false, subfield_map: SF_SUBJECT },
    TagDescriptor { tag: "690", block: BlockId::SubjectAnalysis, field: "local_subject_heading",
        field_type: FieldType::StringList, is_control: false, subfield_map: SF_A },

    // ── Bloc 7 – Responsabilité intellectuelle ───────────────────────────
    TagDescriptor { tag: "100", block: BlockId::IntellectualResponsibility, field: "main_entry_personal_name",
        field_type: FieldType::PersonalName, is_control: false, subfield_map: SF_PERSONAL_NAME },
    TagDescriptor { tag: "110", block: BlockId::IntellectualResponsibility, field: "main_entry_corporate_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "111", block: BlockId::IntellectualResponsibility, field: "main_entry_meeting_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "130", block: BlockId::IntellectualResponsibility, field: "main_entry_uniform_title",
        field_type: FieldType::UniformTitle, is_control: false, subfield_map: SF_UNIFORM_TITLE },
    TagDescriptor { tag: "700", block: BlockId::IntellectualResponsibility, field: "added_entry_personal_name",
        field_type: FieldType::PersonalName, is_control: false, subfield_map: SF_PERSONAL_NAME },
    TagDescriptor { tag: "710", block: BlockId::IntellectualResponsibility, field: "added_entry_corporate_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "711", block: BlockId::IntellectualResponsibility, field: "added_entry_meeting_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "720", block: BlockId::IntellectualResponsibility, field: "added_entry_uncontrolled_name",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "730", block: BlockId::IntellectualResponsibility, field: "added_entry_uniform_title",
        field_type: FieldType::UniformTitle, is_control: false, subfield_map: SF_UNIFORM_TITLE },
    TagDescriptor { tag: "740", block: BlockId::IntellectualResponsibility, field: "added_entry_uncontrolled_related_title",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "751", block: BlockId::IntellectualResponsibility, field: "added_entry_geographic_name",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "752", block: BlockId::IntellectualResponsibility, field: "added_entry_hierarchical_place",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "753", block: BlockId::IntellectualResponsibility, field: "system_details_access",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "754", block: BlockId::IntellectualResponsibility, field: "taxonomic_identification",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "800", block: BlockId::IntellectualResponsibility, field: "series_added_entry_personal_name",
        field_type: FieldType::PersonalName, is_control: false, subfield_map: SF_PERSONAL_NAME },
    TagDescriptor { tag: "810", block: BlockId::IntellectualResponsibility, field: "series_added_entry_corporate_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },
    TagDescriptor { tag: "811", block: BlockId::IntellectualResponsibility, field: "series_added_entry_meeting_name",
        field_type: FieldType::CorporateName, is_control: false, subfield_map: SF_CORPORATE_NAME },

    // ── Bloc 8 – Usage international ────────────────────────────────────
    TagDescriptor { tag: "040", block: BlockId::InternationalUse, field: "cataloging_source",
        field_type: FieldType::GenericDataField, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "original_cataloging_agency", repeatable: false },
            SubfieldMapping { code: 'b', name: "language_of_cataloging", repeatable: false },
            SubfieldMapping { code: 'c', name: "transcribing_agency", repeatable: false },
            SubfieldMapping { code: 'd', name: "modifying_agency", repeatable: true },
            SubfieldMapping { code: 'e', name: "description_conventions", repeatable: false },
        ] },
    TagDescriptor { tag: "850", block: BlockId::InternationalUse, field: "holding_institution",
        field_type: FieldType::SimpleString, is_control: false, subfield_map: SF_A },
    TagDescriptor { tag: "852", block: BlockId::InternationalUse, field: "location_call_number",
        field_type: FieldType::Specimen, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'a', name: "library", repeatable: false },
            SubfieldMapping { code: 'b', name: "section", repeatable: false },
            SubfieldMapping { code: 'p', name: "barcode", repeatable: false },
            SubfieldMapping { code: 'h', name: "call_number", repeatable: false },
        ] },
    TagDescriptor { tag: "856", block: BlockId::InternationalUse, field: "electronic_location_access",
        field_type: FieldType::SimpleString, is_control: false,
        subfield_map: &[
            SubfieldMapping { code: 'u', name: "uri", repeatable: false },
            SubfieldMapping { code: 'z', name: "public_note", repeatable: false },
        ] },

    // ── Bloc 9 – Usage local ─────────────────────────────────────────────
    TagDescriptor { tag: "952", block: BlockId::LocalUse, field: "local_use_field",
        field_type: FieldType::GenericDataField, is_control: false,
        subfield_map: &[]
        },
];

impl FormatDescriptor for Marc21Format {
    fn name(&self) -> &str { "marc21" }

    fn tag_descriptor(&self, tag: &str) -> Option<&'static TagDescriptor> {
        MARC21_DESCRIPTORS.iter().find(|d| d.tag == tag)
    }

    fn field_type_to_tag(&self, field_type: FieldType) -> Vec<&'static TagDescriptor> {
        MARC21_DESCRIPTORS.iter().filter(|d| d.field_type == field_type).collect()
    }

    fn field_to_tag(&self, block: BlockId, field: &str) -> Option<&'static TagDescriptor> {
        MARC21_DESCRIPTORS.iter().find(|d| d.block == block && d.field == field)
    }

    fn all_descriptors(&self) -> &'static [TagDescriptor] {
        MARC21_DESCRIPTORS
    }

    fn personal_name_type_from_ind1(&self, ind1: char) -> PersonalNameType {
        match ind1 {
            '0' => PersonalNameType::Forename,
            '3' => PersonalNameType::FamilyName,
            _ => PersonalNameType::Surname,
        }
    }

    fn personal_name_type_to_ind1(&self, name_type: &PersonalNameType) -> char {
        match name_type {
            PersonalNameType::Forename => '0',
            PersonalNameType::Surname => '1',
            PersonalNameType::FamilyName => '3',
        }
    }

    fn organization_name_type_from_ind1(&self, ind1: char) -> OrganizationNameType {
        match ind1 {
            '0' => OrganizationNameType::InvertedName,
            '1' => OrganizationNameType::JurisdictionName,
            _ => OrganizationNameType::DirectOrder,
        }
    }

    fn organization_name_type_to_ind1(&self, name_type: &OrganizationNameType) -> char {
        match name_type {
            OrganizationNameType::InvertedName => '0',
            OrganizationNameType::JurisdictionName => '1',
            OrganizationNameType::DirectOrder => '2',
        }
    }
}
