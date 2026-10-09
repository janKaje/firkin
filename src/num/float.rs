#![allow(unused)]

use crate::{
    error::FirkinError,
    num::{
        int::IntegerWrapper,
        number::NumberWrapper,
        rat::RationalWrapper,
        smallrational::SmallRational,
        traits::{FromFloats, FromInts, PythonMathFunctions},
    },
};
use rug::{
    Assign, Float, Integer, Rational,
    float::Round,
    ops::{AssignRound, Pow},
};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

const DEFAULT_PREC: u32 = 53;

/// Wraps around the logic for both small and large precision floats
#[derive(Clone, Debug)]
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

impl PythonMathFunctions for FloatWrapper {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_pow(other, modulus),
            BigFloat(float) => float.py_pow(other, modulus),
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_truediv(other),
            BigFloat(float) => float.py_truediv(other),
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_floordiv(other),
            BigFloat(float) => float.py_floordiv(other),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_abs(),
            BigFloat(float) => float.py_abs(),
        }
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_int(),
            BigFloat(float) => float.py_int(),
        }
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        match self {
            SmallFloat(f) => f.py_float(),
            BigFloat(float) => float.py_float(),
        }
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_round(ndigits),
            BigFloat(float) => float.py_round(ndigits),
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_floor(),
            BigFloat(float) => float.py_floor(),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_ceil(),
            BigFloat(float) => float.py_ceil(),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_trunc(),
            BigFloat(float) => float.py_trunc(),
        }
    }

    fn py_divmod(
        &self,
        other: &NumberWrapper,
    ) -> Result<(IntegerWrapper, NumberWrapper), FirkinError> {
        match self {
            SmallFloat(f) => f.py_divmod(other),
            BigFloat(float) => float.py_divmod(other),
        }
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match self {
            SmallFloat(f) => f.py_mod(other),
            BigFloat(float) => float.py_mod(other),
        }
    }
}

impl PythonMathFunctions for f64 {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok(NumberWrapper::from(
                    FloatWrapper::from_incomplete_float(self.rem(*i as f64)),
                )),
                IntegerWrapper::Int(integer) => Ok(NumberWrapper::from(self.rem(integer.to_f64()))),
            },
            NumberWrapper::Rat(_) => Ok(self.rem(other.to_f64()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(NumberWrapper::from(self.rem(f))),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self.rem(float)).into()),
            },
        }
    }

    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match modulus {
            Some(m) => self.py_pow(other, None)?.py_mod(m),
            None => match other {
                NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => Ok(self.powf(*i as f64).into()),
                    IntegerWrapper::Int(integer) => Ok(self.powf(integer.to_f64()).into()),
                },
                NumberWrapper::Rat(_) => Ok(self.pow(other.to_f64()).into()),
                NumberWrapper::Float(float_wrapper) => match float_wrapper {
                    SmallFloat(f) => Ok(self.powf(*f).into()),
                    BigFloat(float) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(float)).into())
                    }
                },
            },
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.abs() == 0.0 {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok((self / *i as f64).into()),
                IntegerWrapper::Int(integer) => Ok((self / integer.to_f64()).into()),
            },
            NumberWrapper::Rat(rational_wrapper) => Ok(self.div(other.to_f64()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok((self / *f).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self / float).into()),
            },
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.abs() == 0.0 {
            return Ok(IntegerWrapper::ZERO);
        }
        self.py_truediv(other)?.py_floor()
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(self.abs().into())
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.trunc()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot convert infinity to integer".into(),
            )),
        }
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(*self)
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if self.is_infinite() {
            return Err(FirkinError::CannotConvertInfinity(
                "Cannot round infinity".into(),
            ));
        }
        if ndigits == 0 {
            Ok(self.round().into())
        } else if ndigits < 0 {
            let mul = 10.0_f64.powi(ndigits);
            Ok(IntegerWrapper::try_from(self.mul(mul).round().div(mul))
                .unwrap()
                .into())
        } else {
            let mul = 10.0_f64.powi(ndigits);
            Ok(self.mul(mul).round().div(mul).into())
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.floor()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot floor infinity".into(),
            )),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.ceil()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot ceil infinity".into(),
            )),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.trunc()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot trunc infinity".into(),
            )),
        }
    }
}

impl PythonMathFunctions for Float {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok(NumberWrapper::from(
                    FloatWrapper::from_incomplete_float(self.rem(i)),
                )),
                IntegerWrapper::Int(integer) => Ok(NumberWrapper::from(
                    self.rem(FloatWrapper::from(integer).to_float()),
                )),
            },
            NumberWrapper::Rat(_) => Ok(self.rem(other.to_float()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self.rem(f)).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self.rem(float)).into()),
            },
        }
    }

    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match modulus {
            Some(m) => self.py_pow(other, None)?.py_mod(m),
            None => match other {
                NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(i)).into())
                    }
                    IntegerWrapper::Int(integer) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(integer)).into())
                    }
                },
                NumberWrapper::Rat(_) => Ok(self.pow(other.to_float()).into()),
                NumberWrapper::Float(float_wrapper) => match float_wrapper {
                    SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self.pow(f)).into()),
                    BigFloat(float) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(float)).into())
                    }
                },
            },
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => {
                    Ok(FloatWrapper::from_incomplete_float(self / i).into())
                }
                IntegerWrapper::Int(integer) => {
                    Ok(FloatWrapper::from_incomplete_float(self / integer).into())
                }
            },
            NumberWrapper::Rat(rational_wrapper) => Ok(self.div(other.to_float()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self / f).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self / float).into()),
            },
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(IntegerWrapper::ZERO);
        }
        self.py_truediv(other)?.py_floor()
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(NumberWrapper::from(self.clone().abs()))
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(
            match self.clone().trunc().to_integer() {
                Some(res) => res,
                None => {
                    return Err(FirkinError::CannotConvertInfinity(String::from(
                        "Cannot convert infinity to integer",
                    )));
                }
            },
        ))
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(self.to_f64())
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        let mul = FloatWrapper::from(10).to_float().pow(ndigits);
        Ok(NumberWrapper::from(
            match FloatWrapper::from_incomplete_float(self * &mul).round() {
                Ok(res) => FloatWrapper::from(res).to_float() / mul,
                Err(e) => {
                    return Err(FirkinError::CannotConvertInfinity(
                        "Cannot round infinity".into(),
                    ));
                }
            },
        ))
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().floor().try_into() {
            Ok(res) => Ok(res),
            Err(e) => Err(FirkinError::CannotConvertInfinity(
                "Cannot floor infinity".into(),
            )),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().ceil().try_into() {
            Ok(res) => Ok(res),
            Err(e) => Err(FirkinError::CannotConvertInfinity(
                "Cannot ceil infinity".into(),
            )),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().trunc().try_into() {
            Ok(res) => Ok(res),
            Err(e) => Err(FirkinError::CannotConvertInfinity(
                "Cannot trunc infinity".into(),
            )),
        }
    }
}

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
