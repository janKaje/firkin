// #![allow(unused)]

use crate::{
    error::FirkinError,
    num::{
        int::IntegerWrapper,
        number::NumberWrapper,
        rat::RationalWrapper,
        smallrational::SmallRational,
        traits::{
            FromFloats, FromInts, NumpyUFuncs, PythonMathFunctions, PythonNum, RichDisplay,
            auto_impl_numpy_ufunc_trait, auto_impl_pymath_trait,
        },
    },
};
use rug::{Assign, Float, Integer, Rational};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

pub(crate) const DEFAULT_PREC: u32 = 53;

/// Wraps around the logic for both small and large precision floats
#[derive(Clone, Debug)]
pub(crate) enum FloatWrapper {
    SmallFloat(f64),
    BigFloat(Float),
}

impl FloatWrapper {
    pub(crate) fn prec(&self) -> u32 {
        match self {
            SmallFloat(_) => 53,
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

    pub(crate) fn round(&self) -> Result<IntegerWrapper, &'static str> {
        match self {
            SmallFloat(f) => f.round().try_into(),
            BigFloat(float) => float.clone().round().try_into(),
        }
    }

    pub(crate) fn is_integer(&self) -> bool {
        match self {
            SmallFloat(f) => &f.trunc() == f,
            BigFloat(float) => float.is_integer(),
        }
    }

    pub(crate) fn to_integer(&self) -> Result<IntegerWrapper, &'static str> {
        self.trunc()
    }

    pub(crate) fn trunc(&self) -> Result<IntegerWrapper, &'static str> {
        match self {
            SmallFloat(f) => f.trunc().try_into(),
            BigFloat(float) => float.clone().trunc().try_into(),
        }
    }

    pub(crate) fn is_infinite(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_infinite(),
            BigFloat(float) => float.is_infinite(),
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_finite(),
            BigFloat(float) => float.is_finite(),
        }
    }

    pub(crate) fn is_sign_positive(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_sign_positive(),
            BigFloat(float) => float.is_sign_positive(),
        }
    }

    pub(crate) fn is_sign_negative(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_sign_negative(),
            BigFloat(float) => float.is_sign_negative(),
        }
    }

    pub(crate) fn is_normal(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_normal(),
            BigFloat(float) => float.is_normal(),
        }
    }

    pub(crate) fn is_nan(&self) -> bool {
        match self {
            SmallFloat(f) => f.is_nan(),
            BigFloat(float) => float.is_nan(),
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self {
            SmallFloat(f) => f.abs() == 0.0,
            BigFloat(float) => float.is_zero(),
        }
    }

    pub(crate) fn from_incomplete_float<T>(input: T) -> FloatWrapper
    where
        Float: Assign<T>,
    {
        let mut new = Float::new(DEFAULT_PREC);
        new.assign(input);
        FloatWrapper::from(new)
    }
}

use FloatWrapper::{BigFloat, SmallFloat};

impl Display for FloatWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmallFloat(flt) => flt.fmt(f),
            BigFloat(float) => float.fmt(f),
        }
    }
}

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

impl From<&Integer> for FloatWrapper {
    fn from(value: &Integer) -> Self {
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

impl From<&IntegerWrapper> for FloatWrapper {
    fn from(value: &IntegerWrapper) -> Self {
        match value {
            IntegerWrapper::SmallInt(i) => FloatWrapper::from(*i),
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

impl From<&Rational> for FloatWrapper {
    fn from(value: &Rational) -> Self {
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

impl From<&RationalWrapper> for FloatWrapper {
    fn from(value: &RationalWrapper) -> Self {
        match value {
            RationalWrapper::SmallRat(r) => FloatWrapper::from(*r),
            RationalWrapper::Rat(rational) => FloatWrapper::from(rational),
        }
    }
}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SmallFloat(f), SmallFloat(f2)) => f.eq(f2),
            (SmallFloat(f), BigFloat(float)) => f.eq(float),
            (BigFloat(float), SmallFloat(f)) => float.eq(f),
            (BigFloat(float), BigFloat(float2)) => float.eq(float2),
        }
    }
}

impl PartialOrd for FloatWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SmallFloat(f), SmallFloat(f2)) => f.partial_cmp(f2),
            (SmallFloat(f), BigFloat(float)) => f.partial_cmp(float),
            (BigFloat(float), SmallFloat(f)) => float.partial_cmp(f),
            (BigFloat(float), BigFloat(float2)) => float.partial_cmp(float2),
        }
    }
}

impl PartialEq<IntegerWrapper> for FloatWrapper {
    fn eq(&self, other: &IntegerWrapper) -> bool {
        if !self.is_integer() {
            return false;
        }
        match self.trunc() {
            Ok(r) => r.eq(other),
            Err(_) => false,
        }
    }
}

impl PartialOrd<IntegerWrapper> for FloatWrapper {
    fn partial_cmp(&self, other: &IntegerWrapper) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        } else if self.is_nan() {
            None
        } else {
            self.partial_cmp(&FloatWrapper::from(other))
        }
    }
}

impl PartialEq<FloatWrapper> for IntegerWrapper {
    fn eq(&self, other: &FloatWrapper) -> bool {
        if !other.is_integer() {
            return false;
        }
        match other.trunc() {
            Ok(r) => self.eq(&r),
            Err(_) => false,
        }
    }
}

impl PartialOrd<FloatWrapper> for IntegerWrapper {
    fn partial_cmp(&self, other: &FloatWrapper) -> Option<std::cmp::Ordering> {
        if other.eq(self) {
            Some(std::cmp::Ordering::Equal)
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        } else if other.is_nan() {
            None
        } else {
            FloatWrapper::from(self).partial_cmp(other)
        }
    }
}

impl PartialEq<RationalWrapper> for FloatWrapper {
    fn eq(&self, other: &RationalWrapper) -> bool {
        self.eq(&FloatWrapper::from(other))
    }
}

impl PartialOrd<RationalWrapper> for FloatWrapper {
    fn partial_cmp(&self, other: &RationalWrapper) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&FloatWrapper::from(other))
    }
}

impl PartialEq<FloatWrapper> for RationalWrapper {
    fn eq(&self, other: &FloatWrapper) -> bool {
        FloatWrapper::from(self).eq(other)
    }
}

impl PartialOrd<FloatWrapper> for RationalWrapper {
    fn partial_cmp(&self, other: &FloatWrapper) -> Option<std::cmp::Ordering> {
        FloatWrapper::from(self).partial_cmp(other)
    }
}

impl FromFloats for FloatWrapper {}

impl FromInts for FloatWrapper {}

impl PythonNum for FloatWrapper {}

auto_impl_pymath_trait! {FloatWrapper {BigFloat SmallFloat}}
auto_impl_numpy_ufunc_trait! {FloatWrapper {BigFloat SmallFloat}}

impl FromStr for FloatWrapper {
    type Err = FirkinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if DEFAULT_PREC <= 53 {
            match s.parse::<f64>() {
                Ok(f) => Ok(FloatWrapper::SmallFloat(f)),
                Err(e) => Err(FirkinError::CannotConvertToFloat(
                    s.into(),
                    Some(e.to_string()),
                )),
            }
        } else {
            match Float::parse(s) {
                Ok(f) => Ok(FloatWrapper::from_incomplete_float(f)),
                Err(e) => Err(FirkinError::CannotConvertToFloat(
                    s.into(),
                    Some(e.to_string()),
                )),
            }
        }
    }
}

impl RichDisplay for FloatWrapper {
    fn display_default(&self) -> String {
        match self {
            SmallFloat(f) => format!("{:?}", f),
            BigFloat(float) => format!("{:?}", float),
        }
    }

    fn display_debug(&self) -> String {
        match self {
            SmallFloat(f) => format!("<SmallFloat {:?}>", f),
            BigFloat(float) => format!("<BigFloat {:?}>", float),
        }
    }

    fn format(&self, format_specifier: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rug::Float;

    use crate::{
        error::FirkinError,
        num::{float::FloatWrapper, int::IntegerWrapper},
    };

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

    #[test]
    fn from_str() -> Result<(), FirkinError> {
        let mut rng = rand::rng();
        for _ in 0..100 {
            // generate completely random float
            let mut as_bytes = 0.0_f64.to_ne_bytes();
            rng.fill_bytes(&mut as_bytes);
            let as_float = f64::from_ne_bytes(as_bytes);
            assert_eq!(
                FloatWrapper::from(as_float),
                as_float.to_string().parse::<FloatWrapper>()?
            );
        }

        Ok(())
    }
}
