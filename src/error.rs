use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarcError {
    #[error("invalid record: {0}")]
    InvalidRecord(&'static str),

    #[error("invalid field value: {0}")]
    InvalidFieldValue(&'static str),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("encoding error")]
    Encoding,

    #[error("XML error: {0}")]
    Xml(String),
}

impl From<&'static str> for MarcError {
    fn from(msg: &'static str) -> Self {
        MarcError::InvalidFieldValue(msg)
    }
}
