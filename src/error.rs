use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    NoCombinations,
    ValidatingItemSet { msg: String },
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeJson(e) => e.fmt(f),
            Self::NoCombinations => write!(f, "There are no item set combinations"),
            Self::ValidatingItemSet { msg } => {
                write!(f, "Validation of the item set failed: {}", msg)
            }
        }
    }
}
