use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// MARC / ISO 3166 country code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountryCode {
    France,
    UnitedStates,
    UnitedKingdom,
    Germany,
    Spain,
    Italy,
    Canada,
    Belgium,
    Switzerland,
    Other(String),
}

impl CountryCode {
    pub fn from_code(code: &str) -> Self {
        let lower = code.to_lowercase();
        match lower.as_str() {
            "fr" | "fra" => Self::France,
            "xxu" | "usa" | "us" => Self::UnitedStates,
            "xxk" | "gb" | "gbr" => Self::UnitedKingdom,
            "gw" | "deu" | "de" => Self::Germany,
            "sp" | "esp" | "es" => Self::Spain,
            "it" | "ita" => Self::Italy,
            "xxc" | "can" | "ca" => Self::Canada,
            "be" | "bel" => Self::Belgium,
            "sz" | "che" | "ch" => Self::Switzerland,
            _ => Self::Other(code.to_string()),
        }
    }

    pub fn as_code(&self) -> &str {
        match self {
            Self::France => "fr",
            Self::UnitedStates => "xxu",
            Self::UnitedKingdom => "xxk",
            Self::Germany => "gw",
            Self::Spain => "sp",
            Self::Italy => "it",
            Self::Canada => "xxc",
            Self::Belgium => "be",
            Self::Switzerland => "sz",
            Self::Other(ref s) => s.as_str(),
        }
    }
}

impl Serialize for CountryCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_code())
    }
}

impl<'de> Deserialize<'de> for CountryCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(CountryCode::from_code(&s))
    }
}
