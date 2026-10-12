use std::{fmt::Display, str::FromStr};

use pyo3::{prelude::*, types::*};
use regex::regex;

mod float;
mod int;
mod number;
mod python_math;
mod rat;
mod sign;
mod smallrational;
mod traits;

use number::NumberWrapper;

use crate::{error::FirkinError, num::{int::IntegerWrapper, rat::RationalWrapper, traits::RichDisplay}};

const DEFAULT_RATIONAL_SIG_FIGS: usize = 5;

#[pyclass(skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct FirkinNumber {
    value: NumberWrapper
}

#[pymethods]
impl FirkinNumber {
    #[classmethod]
    fn new(_cls: &Bound<'_, PyType>, int: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(int)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.display_default())
    }

    fn debug(&self) -> PyResult<String> {
        Ok(self.display_debug())
    }
}

impl Display for FirkinNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl FirkinNumber {

    fn from_py_int(py_int: Bound<'_, PyInt>) -> PyResult<FirkinNumber> {
        if let Ok(i) = py_int.extract::<i64>() {
            return Ok(FirkinNumber { value: NumberWrapper::from(i) })
        } else if let Ok(i) = py_int.extract::<i128>() {
            return Ok(FirkinNumber {value: NumberWrapper::from(i)});
        } else if let Ok(u) = py_int.extract::<u128>() {
            return Ok(FirkinNumber { value: NumberWrapper::from(u) })
        } 

        let py_str = py_int.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        if let Ok(s) = py_str.extract::<String>() {
            return Ok( FirkinNumber { value: NumberWrapper::from_str(&s)? })
        }

        Err(FirkinError::CannotConvertToNumber(py_int.to_string(), Some("Unable to convert to integer.".into())).into())
    }

    fn from_py_float(py_float: Bound<'_, PyFloat>) -> PyResult<FirkinNumber> {
        let py_str = py_float.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        match py_str.extract::<String>() {
            Ok(s) => {
                let m = match regex!(r"\s*([-+]?\d*).?(\d*)[eE]?([-+]?\d*)\s*").captures(&s) {
                    Some(m) => m,
                    None => return Err(FirkinError::CannotConvertToNumber(s, Some("string did not meet format specifications for float".into())).into()),
                };

                let consider_rational: bool;

                if (&m[1]).trim_start_matches('0').len() == 0 {
                    // no pre-decimal digits
                    if (&m[2]).trim_matches('0').len() <= DEFAULT_RATIONAL_SIG_FIGS {
                        consider_rational = true;
                    } else {
                        consider_rational = false;
                    }
                } else if (&m[2]).trim_end_matches('0').len() == 0 {
                    // no post-decimal digits
                    if (&m[1]).trim_matches('0').len() <= DEFAULT_RATIONAL_SIG_FIGS {
                        consider_rational = true;
                    } else {
                        consider_rational = false;
                    }
                } else {
                    // both pre and post decimal digits
                    if (&m[1]).trim_start_matches('0').len() + (&m[2]).trim_end_matches('0').len() <= DEFAULT_RATIONAL_SIG_FIGS {
                        consider_rational = true;
                    } else {
                        consider_rational = false;
                    }
                }

                if consider_rational {
                    Ok( FirkinNumber {value: NumberWrapper::from(match RationalWrapper::from_decimal_str(&s) {
                        Ok(r) => r,
                        Err(e) => return Err(e.into()),
                    })})
                } else {
                    match py_float.extract::<f64>() {
                        Ok(f) => Ok(FirkinNumber { value: NumberWrapper::from(f)}),
                        Err(_) => Err(FirkinError::CannotConvertToNumber(py_float.to_string(), Some("float format spec fail".into())).into()),
                    }
                }
            },
            Err(_) => {
                match py_float.extract::<f64>() {
                    Ok(f) => Ok(FirkinNumber { value: NumberWrapper::from(f)}),
                    Err(_) => Err(FirkinError::CannotConvertToNumber(py_float.to_string(), Some("float format spec fail".into())).into()),
                }
            },
        }
    }
}

impl FromPyObject<'_, '_> for FirkinNumber {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(f) = obj.cast::<PyInt>() {
            FirkinNumber::from_py_int(f.to_owned())
        } else if let Ok(f) = obj.cast::<PyFloat>() {
            FirkinNumber::from_py_float(f.to_owned())
        } else {
            Err(FirkinError::CannotConvertToNumber(obj.to_string(), None).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use pyo3::{PyResult, Python};

    fn run_closure<F>(closure: F) -> PyResult<()>
    where
        F: for<'py> FnOnce(Python<'py>) -> PyResult<()>,
    {
        Python::initialize();

        match Python::attach(closure) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn test_from_int() -> PyResult<()> {
        run_closure(|py| {
            py.run(
                cr#"from firkin import FirkinNumber
for i in [
1234812,
-129873587561632981623847125461234,
-128349712349817265916236498762985612347198237648172359817236491562384716289576192344871234,
]:
    assert str(FirkinNumber.new(i)) == str(i)
"#,
                None,
                None,
            )
        })
    }
}

impl RichDisplay for FirkinNumber {
    fn display_default(&self) -> String {
        self.value.display_default()
    }

    fn display_debug(&self) -> String {
        self.value.display_debug()
    }

    fn format(&self, format_specifier:String) -> String {
        self.value.format(format_specifier)
    }
}