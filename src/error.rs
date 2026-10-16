use pyo3::exceptions::*;
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
pub(crate) enum FirkinError {
    UnitNotFound(String),
    LogUnitNotFound(String),
    ConstantNotFound(String),
    IncompatibleUnits { first: String, second: String },
    CannotConvertToFirkin(String),
    CannotConvertToNumber(String),
    UnitAlreadyExists(String),
}

impl std::error::Error for FirkinError {}

impl fmt::Display for FirkinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirkinError::UnitNotFound(query) => write!(f, "Unit {query} not found"),
            FirkinError::LogUnitNotFound(query) => {
                write!(f, "Logarithmic ratio unit {query} not found")
            }
            FirkinError::ConstantNotFound(query) => write!(f, "Constant {query} not found"),
            FirkinError::IncompatibleUnits { first, second } => {
                write!(f, "{first} and {second} are not compatible")
            }
            FirkinError::CannotConvertToFirkin(other) => {
                write!(f, "{other} cannot be coerced into a unit")
            }
            FirkinError::CannotConvertToNumber(other) => {
                write!(f, "{other} cannot be coerced into a number")
            }
            FirkinError::UnitAlreadyExists(unit_query) => {
                write!(f, "Unit with the name/alias {} already exists", unit_query)
            }
        }
    }
}

impl std::convert::From<FirkinError> for PyErr {
    fn from(err: FirkinError) -> PyErr {
        match &err {
            FirkinError::UnitNotFound(_query) => PyLookupError::new_err(err.to_string()),
            FirkinError::LogUnitNotFound(_query) => PyLookupError::new_err(err.to_string()),
            FirkinError::ConstantNotFound(_query) => PyLookupError::new_err(err.to_string()),
            FirkinError::IncompatibleUnits {
                first: _,
                second: _,
            } => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToFirkin(_other) => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToNumber(_other) => PyTypeError::new_err(err.to_string()),
            FirkinError::UnitAlreadyExists(_) => PyValueError::new_err(err.to_string()),
        }
    }
}
