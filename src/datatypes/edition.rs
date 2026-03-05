use serde::{Deserialize, Serialize};

use crate::datatypes::{
    NoteData, data_field_from_desc, find_code_for_name, get_remaining_subfields,
    get_subfield_by_names, known_codes_from_map, push_subfield_by_names,
};
use crate::formats::{FieldType, SubfieldMapping, TagDescriptor};
use crate::record::DataField;

/// Publication function (RDA 264 ind2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationFunction {
    Production,
    Publication,
    Distribution,
    Manufacture,
    Copyright,
}

impl PublicationFunction {
    pub fn from_ind2(ind2: char) -> Option<Self> {
        match ind2 {
            '0' => Some(Self::Production),
            '1' => Some(Self::Publication),
            '2' => Some(Self::Distribution),
            '3' => Some(Self::Manufacture),
            '4' => Some(Self::Copyright),
            _ => None,
        }
    }

    pub fn to_ind2(&self) -> char {
        match self {
            Self::Production => '0',
            Self::Publication => '1',
            Self::Distribution => '2',
            Self::Manufacture => '3',
            Self::Copyright => '4',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditionStatementData {
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remainder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl EditionStatementData {
    pub fn from_subfields_with_map(subfields: &[(char, String)], map: &[SubfieldMapping]) -> Option<Self> {
        let edition = get_subfield_by_names(subfields, map, &["edition"])?;
        let known = known_codes_from_map(map);
        Some(Self {
            edition,
            remainder: get_subfield_by_names(subfields, map, &["remainder"]),
            other_subfields: get_remaining_subfields(subfields, &known),
        })
    }

    pub fn to_subfields_with_map(&self, map: &[SubfieldMapping]) -> Vec<(char, String)> {
        let code = find_code_for_name(map, "edition").unwrap_or('a');
        let mut out = vec![(code, self.edition.clone())];
        push_subfield_by_names(&mut out, map, &["remainder"], &self.remainder);
        out.extend(self.other_subfields.clone());
        out
    }
}

/// Publication/imprint: place, publisher, date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationData {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_rda: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<PublicationFunction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub places: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub publishers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dates: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_places: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_dates: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_subfields: Vec<(char, String)>,
}

impl PublicationData {
    /// Format-agnostic parse via SubfieldMapping.
    pub fn from_subfields_with_map(ind2: char, subfields: &[(char, String)], desc: &TagDescriptor) -> Self {
        let map = desc.subfield_map;
        let known = known_codes_from_map(map);
        let place_code = find_code_for_name(map, "place");
        let publisher_code = find_code_for_name(map, "publisher");
        let date_code = find_code_for_name(map, "date");
        let mfg_place_code = find_code_for_name(map, "manufacturing_place");
        let mfg_date_code = find_code_for_name(map, "manufacturing_date");

        Self {
            is_rda: desc.field == "publication_distribution_imprint_rda",
            function: PublicationFunction::from_ind2(ind2),
            places: place_code.map(|c| subfields.iter().filter(|(sc,_)| *sc==c).map(|(_,v)| v.clone()).collect()).unwrap_or_default(),
            publishers: publisher_code.map(|c| subfields.iter().filter(|(sc,_)| *sc==c).map(|(_,v)| v.clone()).collect()).unwrap_or_default(),
            dates: date_code.map(|c| subfields.iter().filter(|(sc,_)| *sc==c).map(|(_,v)| v.clone()).collect()).unwrap_or_default(),
            manufacturing_places: mfg_place_code.map(|c| subfields.iter().filter(|(sc,_)| *sc==c).map(|(_,v)| v.clone()).collect()).unwrap_or_default(),
            manufacturing_dates: mfg_date_code.map(|c| subfields.iter().filter(|(sc,_)| *sc==c).map(|(_,v)| v.clone()).collect()).unwrap_or_default(),
            other_subfields: get_remaining_subfields(subfields, &known),
        }
    }

    /// Format-agnostic serialization via TagDescriptor.
    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> DataField {
        let map = desc.subfield_map;
        let mut out = Vec::new();
        let ind2 = self.function.as_ref().map(|f| f.to_ind2()).unwrap_or(' ');
        if let Some(code) = find_code_for_name(map, "place") {
            for p in &self.places { out.push((code, p.clone())); }
        }
        if let Some(code) = find_code_for_name(map, "publisher") {
            for p in &self.publishers { out.push((code, p.clone())); }
        }
        if let Some(code) = find_code_for_name(map, "date") {
            for d in &self.dates { out.push((code, d.clone())); }
        }
        if let Some(code) = find_code_for_name(map, "manufacturing_place") {
            for p in &self.manufacturing_places { out.push((code, p.clone())); }
        }
        if let Some(code) = find_code_for_name(map, "manufacturing_date") {
            for d in &self.manufacturing_dates { out.push((code, d.clone())); }
        }
        out.extend(self.other_subfields.clone());
        data_field_from_desc(desc, ' ', ind2, out)
    }

    pub fn place(&self) -> Option<&str> {
        self.places.first().map(String::as_str)
    }

    pub fn publisher(&self) -> Option<&str> {
        self.publishers.first().map(String::as_str)
    }

    pub fn date(&self) -> Option<&str> {
        self.dates.first().map(String::as_str)
    }
}

/// Edition fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Edition {
    EditionStatement(EditionStatementData),
    Publication(PublicationData),
    MusicalPresentationStatement(NoteData),
    CartographicMathematicalData(NoteData),
    ComputerFileCharacteristics(NoteData),
    CountryOfProducingEntity(NoteData),
    PhilatelicIssueData(NoteData),
}

impl Edition {
    /// Format-agnostic parse via TagDescriptor.
    pub fn from_desc(_ind1: char, ind2: char, subfields: &[(char, String)], desc: &TagDescriptor) -> Option<Self> {
        match desc.field_type {
            FieldType::EditionStatement => {
                EditionStatementData::from_subfields_with_map(subfields, desc.subfield_map)
                    .map(Edition::EditionStatement)
            }
            FieldType::Publication => {
                Some(Edition::Publication(PublicationData::from_subfields_with_map(ind2, subfields, desc)))
            }
            FieldType::NoteEntry | FieldType::SimpleString => {
                NoteData::from_subfields_with_map(subfields, desc.subfield_map)
                    .map(|n| match desc.field {
                        "musical_presentation_statement" | "music_presentation_statement" =>
                            Edition::MusicalPresentationStatement(n),
                        "cartographic_mathematical_data" =>
                            Edition::CartographicMathematicalData(n),
                        "computer_file_characteristics" =>
                            Edition::ComputerFileCharacteristics(n),
                        "country_of_producing_entity" =>
                            Edition::CountryOfProducingEntity(n),
                        "philatelic_issue_data" =>
                            Edition::PhilatelicIssueData(n),
                        _ => Edition::EditionStatement(EditionStatementData {
                            edition: n.text,
                            remainder: None,
                            other_subfields: n.other_subfields,
                        }),
                    })
            }
            _ => None,
        }
    }

    /// Format-agnostic serialization via TagDescriptor.
    pub fn to_raw_with_desc(&self, desc: &TagDescriptor) -> Option<DataField> {
        let df = match self {
            Edition::EditionStatement(d) => {
                data_field_from_desc(desc, ' ', ' ', d.to_subfields_with_map(desc.subfield_map))
            }
            Edition::Publication(d) => d.to_raw_with_desc(desc),
            Edition::MusicalPresentationStatement(d)
            | Edition::CartographicMathematicalData(d)
            | Edition::ComputerFileCharacteristics(d)
            | Edition::CountryOfProducingEntity(d)
            | Edition::PhilatelicIssueData(d) => {
                data_field_from_desc(desc, ' ', ' ', d.to_subfields_with_map(desc.subfield_map))
            }
        };
        Some(df)
    }
}
