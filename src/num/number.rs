#![allow(unused)]

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    error::FirkinError,
    num::{
        float::FloatWrapper,
        int::IntegerWrapper,
        rat::RationalWrapper,
        smallrational::SmallRational,
        traits::{FromFloats, FromInts, PythonMathFunctions, PythonNum},
    },
};

/// Single number, whether it's integer, rational, or irrational
#[derive(Clone)]
pub(crate) enum NumberWrapper {
    Int(IntegerWrapper),
    Rat(RationalWrapper),
    Float(FloatWrapper),
}

use NumberWrapper::{Float, Int, Rat};
use rug::{Float as RugFloat, Integer, Rational};

impl NumberWrapper {
    pub(crate) const ZERO: Self = NumberWrapper::Int(IntegerWrapper::ZERO);

    pub(crate) fn pow(&self, other: &NumberWrapper) -> NumberWrapper {
        todo!()
    }

    pub(crate) fn log(&self, base: &NumberWrapper) -> NumberWrapper {
        todo!()
    }

    pub(crate) fn abs(&self) -> NumberWrapper {
        match self {
            Int(i) => Int(i.abs()),
            Rat(r) => Rat(r.abs()),
            Float(f) => Float(f.abs()),
        }
    }
    // pub(crate) fn floor(&self) -> NumberWrapper {
    //     match self {
    //         Int(i) => Int(i.floor()),
    //         Rat(r) => Int(r.floor()),
    //         Float(f) => Int(f.floor()),
    //     }
    // }
    // pub(crate) fn ceil(&self) -> NumberWrapper {
    //     match self {
    //         Int(i) => Int(i.ceil()),
    //         Rat(r) => Int(r.ceil()),
    //         Float(f) => Int(f.ceil()),
    //     }
    // }
    // pub(crate) fn round(&self) -> NumberWrapper {
    //     match self {
    //         Int(i) => Int(i.round()),
    //         Rat(r) => Int(r.round()),
    //         Float(f) => Int(f.round()),
    //     }
    // }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            Int(i) => i.to_f64(),
            Rat(r) => r.to_f64(),
            Float(f) => f.to_f64(),
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
        Rat(value)
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
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        todo!()
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        todo!()
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        todo!()
    }

    fn py_divmod(
        &self,
        other: &NumberWrapper,
    ) -> Result<(IntegerWrapper, NumberWrapper), FirkinError> {
        todo!()
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        todo!()
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        todo!()
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        todo!()
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        todo!()
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        todo!()
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        todo!()
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        todo!()
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        todo!()
    }
}

impl PythonNum for NumberWrapper {}
