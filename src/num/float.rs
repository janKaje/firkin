#![allow(unused)]

use crate::num::{
    int::IntegerWrapper,
    rat::RationalWrapper,
    smallrational::SmallRational,
    traits::{FromFloats, FromInts, PythonMathFunctions},
};
use rug::{Float, Integer, Rational};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

const DEFAULT_PREC: u32 = 53;

/// Wraps around the logic for both small and large precision floats
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum FloatWrapper {
    SmallFloat(f64),
    BigFloat(Float),
}

impl FloatWrapper {
    pub(crate) fn prec(&self) -> u32 {
        match self {
            SmallFloat(f) => 53,
            BigFloat(float) => float.prec(),
        }
    }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            SmallFloat(f) => *f,
            BigFloat(float) => float.to_f64(),
        }
    }

    pub(crate) fn to_float(&self) -> Float {
        match self {
            SmallFloat(f) => Float::with_val(53, f),
            BigFloat(float) => float.clone(),
        }
    }

    pub(crate) fn abs(&self) -> FloatWrapper {
        match self {
            SmallFloat(f) => f.abs().into(),
            BigFloat(float) => float.clone().abs().into(),
        }
    }

    pub(crate) fn floor(&self) -> Result<IntegerWrapper, &'static str> {
        match self {
            SmallFloat(f) => f.floor().try_into(),
            BigFloat(float) => float.clone().floor().try_into(),
        }
    }

    pub(crate) fn ceil(&self) -> Result<IntegerWrapper, &'static str> {
        match self {
            SmallFloat(f) => f.ceil().try_into(),
            BigFloat(float) => float.clone().ceil().try_into(),
        }
    }

    pub(crate) fn round(&self) -> Result<IntegerWrapper, &'static str> {
        match self {
            SmallFloat(f) => f.round().try_into(),
            BigFloat(float) => float.clone().round().try_into(),
        }
    }
}

use FloatWrapper::{BigFloat, SmallFloat};

macro_rules! arith_binary {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<FloatWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                let new_prec = self.prec().max(rhs.prec());
                match (self, rhs) {
                    (SmallFloat(f), SmallFloat(f2)) => {
                        let new = f.$method(f2);
                        if new.is_infinite() {
                            BigFloat(Float::with_val(53, f).$method(f2))
                        } else {
                            SmallFloat(new)
                        }
                    }
                    (SmallFloat(f), BigFloat(float)) => FloatWrapper::from(f.$method(float)),
                    (BigFloat(float), SmallFloat(f)) => FloatWrapper::from(float.$method(f)),
                    (BigFloat(float), BigFloat(float2)) => {
                        FloatWrapper::from(float.$method(float2))
                    }
                }
            }
        }

        impl $Imp<&FloatWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                let new_prec = self.prec().max(rhs.prec());
                match (self, rhs) {
                    (SmallFloat(f), SmallFloat(f2)) => {
                        let new = f.$method(f2);
                        if new.is_infinite() {
                            BigFloat(Float::with_val(53, f).$method(f2))
                        } else {
                            SmallFloat(new)
                        }
                    }
                    (SmallFloat(f), BigFloat(float)) => {
                        FloatWrapper::from(Float::with_val(new_prec, f.$method(float)))
                    }
                    (BigFloat(float), SmallFloat(f)) => FloatWrapper::from(float.$method(f)),
                    (BigFloat(float), BigFloat(float2)) => {
                        FloatWrapper::from(float.$method(float2))
                    }
                }
            }
        }

        impl $Imp<FloatWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                self.$method(&rhs)
            }
        }

        impl $Imp<&FloatWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                let new_prec = self.prec().max(rhs.prec());
                match (self, rhs) {
                    (SmallFloat(f), SmallFloat(f2)) => {
                        let new = f.$method(f2);
                        if new.is_infinite() {
                            BigFloat(Float::with_val(53, f).$method(f2))
                        } else {
                            SmallFloat(new)
                        }
                    }
                    (SmallFloat(f), BigFloat(float)) => {
                        FloatWrapper::from(Float::with_val(new_prec, f.$method(float)))
                    }
                    (BigFloat(float), SmallFloat(f)) => {
                        FloatWrapper::from(Float::with_val(new_prec, float.$method(f)))
                    }
                    (BigFloat(float), BigFloat(float2)) => {
                        FloatWrapper::from(Float::with_val(new_prec, float.$method(float2)))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs.clone()))
            }
        }

        impl $Imp<IntegerWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: IntegerWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs))
            }
        }

        impl $Imp<&IntegerWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs.clone()))
            }
        }

        impl $Imp<IntegerWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: IntegerWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs))
            }
        }

        impl $Imp<FloatWrapper> for IntegerWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self).$method(rhs)
            }
        }

        impl $Imp<&FloatWrapper> for IntegerWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self).$method(rhs)
            }
        }

        impl $Imp<FloatWrapper> for &IntegerWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self.clone()).$method(rhs)
            }
        }

        impl $Imp<&FloatWrapper> for &IntegerWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self.clone()).$method(rhs)
            }
        }

        impl $Imp<&RationalWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &RationalWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs.clone()))
            }
        }

        impl $Imp<RationalWrapper> for FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: RationalWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs))
            }
        }

        impl $Imp<&RationalWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &RationalWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs.clone()))
            }
        }

        impl $Imp<RationalWrapper> for &FloatWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: RationalWrapper) -> FloatWrapper {
                self.$method(FloatWrapper::from(rhs))
            }
        }

        impl $Imp<FloatWrapper> for RationalWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self).$method(rhs)
            }
        }

        impl $Imp<&FloatWrapper> for RationalWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self).$method(rhs)
            }
        }

        impl $Imp<FloatWrapper> for &RationalWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self.clone()).$method(rhs)
            }
        }

        impl $Imp<&FloatWrapper> for &RationalWrapper {
            type Output = FloatWrapper;

            fn $method(self, rhs: &FloatWrapper) -> FloatWrapper {
                FloatWrapper::from(self.clone()).$method(rhs)
            }
        }

        impl<Rhs> $ImpAssign<Rhs> for FloatWrapper
        where
            FloatWrapper: From<Rhs>,
            Rhs: From<Float>,
        {
            fn $method_assign(&mut self, rhs: Rhs) {
                let rhs: FloatWrapper = rhs.into();
                let new_prec = self.prec().max(rhs.prec());
                match (&self, &rhs) {
                    (SmallFloat(f), SmallFloat(f2)) => {
                        let new = f.$method(f2);
                        if new.is_infinite() {
                            *self = BigFloat(Float::with_val(53, *f).$method(f2))
                        } else {
                            *self = SmallFloat(new)
                        }
                    }
                    (SmallFloat(f), BigFloat(float)) => {
                        *self =
                            FloatWrapper::from(Float::with_val(new_prec, f.$method(float)).into())
                    }
                    (BigFloat(float), SmallFloat(f)) => {
                        *self = FloatWrapper::from(float.clone().$method(f).into())
                    }
                    (BigFloat(float), BigFloat(float2)) => {
                        *self = FloatWrapper::from(
                            Float::with_val(new_prec, float.$method(float2)).into(),
                        )
                    }
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

impl Neg for FloatWrapper {
    type Output = FloatWrapper;

    fn neg(self) -> Self::Output {
        match self {
            SmallFloat(f) => SmallFloat(-f),
            BigFloat(float) => BigFloat(-float),
        }
    }
}

impl Neg for &FloatWrapper {
    type Output = FloatWrapper;

    fn neg(self) -> Self::Output {
        match self {
            SmallFloat(f) => SmallFloat(-f),
            BigFloat(float) => FloatWrapper::from(Float::with_val(float.prec(), -float)),
        }
    }
}

impl From<f64> for FloatWrapper {
    fn from(value: f64) -> Self {
        SmallFloat(value)
    }
}

impl From<f32> for FloatWrapper {
    fn from(value: f32) -> Self {
        SmallFloat(value.into())
    }
}

impl From<Float> for FloatWrapper {
    fn from(value: Float) -> Self {
        if value.to_f64().is_normal() || value == 0 {
            SmallFloat(value.to_f64())
        } else {
            BigFloat(value)
        }
    }
}

macro_rules! from_primitive {
    (
        $Prim:ty
    ) => {
        impl From<$Prim> for FloatWrapper {
            fn from(value: $Prim) -> Self {
                if DEFAULT_PREC <= 53 {
                    SmallFloat(value as f64)
                } else {
                    BigFloat(Float::with_val(DEFAULT_PREC, value))
                }
            }
        }
    };
}

from_primitive! {i8}
from_primitive! {i16}
from_primitive! {i32}
from_primitive! {i64}
from_primitive! {i128}
from_primitive! {isize}
from_primitive! {u8}
from_primitive! {u16}
from_primitive! {u32}
from_primitive! {u64}
from_primitive! {u128}
from_primitive! {usize}

impl From<Integer> for FloatWrapper {
    fn from(value: Integer) -> Self {
        FloatWrapper::from(Float::with_val(DEFAULT_PREC, value))
    }
}

impl From<IntegerWrapper> for FloatWrapper {
    fn from(value: IntegerWrapper) -> Self {
        match value {
            IntegerWrapper::SmallInt(i) => FloatWrapper::from(i),
            IntegerWrapper::Int(integer) => FloatWrapper::from(integer),
        }
    }
}

impl From<SmallRational> for FloatWrapper {
    fn from(value: SmallRational) -> Self {
        SmallFloat(value.into())
    }
}

impl From<Rational> for FloatWrapper {
    fn from(value: Rational) -> Self {
        FloatWrapper::from(Float::with_val(DEFAULT_PREC, value))
    }
}

impl From<RationalWrapper> for FloatWrapper {
    fn from(value: RationalWrapper) -> Self {
        match value {
            RationalWrapper::SmallRat(r) => FloatWrapper::from(r),
            RationalWrapper::Rat(rational) => FloatWrapper::from(rational),
        }
    }
}

impl FromFloats for FloatWrapper {}

impl FromInts for FloatWrapper {}

#[cfg(test)]
mod tests {
    use rug::Float;

    use crate::num::{float::FloatWrapper, int::IntegerWrapper};

    #[test]
    fn froms() {
        assert_eq!(FloatWrapper::from(12783_f64), FloatWrapper::from(12783_f32));
        assert_eq!(
            FloatWrapper::from(Float::with_val(54, 12783)),
            FloatWrapper::from(12783_f64)
        );
    }

    #[test]
    fn arith_unary() {
        assert_eq!(
            FloatWrapper::from(-125.64).abs(),
            FloatWrapper::from(125.64)
        );
        assert_eq!(
            FloatWrapper::from(-125.64).floor().unwrap(),
            IntegerWrapper::from(-126)
        );
        assert_eq!(
            FloatWrapper::from(-125.64).ceil().unwrap(),
            IntegerWrapper::from(-125)
        );
        assert_eq!(
            FloatWrapper::from(-125.64).round().unwrap(),
            IntegerWrapper::from(-126)
        );
    }

    #[test]
    fn arith_binary() {
        assert_eq!(
            FloatWrapper::from(146.3) + FloatWrapper::from(934),
            FloatWrapper::from(1080.3)
        );
        assert_eq!(
            FloatWrapper::from(146.3) - FloatWrapper::from(934),
            FloatWrapper::from(-787.7)
        );
        assert_eq!(
            FloatWrapper::from(146.3) / FloatWrapper::from(934),
            FloatWrapper::from(0.15663811563169167)
        );
        assert_eq!(
            FloatWrapper::from(146.3) * FloatWrapper::from(934),
            FloatWrapper::from(136644.2)
        );
    }
}
