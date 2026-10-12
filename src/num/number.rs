#![allow(unused)]

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    error::FirkinError, num::{
        float::FloatWrapper, int::IntegerWrapper, rat::RationalWrapper, smallrational::SmallRational, traits::{FromFloats, FromInts, PythonMathFunctions, PythonNum, RichDisplay},
    },
};

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
                Err(info) => Err(FirkinError::CannotConvertToInteger(float_wrapper.to_string(), Some(info.into()))),
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
        Int(value)
    }
}

impl From<RationalWrapper> for NumberWrapper {
    fn from(value: RationalWrapper) -> Self {
        if value.is_integer() {
            Int(value.to_integer())
        } else {
            Rat(value)
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
from_int! {Integer}

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
        Rat(RationalWrapper::Rat(value))
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
            (Float(f), Float(f2)) => f.eq(f),
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
            (Float(f), Float(f2)) => f.partial_cmp(f),
        }
    }
}

impl FromInts for NumberWrapper {}

impl FromFloats for NumberWrapper {}

impl PythonMathFunctions for NumberWrapper {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_pow(other, modulus),
            Rat(rational_wrapper) => rational_wrapper.py_pow(other, modulus),
            Float(float_wrapper) => float_wrapper.py_pow(other, modulus),
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_truediv(other),
            Rat(rational_wrapper) => rational_wrapper.py_truediv(other),
            Float(float_wrapper) => float_wrapper.py_truediv(other),
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_floordiv(other),
            Rat(rational_wrapper) => rational_wrapper.py_floordiv(other),
            Float(float_wrapper) => float_wrapper.py_floordiv(other),
        }
    }

    fn py_divmod(
        &self,
        other: &NumberWrapper,
    ) -> Result<(IntegerWrapper, NumberWrapper), FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_divmod(other),
            Rat(rational_wrapper) => rational_wrapper.py_divmod(other),
            Float(float_wrapper) => float_wrapper.py_divmod(other),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_abs(),
            Rat(rational_wrapper) => rational_wrapper.py_abs(),
            Float(float_wrapper) => float_wrapper.py_abs(),
        }
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_int(),
            Rat(rational_wrapper) => rational_wrapper.py_int(),
            Float(float_wrapper) => float_wrapper.py_int(),
        }
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_float(),
            Rat(rational_wrapper) => rational_wrapper.py_float(),
            Float(float_wrapper) => float_wrapper.py_float(),
        }
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_round(ndigits),
            Rat(rational_wrapper) => rational_wrapper.py_round(ndigits),
            Float(float_wrapper) => float_wrapper.py_round(ndigits),
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_floor(),
            Rat(rational_wrapper) => rational_wrapper.py_floor(),
            Float(float_wrapper) => float_wrapper.py_floor(),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_ceil(),
            Rat(rational_wrapper) => rational_wrapper.py_ceil(),
            Float(float_wrapper) => float_wrapper.py_ceil(),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_trunc(),
            Rat(rational_wrapper) => rational_wrapper.py_trunc(),
            Float(float_wrapper) => float_wrapper.py_trunc(),
        }
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match self {
            Int(integer_wrapper) => integer_wrapper.py_mod(other),
            Rat(rational_wrapper) => rational_wrapper.py_mod(other),
            Float(float_wrapper) => float_wrapper.py_mod(other),
        }
    }
}

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

    fn format(&self, format_specifier:String) -> String {
        match self {
            Int(integer_wrapper) => integer_wrapper.format(format_specifier),
            Rat(rational_wrapper) => rational_wrapper.format(format_specifier),
            Float(float_wrapper) => float_wrapper.format(format_specifier),
        }
    }
}