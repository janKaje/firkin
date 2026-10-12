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
    CannotConvertToNumber(String, Option<String>),
    CannotConvertToInteger(String, Option<String>),
    CannotConvertToFloat(String, Option<String>),
    PowModDoesNotExist(String, String, String),
    DivideByZero(String, String),
    CannotConvertInfinity(String),
    OverflowError(String),
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
            FirkinError::CannotConvertToNumber(input, opt_addl_info) => match opt_addl_info {
                Some(addl_info) => write!(
                    f,
                    "Cannot convert \"{}\" to number. Additional info: {}",
                    input, addl_info
                ),
                None => write!(f, "Cannot convert \"{}\" to number", input),
            }
            FirkinError::PowModDoesNotExist(base, exp, modulo) => {
                write!(f, "{base}^{exp}%{modulo} does not exist")
            }
            FirkinError::DivideByZero(numer, denom) => {
                write!(f, "{numer}/{denom} does not exist - divide by zero error")
            }
            FirkinError::CannotConvertInfinity(msg) => write!(f, "{}", msg),
            FirkinError::CannotConvertToInteger(input, opt_addl_info) => match opt_addl_info {
                Some(addl_info) => write!(
                    f,
                    "Cannot convert \"{}\" to integer. Additional info: {}",
                    input, addl_info
                ),
                None => write!(f, "Cannot convert \"{}\" to integer", input),
            },
            FirkinError::CannotConvertToFloat(input, opt_addl_info) => match opt_addl_info {
                Some(addl_info) => write!(
                    f,
                    "Cannot convert \"{}\" to float. Additional info: {}",
                    input, addl_info
                ),
                None => write!(f, "Cannot convert \"{}\" to float", input),
            },
            FirkinError::OverflowError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::convert::From<FirkinError> for PyErr {
    fn from(err: FirkinError) -> PyErr {
        match &err {
            FirkinError::UnitNotFound(_) => PyLookupError::new_err(err.to_string()),
            FirkinError::LogUnitNotFound(_) => PyLookupError::new_err(err.to_string()),
            FirkinError::ConstantNotFound(_) => PyLookupError::new_err(err.to_string()),
            FirkinError::IncompatibleUnits {
                first: _,
                second: _,
            } => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToFirkin(_) => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToNumber(_, _) => PyTypeError::new_err(err.to_string()),
            FirkinError::PowModDoesNotExist(_, _, _) => PyTypeError::new_err(err.to_string()),
            FirkinError::DivideByZero(_, _) => PyZeroDivisionError::new_err(err.to_string()),
            FirkinError::CannotConvertInfinity(_) => PyOverflowError::new_err(err.to_string()),
            FirkinError::CannotConvertToInteger(_, _) => PyTypeError::new_err(err.to_string()),
            FirkinError::CannotConvertToFloat(_, _) => PyTypeError::new_err(err.to_string()),
            FirkinError::OverflowError(_) => PyOverflowError::new_err(err.to_string()),
        }
    }
}
