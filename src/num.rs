use std::{
    fmt::Display,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

use pyo3::prelude::*;

mod float;
mod int;
pub(crate) mod number;
mod numpy_math;
mod python_math;
mod rat;
mod sign;
pub(crate) mod smallrational;
pub(crate) mod traits;

use number::NumberWrapper;

use crate::{
    error::FirkinError,
    num::{
        int::IntegerWrapper,
        traits::{NumpyUFuncs, PythonInt, PythonMathFunctions, RichDisplay},
    },
};

const DEFAULT_RATIONAL_SIG_FIGS: usize = 5;

#[pyclass(from_py_object)]
#[derive(Clone, Debug)]
pub struct FirkinNumber {
    value: NumberWrapper,
}

#[pymethods]
impl FirkinNumber {
    #[new]
    fn init(input: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok(input.into())
    }

    fn is_finite(&self) -> PyResult<bool> {
        Ok(self.value.is_finite())
    }

    fn is_infinite(&self) -> PyResult<bool> {
        Ok(self.value.is_infinite())
    }

    fn is_nan(&self) -> PyResult<bool> {
        Ok(self.value.is_nan())
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

    fn __lt__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value < other)
    }

    fn __le__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value <= other)
    }

    fn __gt__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value > other)
    }

    fn __ge__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value >= other)
    }

    fn __eq__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value == other)
    }

    fn __ne__(&self, other: NumberWrapper) -> PyResult<bool> {
        Ok(self.value != other)
    }

    fn __add__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((&self.value + other).into())
    }

    fn __sub__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((&self.value - other).into())
    }

    fn __mul__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((&self.value * other).into())
    }

    fn __truediv__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok(self.value.py_truediv(&other)?.into())
    }

    fn __floordiv__(&self, other: NumberWrapper) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_floordiv(&other)?)
    }

    fn __mod__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok(self.value.py_mod(&other)?.into())
    }

    fn __divmod__(&self, other: NumberWrapper) -> PyResult<(IntegerWrapper, FirkinNumber)> {
        let (numa, numb) = self.value.py_divmod(&other)?;
        Ok((numa, numb.into()))
    }

    fn __pow__(
        &self,
        other: NumberWrapper,
        modulo: Option<NumberWrapper>,
    ) -> PyResult<FirkinNumber> {
        Ok(self.value.py_pow(&other, modulo)?.into())
    }

    fn __radd__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((other + &self.value).into())
    }

    fn __rsub__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((other - &self.value).into())
    }

    fn __rmul__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok((other * &self.value).into())
    }

    fn __rtruediv__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok(other.py_truediv(&self.value)?.into())
    }

    fn __rfloordiv__(&self, other: NumberWrapper) -> PyResult<IntegerWrapper> {
        Ok(other.py_floordiv(&self.value)?)
    }

    fn __rmod__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        Ok(other.py_mod(&self.value)?.into())
    }

    fn __rdivmod__(&self, other: NumberWrapper) -> PyResult<(IntegerWrapper, FirkinNumber)> {
        let (numa, numb) = other.py_divmod(&self.value)?;
        Ok((numa, numb.into()))
    }

    fn __rpow__(
        &self,
        other: NumberWrapper,
        modulo: Option<NumberWrapper>,
    ) -> PyResult<FirkinNumber> {
        Ok(FirkinNumber {
            value: other.py_pow(&self.value, modulo)?,
        })
    }

    fn __iadd__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = &self.value + other;
        Ok(())
    }

    fn __isub__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = &self.value - other;
        Ok(())
    }

    fn __imul__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = &self.value * other;
        Ok(())
    }

    fn __itruediv__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = self.value.py_truediv(&other)?;
        Ok(())
    }

    fn __ifloordiv__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = self.value.py_floordiv(&other)?.into();
        Ok(())
    }

    fn __imod__(&mut self, other: NumberWrapper) -> PyResult<()> {
        self.value = self.value.py_mod(&other)?;
        Ok(())
    }

    fn __ipow__(&mut self, other: NumberWrapper, modulo: Option<NumberWrapper>) -> PyResult<()> {
        self.value = self.value.py_pow(&other, modulo)?;
        Ok(())
    }

    fn __neg__(&self) -> PyResult<FirkinNumber> {
        Ok((-self.value.clone()).into())
    }

    fn __pos__(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.clone().into())
    }

    fn __abs__(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.py_abs()?.into())
    }

    fn __int__(&self) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_int()?)
    }

    fn __float__(&self) -> PyResult<f64> {
        Ok(self.value.py_float()?)
    }

    fn __round__(&self, ndigits: Option<NumberWrapper>) -> PyResult<FirkinNumber> {
        match &ndigits {
            Some(n) => Ok(FirkinNumber {
                value: self.value.py_round(match n.to_integer()?.to_i32() {
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
            None => Ok(self.value.py_round(0)?.into()),
        }
    }

    fn __trunc__(&self) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_trunc()?)
    }

    fn __floor__(&self) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_floor()?)
    }

    fn __ceil__(&self) -> PyResult<IntegerWrapper> {
        Ok(self.value.py_ceil()?)
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

    fn __and__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitand(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise and method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __or__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitor(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise or method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __xor__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i.bitxor(i2).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise xor method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rlshift__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => match i.to_u32() {
                Ok(u) => FirkinNumber { value: other }.__lshift__(u),
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

    fn __rrshift__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match &self.value {
            NumberWrapper::Int(i) => match i.to_u32() {
                Ok(u) => FirkinNumber { value: other }.__rshift__(u),
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

    fn __rand__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i2.bitand(i).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise and method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __ror__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
            (NumberWrapper::Int(i), NumberWrapper::Int(i2)) => Ok(FirkinNumber {
                value: i2.bitor(i).into(),
            }),
            _ => Err(FirkinError::IntegerOnlyMethod(
                "Bitwise or method only available on integers".into(),
            )
            .into()),
        }
    }

    fn __rxor__(&self, other: NumberWrapper) -> PyResult<FirkinNumber> {
        match (&self.value, &other) {
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

    fn __iand__(&mut self, other: NumberWrapper) -> PyResult<()> {
        match (&mut self.value, &other) {
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

    fn __ior__(&mut self, other: NumberWrapper) -> PyResult<()> {
        match (&mut self.value, &other) {
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

    fn __ixor__(&mut self, other: NumberWrapper) -> PyResult<()> {
        match (&mut self.value, &other) {
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

    // numpy ufuncs
    fn exp(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_exp()?.into())
    }

    fn exp2(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_exp2()?.into())
    }

    fn log(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_log()?.into())
    }

    fn log2(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_log2()?.into())
    }

    fn log10(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_log10()?.into())
    }

    fn sqrt(&self) -> PyResult<FirkinNumber> {
        Ok(self.value.np_sqrt()?.into())
    }
}

impl Display for FirkinNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl From<NumberWrapper> for FirkinNumber {
    fn from(value: NumberWrapper) -> Self {
        FirkinNumber { value }
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

    #[test]
    fn test_to_int() -> PyResult<()> {
        run_closure(|py| {
            py.run(
                cr#"from firkin import FirkinNumber
for i in [
1234812,
-129873587561632981623847125461234,
-128349712349817265916236498762985612347198237648172359817236491562384716289576192344871234,
]:
    assert int(FirkinNumber.new(i)) == i
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
