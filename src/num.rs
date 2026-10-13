use std::{
    fmt::Display,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
    str::FromStr,
};

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

use crate::{
    error::FirkinError,
    num::{
        int::IntegerWrapper,
        rat::RationalWrapper,
        traits::{PythonInt, PythonMathFunctions, RichDisplay},
    },
};

const DEFAULT_RATIONAL_SIG_FIGS: usize = 5;

#[pyclass(skip_from_py_object)]
#[derive(Clone, Debug)]
pub struct FirkinNumber {
    value: NumberWrapper,
}

#[pymethods]
impl FirkinNumber {
    #[classmethod]
    fn new(_cls: &Bound<'_, PyType>, int: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(int)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.display_debug())
    }

    fn debug(&self) -> PyResult<String> {
        Ok(self.display_debug())
    }

    fn __format__(&self, format_spec: String) -> PyResult<String> {
        Ok(self.format(format_spec))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.display_default())
    }

    fn __lt__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value < other.value)
    }

    fn __le__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value <= other.value)
    }

    fn __gt__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value > other.value)
    }

    fn __ge__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value >= other.value)
    }

    fn __eq__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value == other.value)
    }

    fn __ne__(&self, other: FirkinNumber) -> PyResult<bool> {
        Ok(self.value != other.value)
    }

    fn __add__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: &self.value + other.value,
        })
    }

    fn __sub__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: &self.value - other.value,
        })
    }

    fn __mul__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: &self.value * other.value,
        })
    }

    fn __truediv__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_truediv(&other.value)?,
        })
    }

    fn __floordiv__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_floordiv(&other.value)?.into(),
        })
    }

    fn __mod__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_mod(&other.value)?,
        })
    }

    fn __divmod__(&self, other: FirkinNumber) -> PyResult<(FirkinNumber, FirkinNumber)> {
        let (numa, numb) = self.value.py_divmod(&other.value)?;
        Ok((
            FirkinNumber { value: numa.into() },
            FirkinNumber { value: numb },
        ))
    }

    fn __pow__(&self, other: FirkinNumber, modulo: Option<FirkinNumber>) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_pow(
                &other.value,
                match modulo {
                    Some(m) => Some(m.value),
                    None => None,
                },
            )?,
        })
    }

    fn __radd__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value + &self.value,
        })
    }

    fn __rsub__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value - &self.value,
        })
    }

    fn __rmul__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value * &self.value,
        })
    }

    fn __rtruediv__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value.py_truediv(&self.value)?,
        })
    }

    fn __rfloordiv__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value.py_floordiv(&self.value)?.into(),
        })
    }

    fn __rmod__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value.py_mod(&self.value)?,
        })
    }

    fn __rdivmod__(&self, other: FirkinNumber) -> PyResult<(FirkinNumber, FirkinNumber)> {
        let (numa, numb) = other.value.py_divmod(&self.value)?;
        Ok((
            FirkinNumber { value: numa.into() },
            FirkinNumber { value: numb },
        ))
    }

    fn __rpow__(
        &self,
        other: FirkinNumber,
        modulo: Option<FirkinNumber>,
    ) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.value.py_pow(
                &self.value,
                match modulo {
                    Some(m) => Some(m.value),
                    None => None,
                },
            )?,
        })
    }

    fn __iadd__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = &self.value + other.value;
        Ok(())
    }

    fn __isub__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = &self.value - other.value;
        Ok(())
    }

    fn __imul__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = &self.value * other.value;
        Ok(())
    }

    fn __itruediv__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = self.value.py_truediv(&other.value)?;
        Ok(())
    }

    fn __ifloordiv__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = self.value.py_floordiv(&other.value)?.into();
        Ok(())
    }

    fn __imod__(&mut self, other: FirkinNumber) -> PyResult<()> {
        self.value = self.value.py_mod(&other.value)?;
        Ok(())
    }

    fn __ipow__(&mut self, other: FirkinNumber, modulo: Option<FirkinNumber>) -> PyResult<()> {
        self.value = self.value.py_pow(
            &other.value,
            match modulo {
                Some(m) => Some(m.value),
                None => None,
            },
        )?;
        Ok(())
    }

    fn __neg__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: -self.value.clone(),
        })
    }

    fn __pos__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.clone(),
        })
    }

    fn __abs__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.abs(),
        })
    }

    fn __int__(&self) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_int()?)
    }

    fn __float__(&self) -> PyResult<f64> {
        Ok(self.value.py_float()?)
    }

    fn __round__(&self, ndigits: Option<FirkinNumber>) -> PyResult<FirkinNumber> {
        match &ndigits {
            Some(n) => Ok(FirkinNumber {
                value: self.value.py_round(match n.value.to_integer()?.to_i32() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(FirkinError::OverflowError(format!(
                            "Object {:?} could not be converted to 32-bit integer",
                            ndigits
                        ))
                        .into());
                    }
                })?,
            }),
            None => Ok(FirkinNumber {
                value: self.value.py_round(0)?,
            }),
        }
    }

    fn __trunc__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_trunc()?.into(),
        })
    }

    fn __floor__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_floor()?.into(),
        })
    }

    fn __ceil__(&self) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: self.value.py_ceil()?.into(),
        })
    }

    // integer only methods
    fn __lshift__(&self, other: u32) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => Ok(FirkinNumber {
                value: i.shl(other).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rshift__(&self, other: u32) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => Ok(FirkinNumber {
                value: i.shr(other).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __and__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitand(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise and method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __or__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitor(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise or method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __xor__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitxor(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise xor method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rlshift__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => match i.to_u32() {
                Ok(u) => other.__lshift__(u),
                Err(_) => Err(FirkinError::OverflowError(
                    "Right side of bit shift must be between 0 and 2^32 - 1".into(),
                )
                .into()),
            },
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rrshift__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => match i.to_u32() {
                Ok(u) => other.__rshift__(u),
                Err(_) => Err(FirkinError::OverflowError(
                    "Right side of bit shift must be between 0 and 2^32 - 1".into(),
                )
                .into()),
            },
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rand__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i2.bitand(i).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise and method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __ror__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i2.bitor(i).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise or method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rxor__(&self, other: FirkinNumber) -> PyResult<FirkinNumber> {
        match (&self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i2.bitxor(i).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise xor method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __ilshift__(&mut self, other: u32) -> PyResult<()> {
        match &mut self.value {
            NumberWrapper::Int(i) => {
                i.shl_assign(other);
                Ok(())
            }
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __irshift__(&mut self, other: u32) -> PyResult<()> {
        match &mut self.value {
            NumberWrapper::Int(i) => {
                i.shr_assign(other);
                Ok(())
            }
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bit shifting only available on integers".into(),
            )
            .into()),
        }
    }

    fn __iand__(&mut self, other: FirkinNumber) -> PyResult<()> {
        match (&mut self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => {
                i.bitand_assign(i2);
                Ok(())
            }
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise and method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __ior__(&mut self, other: FirkinNumber) -> PyResult<()> {
        match (&mut self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => {
                i.bitor_assign(i2);
                Ok(())
            }
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise or method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __ixor__(&mut self, other: FirkinNumber) -> PyResult<()> {
        match (&mut self.value, &other.value) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => {
                i.bitxor_assign(i2);
                Ok(())
            }
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise xor method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __invert__(&self) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => Ok(FirkinNumber {
                value: i.not().into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise not method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __index__(&self) -> PyResult<IntegerWrapper> {
        match &self.value {
            NumberWrapper::Int(i) => match i.py_index() {
                Ok(r) => Ok(r),
                Err(e) => Err(e.into()),
            },
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Index method only available on integers".into(),
            )
            .into()),
        }
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
            return Ok(FirkinNumber {
                value: NumberWrapper::from(i),
            });
        } else if let Ok(i) = py_int.extract::<i128>() {
            return Ok(FirkinNumber {
                value: NumberWrapper::from(i),
            });
        } else if let Ok(u) = py_int.extract::<u128>() {
            return Ok(FirkinNumber {
                value: NumberWrapper::from(u),
            });
        }

        let py_str = py_int.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        if let Ok(s) = py_str.extract::<String>() {
            return Ok(FirkinNumber {
                value: NumberWrapper::from_str(&s)?,
            });
        }

        Err(FirkinError::CannotConvertToNumber(
            py_int.to_string(),
            Some("Unable to convert to integer.".into()),
        )
        .into())
    }

    fn from_py_float(py_float: Bound<'_, PyFloat>) -> PyResult<FirkinNumber> {
        let py_str = py_float.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        match py_str.extract::<String>() {
            Ok(s) => FirkinNumber::from_string(s),
            Err(_) => match py_float.extract::<f64>() {
                Ok(f) => Ok(FirkinNumber {
                    value: NumberWrapper::from(f),
                }),
                Err(_) => Err(FirkinError::CannotConvertToNumber(
                    py_float.to_string(),
                    Some("float format spec fail".into()),
                )
                .into()),
            },
        }
    }

    fn from_string(py_string: String) -> PyResult<FirkinNumber> {
        // this captures floats, integers, and rational numbers
        let m = match regex!(r"\s*([-+]?\d*).?(\d*)[eE]?([-+]?\d*)\s*").captures(&py_string) {
            Some(m) => m,
            None => match RationalWrapper::from_str_radix(&py_string, 10) {
                Some(rational) => {
                    return Ok(FirkinNumber {
                        value: rational.into(),
                    });
                }
                None => {
                    return Err(FirkinError::CannotConvertToNumber(
                        py_string,
                        Some("string did not meet format specifications".into()),
                    )
                    .into());
                }
            },
        };

        // if sig figs is <= default rational, parse the string as a rational number
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
            if (&m[1]).trim_start_matches('0').len() + (&m[2]).trim_end_matches('0').len()
                <= DEFAULT_RATIONAL_SIG_FIGS
            {
                consider_rational = true;
            } else {
                consider_rational = false;
            }
        }

        if consider_rational {
            Ok(FirkinNumber {
                value: NumberWrapper::from(match RationalWrapper::from_decimal_str(&py_string) {
                    Ok(r) => r,
                    Err(e) => return Err(e.into()),
                }),
            })
        } else {
            match py_string.parse::<f64>() {
                Ok(f) => Ok(FirkinNumber {
                    value: NumberWrapper::from(f),
                }),
                Err(_) => Err(FirkinError::CannotConvertToNumber(
                    py_string,
                    Some("float format spec fail".into()),
                )
                .into()),
            }
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
        } else if let Ok(s) = obj.cast::<PyString>() {
            FirkinNumber::from_string(s.extract::<String>()?)
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

    fn format(&self, format_specifier: String) -> String {
        self.value.format(format_specifier)
    }
}
