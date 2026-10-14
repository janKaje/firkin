// #![allow(unused)]

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use pyo3::{prelude::*, types::*};
use regex::regex;

use crate::{
    error::FirkinError,
    num::{
        DEFAULT_RATIONAL_SIG_FIGS, FirkinNumber,
        float::FloatWrapper,
        int::IntegerWrapper,
        rat::RationalWrapper,
        smallrational::SmallRational,
        traits::{
            FromFloats, FromInts, NumpyUFuncs, PythonMathFunctions, PythonNum, RichDisplay,
            auto_impl_numpy_ufunc_trait, auto_impl_pymath_trait,
        },
    },
};

pub(crate) const MAX_MEMORY_EXACT: u32 = 8 * 256; // 256 bytes is just too much for a single number. convert to float.

/// Single number, whether it's integer, rational, or irrational
#[derive(Clone, Debug)]
pub(crate) enum NumberWrapper {
    Int(IntegerWrapper),
    Rat(RationalWrapper),
    Float(FloatWrapper),
}

use NumberWrapper::{Float, Int, Rat};
use rug::{Float as RugFloat, Integer, Rational};

impl NumberWrapper {
    pub(crate) const ZERO: Self = NumberWrapper::Int(IntegerWrapper::ZERO);
    pub(crate) const ONE: Self = NumberWrapper::Int(IntegerWrapper::ONE);

    pub(crate) fn abs(&self) -> NumberWrapper {
        match self {
            Int(i) => Int(i.abs()),
            Rat(r) => Rat(r.abs()),
            Float(f) => Float(f.abs()),
        }
    }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            Int(i) => i.to_f64(),
            Rat(r) => r.to_f64(),
            Float(f) => f.to_f64(),
        }
    }

    pub(crate) fn to_float(&self) -> RugFloat {
        match self {
            Int(integer_wrapper) => FloatWrapper::from(integer_wrapper).to_float(),
            Rat(rational_wrapper) => FloatWrapper::from(rational_wrapper).to_float(),
            Float(float_wrapper) => float_wrapper.to_float(),
        }
    }

    pub(crate) fn to_integer(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => Ok(integer_wrapper.clone()),
            Rat(rational_wrapper) => Ok(rational_wrapper.to_integer()),
            Float(float_wrapper) => match float_wrapper.to_integer() {
                Ok(i) => Ok(i),
                Err(info) => Err(FirkinError::CannotConvertToInteger(
                    float_wrapper.to_string(),
                    Some(info.into()),
                )),
            },
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self {
            Int(integer_wrapper) => integer_wrapper.is_zero(),
            Rat(rational_wrapper) => rational_wrapper.is_zero(),
            Float(float_wrapper) => float_wrapper.is_zero(),
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        match self {
            Float(f) => f.is_finite(),
            _ => true,
        }
    }

    pub(crate) fn is_infinite(&self) -> bool {
        match self {
            Float(f) => f.is_infinite(),
            _ => false,
        }
    }

    pub(crate) fn is_nan(&self) -> bool {
        match self {
            Float(f) => f.is_nan(),
            _ => false,
        }
    }

    fn from_py_int(py_int: Bound<'_, PyInt>) -> PyResult<NumberWrapper> {
        if let Ok(i) = py_int.extract::<i64>() {
            return Ok(NumberWrapper::from(i));
        } else if let Ok(i) = py_int.extract::<i128>() {
            return Ok(NumberWrapper::from(i));
        } else if let Ok(u) = py_int.extract::<u128>() {
            return Ok(NumberWrapper::from(u));
        }

        let py_str = py_int.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        if let Ok(s) = py_str.extract::<String>() {
            return Ok(NumberWrapper::from_str(&s)?);
        }

        Err(FirkinError::CannotConvertToNumber(
            py_int.to_string(),
            Some("Unable to convert to integer.".into()),
        )
        .into())
    }

    fn from_py_float(py_float: Bound<'_, PyFloat>) -> PyResult<NumberWrapper> {
        let py_str = py_float.getattr("__str__")?.call0()?;
        let py_str = py_str.cast::<PyString>()?;
        match py_str.extract::<String>() {
            Ok(s) => NumberWrapper::from_string(s),
            Err(_) => match py_float.extract::<f64>() {
                Ok(f) => Ok(NumberWrapper::from(f)),
                Err(_) => Err(FirkinError::CannotConvertToNumber(
                    py_float.to_string(),
                    Some("float format spec fail".into()),
                )
                .into()),
            },
        }
    }

    fn from_string(py_string: String) -> PyResult<NumberWrapper> {
        // this captures floats, integers, and rational numbers
        let m = match regex!(r"\s*([-+]?\d*).?(\d*)[eE]?([-+]?\d*)\s*").captures(&py_string) {
            Some(m) => m,
            None => match RationalWrapper::from_str_radix(&py_string, 10) {
                Some(rational) => {
                    return Ok(rational.into());
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
            Ok(NumberWrapper::from(
                match RationalWrapper::from_decimal_str(&py_string) {
                    Ok(r) => r,
                    Err(e) => return Err(e.into()),
                },
            ))
        } else {
            match py_string.parse::<f64>() {
                Ok(f) => Ok(NumberWrapper::from(f)),
                Err(_) => Err(FirkinError::CannotConvertToNumber(
                    py_string,
                    Some("float format spec fail".into()),
                )
                .into()),
            }
        }
    }
}

impl Display for NumberWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(integer_wrapper) => integer_wrapper.fmt(f),
            Rat(rational_wrapper) => rational_wrapper.fmt(f),
            Float(float_wrapper) => float_wrapper.fmt(f),
        }
    }
}

macro_rules! arith_binary {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<NumberWrapper> for NumberWrapper {
            type Output = NumberWrapper;

            fn $method(self, rhs: NumberWrapper) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $ImpAssign<NumberWrapper> for NumberWrapper {
            fn $method_assign(&mut self, rhs: NumberWrapper) {
                match (&self, rhs) {
                    (Int(i), Int(i2)) => *self = Int(i.$method(i2)),
                    (Int(i), Rat(r)) => *self = Rat(i.$method(r)),
                    (Int(i), Float(f)) => *self = Float(i.$method(f)),
                    (Rat(r), Int(i)) => *self = Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => *self = Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => *self = Float(r.$method(f)),
                    (Float(f), Int(i)) => *self = Float(f.$method(i)),
                    (Float(f), Rat(r)) => *self = Float(f.$method(r)),
                    (Float(f), Float(f2)) => *self = Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<&NumberWrapper> for &NumberWrapper {
            type Output = NumberWrapper;

            fn $method(self, rhs: &NumberWrapper) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<&NumberWrapper> for NumberWrapper {
            type Output = NumberWrapper;

            fn $method(self, rhs: &NumberWrapper) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $ImpAssign<&NumberWrapper> for NumberWrapper {
            fn $method_assign(&mut self, rhs: &NumberWrapper) {
                match (&self, rhs) {
                    (Int(i), Int(i2)) => *self = Int(i.$method(i2)),
                    (Int(i), Rat(r)) => *self = Rat(i.$method(r)),
                    (Int(i), Float(f)) => *self = Float(i.$method(f)),
                    (Rat(r), Int(i)) => *self = Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => *self = Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => *self = Float(r.$method(f)),
                    (Float(f), Int(i)) => *self = Float(f.$method(i)),
                    (Float(f), Rat(r)) => *self = Float(f.$method(r)),
                    (Float(f), Float(f2)) => *self = Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<NumberWrapper> for &NumberWrapper {
            type Output = NumberWrapper;

            fn $method(self, rhs: NumberWrapper) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }
    };
}

arith_binary! {
    Add { add }
    AddAssign { add_assign }
}

arith_binary! {
    Sub { sub }
    SubAssign { sub_assign }
}

arith_binary! {
    Mul { mul }
    MulAssign { mul_assign }
}

arith_binary! {
    Div { div }
    DivAssign { div_assign }
}

impl Neg for NumberWrapper {
    type Output = NumberWrapper;

    fn neg(self) -> Self::Output {
        match self {
            Int(i) => Int(-i),
            Rat(r) => Rat(-r),
            Float(f) => Float(-f),
        }
    }
}

impl From<IntegerWrapper> for NumberWrapper {
    fn from(value: IntegerWrapper) -> Self {
        match value {
            IntegerWrapper::SmallInt(_) => Int(value),
            IntegerWrapper::Int(integer) => integer.into(),
        }
    }
}

impl From<RationalWrapper> for NumberWrapper {
    fn from(value: RationalWrapper) -> Self {
        if value.is_integer() {
            Int(value.to_integer())
        } else {
            match value {
                RationalWrapper::SmallRat(_) => Rat(value),
                RationalWrapper::Rat(rational) => rational.into(),
            }
        }
    }
}

impl From<FloatWrapper> for NumberWrapper {
    fn from(value: FloatWrapper) -> Self {
        Float(value)
    }
}

macro_rules! from_int {
    (
        $Int:ty
    ) => {
        impl From<$Int> for NumberWrapper {
            fn from(value: $Int) -> Self {
                Int(IntegerWrapper::from(value))
            }
        }
    };
}

from_int! {i8}
from_int! {i16}
from_int! {i32}
from_int! {i64}
from_int! {i128}
from_int! {isize}
from_int! {u8}
from_int! {u16}
from_int! {u32}
from_int! {u64}
from_int! {u128}
from_int! {usize}

impl From<Integer> for NumberWrapper {
    fn from(value: Integer) -> Self {
        if value.signed_bits() > MAX_MEMORY_EXACT {
            Float(FloatWrapper::from(value))
        } else {
            Int(value.into())
        }
    }
}

macro_rules! from_float {
    (
        $Float:ty
    ) => {
        impl From<$Float> for NumberWrapper {
            fn from(value: $Float) -> Self {
                Float(FloatWrapper::from(value))
            }
        }
    };
}

from_float! {f32}
from_float! {f64}
from_float! {RugFloat}

impl From<SmallRational> for NumberWrapper {
    fn from(value: SmallRational) -> Self {
        Rat(RationalWrapper::SmallRat(value))
    }
}

impl From<Rational> for NumberWrapper {
    fn from(value: Rational) -> Self {
        if value.numer().signed_bits() + value.denom().signed_bits() > MAX_MEMORY_EXACT {
            Float(FloatWrapper::from(value))
        } else {
            Rat(value.into())
        }
    }
}

impl PartialEq for NumberWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Int(i), Int(i2)) => i.eq(i2),
            (Int(i), Rat(r)) => i.eq(r),
            (Int(i), Float(f)) => i.eq(f),
            (Rat(r), Int(i)) => r.eq(i),
            (Rat(r), Rat(r2)) => r.eq(r2),
            (Rat(r), Float(f)) => r.eq(f),
            (Float(f), Int(i)) => f.eq(i),
            (Float(f), Rat(r)) => f.eq(r),
            (Float(f), Float(f2)) => f.eq(f2),
        }
    }
}

impl PartialOrd for NumberWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Int(i), Int(i2)) => i.partial_cmp(i2),
            (Int(i), Rat(r)) => i.partial_cmp(r),
            (Int(i), Float(f)) => i.partial_cmp(f),
            (Rat(r), Int(i)) => r.partial_cmp(i),
            (Rat(r), Rat(r2)) => r.partial_cmp(r2),
            (Rat(r), Float(f)) => r.partial_cmp(f),
            (Float(f), Int(i)) => f.partial_cmp(i),
            (Float(f), Rat(r)) => f.partial_cmp(r),
            (Float(f), Float(f2)) => f.partial_cmp(f2),
        }
    }
}

impl FromInts for NumberWrapper {}

impl FromFloats for NumberWrapper {}

auto_impl_pymath_trait! {NumberWrapper {Int Rat Float}}
auto_impl_numpy_ufunc_trait! {NumberWrapper {Int Rat Float}}

impl PythonNum for NumberWrapper {}

impl FromStr for NumberWrapper {
    type Err = FirkinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<IntegerWrapper>() {
            Ok(i.into())
        } else if let Some(r) = RationalWrapper::from_str_radix(s, 10) {
            Ok(r.into())
        } else if let Ok(r) = RationalWrapper::from_decimal_str(s) {
            Ok(r.into())
        } else if let Ok(f) = s.parse::<FloatWrapper>() {
            Ok(f.into())
        } else {
            Err(FirkinError::CannotConvertToNumber(s.into(), None))
        }
    }
}

impl RichDisplay for NumberWrapper {
    fn display_default(&self) -> String {
        match self {
            Int(integer_wrapper) => integer_wrapper.display_default(),
            Rat(rational_wrapper) => rational_wrapper.display_default(),
            Float(float_wrapper) => float_wrapper.display_default(),
        }
    }

    fn display_debug(&self) -> String {
        match self {
            Int(integer_wrapper) => integer_wrapper.display_debug(),
            Rat(rational_wrapper) => rational_wrapper.display_debug(),
            Float(float_wrapper) => float_wrapper.display_debug(),
        }
    }

    fn format(&self, format_specifier: String) -> String {
        match self {
            Int(integer_wrapper) => integer_wrapper.format(format_specifier),
            Rat(rational_wrapper) => rational_wrapper.format(format_specifier),
            Float(float_wrapper) => float_wrapper.format(format_specifier),
        }
    }
}

impl FromPyObject<'_, '_> for NumberWrapper {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(f) = obj.cast::<FirkinNumber>() {
            Ok(f.extract::<FirkinNumber>()?.value)
        } else if let Ok(f) = obj.cast::<PyInt>() {
            NumberWrapper::from_py_int(f.to_owned())
        } else if let Ok(f) = obj.cast::<PyFloat>() {
            NumberWrapper::from_py_float(f.to_owned())
        } else if let Ok(s) = obj.cast::<PyString>() {
            NumberWrapper::from_string(s.extract::<String>()?)
        } else {
            Err(FirkinError::CannotConvertToNumber(obj.to_string(), None).into())
        }
    }
}
