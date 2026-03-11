use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// MARC relator code (e.g. $4 in name fields). Based on LOC relator code list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelatorCode {
    Author,
    Editor,
    Illustrator,
    Translator,
    Director,
    Composer,
    Performer,
    Photographer,
    Publisher,
    Narrator,
    Contributor,
    Compiler,
    Adapter,
    Annotator,
    Arranger,
    Other(String),
}

impl RelatorCode {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "aut" => Self::Author,
            "edt" => Self::Editor,
            "ill" => Self::Illustrator,
            "trl" => Self::Translator,
            "drt" => Self::Director,
            "cmp" => Self::Composer,
            "prf" => Self::Performer,
            "pht" => Self::Photographer,
            "pbl" => Self::Publisher,
            "nrt" => Self::Narrator,
            "ctb" => Self::Contributor,
            "com" => Self::Compiler,
            "adp" => Self::Adapter,
            "ann" => Self::Annotator,
            "arr" => Self::Arranger,
            _ => Self::Other(code.to_string()),
        }
    }

    pub fn as_code(&self) -> &str {
        match self {
            Self::Author => "aut",
            Self::Editor => "edt",
            Self::Illustrator => "ill",
            Self::Translator => "trl",
            Self::Director => "drt",
            Self::Composer => "cmp",
            Self::Performer => "prf",
            Self::Photographer => "pht",
            Self::Publisher => "pbl",
            Self::Narrator => "nrt",
            Self::Contributor => "ctb",
            Self::Compiler => "com",
            Self::Adapter => "adp",
            Self::Annotator => "ann",
            Self::Arranger => "arr",
            Self::Other(ref s) => s.as_str(),
        }
    }
}

impl Serialize for RelatorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_code())
    }
}

impl<'de> Deserialize<'de> for RelatorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(RelatorCode::from_code(&s))
    }
}
