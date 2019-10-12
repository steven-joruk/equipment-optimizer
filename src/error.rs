use crate::item::Location;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoCombinations,
    MissingItem { loc: Location },
    ValidatingItemSet { msg: String },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCombinations => write!(f, "There are no item set combinations"),
            Self::MissingItem { loc } => write!(f, "Failed to assign {:?} to the item set", loc),
            Self::ValidatingItemSet { msg } => {
                write!(f, "Validation of the item set failed: {}", msg)
            }
        }
    }
}
