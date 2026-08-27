use pyo3::exceptions::*;
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
pub(crate) enum FirkinError {
    UnitNotFound(String),
    IncompatibleUnits { first: String, second: String },
    CannotConvertToFirkin(String),
    CannotConvertToNumber(String),
}

impl std::error::Error for FirkinError {}

impl fmt::Display for FirkinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnitNotFound(query) => write!(f, "Unit {query} not found"),
            Self::IncompatibleUnits { first, second } => {
                write!(f, "{first} and {second} are not compatible")
            }
            Self::CannotConvertToFirkin(other) => {
                write!(f, "{other} cannot be coerced into a unit")
            }
            Self::CannotConvertToNumber(other) => {
                write!(f, "{other} cannot be coerced into a number")
            }
        }
    }
}

impl std::convert::From<FirkinError> for PyErr {
    fn from(err: FirkinError) -> PyErr {
        match &err {
            FirkinError::UnitNotFound(_query) => PyLookupError::new_err(err.to_string()),
            FirkinError::IncompatibleUnits {
                first: _,
                second: _,
            } => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToFirkin(_other) => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToNumber(_other) => PyTypeError::new_err(err.to_string()),
        }
    }
}
