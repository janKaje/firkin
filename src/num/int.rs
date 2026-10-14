#![allow(unused)]

use crate::error::FirkinError;
use crate::num::traits::{
    FromInts, NumpyUFuncs, PythonInt, PythonMathFunctions, PythonNum, PythonRat, RichDisplay,
    TryFromFloats, auto_impl_numpy_ufunc_trait, auto_impl_pymath_trait,
};
use crate::num::{float::FloatWrapper, number::NumberWrapper, rat::RationalWrapper};
use pyo3::IntoPyObject;
use pyo3::prelude::*;
use pyo3::types::PyInt;
use regex::regex;
use rug::integer::{IntegerExt64, Order};
use rug::ops::Pow;
use rug::{Float, Integer, az::Cast, float::MiniFloat};
use std::convert::Infallible;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

/// Wraps around the logic for both small and large integers
#[derive(Clone, Debug, Hash)]
pub(crate) enum IntegerWrapper {
    SmallInt(i64),
    Int(Integer),
}

use IntegerWrapper::{Int, SmallInt};

impl IntegerWrapper {
    pub(crate) const ZERO: Self = IntegerWrapper::SmallInt(0);
    pub(crate) const ONE: Self = IntegerWrapper::SmallInt(1);

    pub(crate) const fn is_positive(&self) -> bool {
        match self {
            SmallInt(i) => i.is_positive(),
            Int(i) => i.is_positive(),
        }
    }

    pub(crate) const fn is_negative(&self) -> bool {
        match self {
            SmallInt(i) => i.is_negative(),
            Int(i) => i.is_negative(),
        }
    }

    pub(crate) const fn is_zero(&self) -> bool {
        match self {
            SmallInt(i) => *i == 0,
            Int(i) => i.is_zero(),
        }
    }

    pub(crate) fn as_i64(&self) -> i64 {
        match self {
            SmallInt(i) => *i,
            Int(i) => i.to_i64_wrapping(),
        }
    }

    pub(crate) fn to_i32(&self) -> Result<i32, &'static str> {
        match self {
            SmallInt(i) => {
                if *i > i32::MAX as i64 || *i < i32::MIN as i64 {
                    Err("Overflow")
                } else {
                    Ok(*i as i32)
                }
            }
            Int(integer) => match integer.to_i32() {
                Some(i) => Ok(i),
                None => Err("Overflow"),
            },
        }
    }

    pub(crate) fn to_u32(&self) -> Result<u32, &'static str> {
        match self {
            SmallInt(i) => {
                if *i > u32::MAX as i64 || *i < 0 {
                    Err("Overflow")
                } else {
                    Ok(*i as u32)
                }
            }
            Int(integer) => match integer.to_u32() {
                Some(i) => Ok(i),
                None => Err("Overflow"),
            },
        }
    }

    pub(crate) fn as_integer(&self) -> Integer {
        match self {
            SmallInt(i) => (*i).into(),
            Int(i) => i.clone(),
        }
    }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            SmallInt(i) => *i as f64,
            Int(integer) => integer.to_f64(),
        }
    }

    pub(crate) fn as_u32(&self) -> u32 {
        match self {
            SmallInt(i) => i.cast(),
            Int(integer) => integer.cast(),
        }
    }

    pub(crate) fn abs(&self) -> IntegerWrapper {
        match self {
            SmallInt(i) => SmallInt(i.abs()),
            Int(integer) => Int(integer.clone().abs()),
        }
    }

    pub(crate) fn floor(&self) -> IntegerWrapper {
        self.clone()
    }

    pub(crate) fn ceil(&self) -> IntegerWrapper {
        self.clone()
    }

    pub(crate) fn round(&self) -> IntegerWrapper {
        self.clone()
    }
}

impl std::fmt::Display for IntegerWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmallInt(i) => i.fmt(f),
            Int(integer) => integer.fmt(f),
        }
    }
}

macro_rules! arith_binary {
    (
        $checked_func:ident;
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $Imp<IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(integer2)))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(integer2)))
                    }
                }
            }
        }

        impl $ImpAssign<IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: IntegerWrapper) {
                self.$method_assign(&rhs);
            }
        }

        impl $ImpAssign<&IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: &IntegerWrapper) {
                match (&self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => *self = SmallInt(r),
                        None => *self = Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        *self = IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), SmallInt(i)) => {
                        *self = IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), Int(integer2)) => {
                        *self = IntegerWrapper::from(Integer::from(integer2.$method(integer)))
                    }
                }
            }
        }
    };
}

arith_binary! {
    checked_add;
    Add { add }
    AddAssign { add_assign }
}

arith_binary! {
    checked_sub;
    Sub { sub }
    SubAssign { sub_assign }
}

arith_binary! {
    checked_mul;
    Mul { mul }
    MulAssign { mul_assign }
}

arith_binary! {
    checked_div;
    Div { div }
    DivAssign { div_assign }
}

impl Shl<u32> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: u32) -> Self::Output {
        match self {
            SmallInt(i) => SmallInt(i.shl(rhs)),
            Int(integer) => IntegerWrapper::from(integer.shl(rhs)),
        }
    }
}

impl Shr<u32> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            SmallInt(i) => SmallInt(i.shr(rhs)),
            Int(integer) => IntegerWrapper::from(integer.shr(rhs)),
        }
    }
}

impl ShlAssign<u32> for IntegerWrapper {
    fn shl_assign(&mut self, rhs: u32) {
        match self {
            SmallInt(i) => *self = SmallInt(i.shl(rhs)),
            Int(integer) => integer.shl_assign(rhs),
        }
    }
}

impl ShrAssign<u32> for IntegerWrapper {
    fn shr_assign(&mut self, rhs: u32) {
        match self {
            SmallInt(i) => *self = SmallInt(i.shr(rhs)),
            Int(integer) => integer.shr_assign(rhs),
        }
    }
}

impl Shl<u32> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: u32) -> Self::Output {
        match self {
            SmallInt(i) => SmallInt(i.shl(rhs)),
            Int(integer) => IntegerWrapper::from(Integer::from(integer.shl(rhs))),
        }
    }
}

impl Shr<u32> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: u32) -> Self::Output {
        match self {
            SmallInt(i) => SmallInt(i.shr(rhs)),
            Int(integer) => IntegerWrapper::from(Integer::from(integer.shr(rhs))),
        }
    }
}

macro_rules! arith_unary {
    (
        $Imp:ident { $method:ident }
    ) => {
        impl $Imp for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self) -> Self::Output {
                match self {
                    SmallInt(i) => SmallInt(i.$method()),
                    Int(integer) => Int(integer.$method()),
                }
            }
        }

        impl $Imp for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self) -> Self::Output {
                match self {
                    SmallInt(i) => SmallInt(i.$method()),
                    Int(integer) => Int(Integer::from(integer.$method())),
                }
            }
        }
    };
}

arith_unary! {Neg {neg}}
arith_unary! {Not {not}}

macro_rules! bitwise_binary {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: IntegerWrapper) -> Self::Output {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => SmallInt(i.$method(i2)),
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $ImpAssign<IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: IntegerWrapper) {
                match (&mut *self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => i.$method_assign(i2),
                    (SmallInt(i), Int(integer)) => *self = IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => integer.$method_assign(i),
                    (Int(integer), Int(integer2)) => integer.$method_assign(integer2),
                }
            }
        }

        impl $Imp<&IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> Self::Output {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => SmallInt(i.$method(i2)),
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $ImpAssign<&IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: &IntegerWrapper) {
                match (&mut *self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => i.$method_assign(i2),
                    (SmallInt(i), Int(integer)) => {
                        *self = IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), SmallInt(i)) => integer.$method_assign(i),
                    (Int(integer), Int(integer2)) => integer.$method_assign(integer2),
                }
            }
        }

        impl $Imp<IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: IntegerWrapper) -> Self::Output {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => SmallInt(i.$method(i2)),
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> Self::Output {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => SmallInt(i.$method(i2)),
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(integer2)))
                    }
                }
            }
        }
    };
}

bitwise_binary! {
    BitAnd {bitand}
    BitAndAssign {bitand_assign}
}
bitwise_binary! {
    BitOr {bitor}
    BitOrAssign {bitor_assign}
}
bitwise_binary! {
    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
}

impl Rem<IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.rem(integer)),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.rem(i)),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl RemAssign<IntegerWrapper> for IntegerWrapper {
    fn rem_assign(&mut self, rhs: IntegerWrapper) {
        match (&mut *self, rhs) {
            (SmallInt(i), SmallInt(i2)) => i.rem_assign(i2),
            (SmallInt(i), Int(integer)) => *self = IntegerWrapper::from(i.rem(integer)),
            (Int(integer), SmallInt(i)) => integer.rem_assign(i),
            (Int(integer), Int(integer1)) => integer.rem_assign(integer1),
        }
    }
}

impl Rem<&IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: &IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.rem(i)),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl RemAssign<&IntegerWrapper> for IntegerWrapper {
    fn rem_assign(&mut self, rhs: &IntegerWrapper) {
        match (&mut *self, rhs) {
            (SmallInt(i), SmallInt(i2)) => i.rem_assign(i2),
            (SmallInt(i), Int(integer)) => {
                *self = IntegerWrapper::from(Integer::from(i.rem(integer)))
            }
            (Int(integer), SmallInt(i)) => integer.rem_assign(i),
            (Int(integer), Int(integer1)) => integer.rem_assign(integer1),
        }
    }
}

impl Rem<IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(Integer::from(integer.rem(i))),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl Rem<&IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: &IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(Integer::from(integer.rem(i))),
            (Int(integer), Int(integer1)) => {
                IntegerWrapper::from(Integer::from(integer.rem(integer1)))
            }
        }
    }
}

impl From<i128> for IntegerWrapper {
    fn from(value: i128) -> Self {
        if value > i64::MAX as i128 || value < i64::MIN as i128 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<u128> for IntegerWrapper {
    fn from(value: u128) -> Self {
        if value > i64::MAX as u128 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<u64> for IntegerWrapper {
    fn from(value: u64) -> Self {
        if value > i64::MAX as u64 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<usize> for IntegerWrapper {
    fn from(value: usize) -> Self {
        if value > i64::MAX as usize {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<Integer> for IntegerWrapper {
    fn from(value: Integer) -> Self {
        if value > Integer::from(i64::MAX) || value < Integer::from(i64::MIN) {
            Int(value.into())
        } else {
            SmallInt(value.to_i64_wrapping())
        }
    }
}

impl From<&Integer> for IntegerWrapper {
    fn from(value: &Integer) -> Self {
        if value > &Integer::from(i64::MAX) || value < &Integer::from(i64::MIN) {
            Int(value.into())
        } else {
            SmallInt(value.to_i64_wrapping())
        }
    }
}

impl From<i8> for IntegerWrapper {
    fn from(value: i8) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i16> for IntegerWrapper {
    fn from(value: i16) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i32> for IntegerWrapper {
    fn from(value: i32) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i64> for IntegerWrapper {
    fn from(value: i64) -> Self {
        SmallInt(value)
    }
}

impl From<isize> for IntegerWrapper {
    fn from(value: isize) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u8> for IntegerWrapper {
    fn from(value: u8) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u16> for IntegerWrapper {
    fn from(value: u16) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u32> for IntegerWrapper {
    fn from(value: u32) -> Self {
        SmallInt(value as i64)
    }
}

impl From<IntegerWrapper> for Integer {
    fn from(value: IntegerWrapper) -> Self {
        value.as_integer()
    }
}

impl From<&IntegerWrapper> for Integer {
    fn from(value: &IntegerWrapper) -> Self {
        value.as_integer()
    }
}

impl TryFrom<f32> for IntegerWrapper {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<IntegerWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else if value.abs() < i64::MAX as f32 {
            Ok(SmallInt(value as i64))
        } else {
            Ok(Int(Integer::from_f32(value).expect("unreachable")))
        }
    }
}

impl TryFrom<f64> for IntegerWrapper {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<IntegerWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else if value.abs() < i64::MAX as f64 {
            Ok(SmallInt(value as i64))
        } else {
            Ok(Int(Integer::from_f64(value).expect("unreachable")))
        }
    }
}

impl TryFrom<Float> for IntegerWrapper {
    type Error = &'static str;

    fn try_from(value: Float) -> Result<IntegerWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else if value <= i64::MAX && value >= i64::MIN {
            Ok(SmallInt(value.cast()))
        } else {
            Ok(Int(value.cast()))
        }
    }
}

impl PartialEq for IntegerWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SmallInt(i), SmallInt(i2)) => i.eq(i2),
            (SmallInt(i), Int(integer)) => i.eq(integer),
            (Int(integer), SmallInt(i)) => integer.eq(i),
            (Int(integer), Int(integer2)) => integer.eq(integer2),
        }
    }
}

impl PartialOrd for IntegerWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SmallInt(i), SmallInt(i2)) => i.partial_cmp(i2),
            (SmallInt(i), Int(integer)) => i.partial_cmp(integer),
            (Int(integer), SmallInt(i)) => integer.partial_cmp(i),
            (Int(integer), Int(integer2)) => integer.partial_cmp(integer2),
        }
    }
}

impl PartialEq<&IntegerWrapper> for IntegerWrapper {
    fn eq(&self, other: &&IntegerWrapper) -> bool {
        (&self).eq(other)
    }
}

impl PartialOrd<&IntegerWrapper> for IntegerWrapper {
    fn partial_cmp(&self, other: &&IntegerWrapper) -> Option<std::cmp::Ordering> {
        (&self).partial_cmp(other)
    }
}

impl PartialEq<IntegerWrapper> for &IntegerWrapper {
    fn eq(&self, other: &IntegerWrapper) -> bool {
        self.eq(&other)
    }
}

impl PartialOrd<IntegerWrapper> for &IntegerWrapper {
    fn partial_cmp(&self, other: &IntegerWrapper) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other)
    }
}

impl FromInts for IntegerWrapper {}

impl TryFromFloats for IntegerWrapper {}

auto_impl_pymath_trait! {IntegerWrapper {SmallInt Int}}
auto_impl_numpy_ufunc_trait! {IntegerWrapper {Int SmallInt}}

impl PythonNum for IntegerWrapper {}
impl PythonRat for IntegerWrapper {}

impl PythonInt for IntegerWrapper {
    fn py_index(&self) -> Result<IntegerWrapper, FirkinError> {
        self.py_int()
    }
}

impl FromStr for IntegerWrapper {
    type Err = FirkinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(i) => Ok(i.into()),
            Err(_) => match s.parse::<Integer>() {
                Ok(integer) => Ok(integer.into()),
                Err(e) => Err(FirkinError::CannotConvertToInteger(
                    s.into(),
                    Some(e.to_string()),
                )),
            },
        }
    }
}

impl RichDisplay for IntegerWrapper {
    fn display_default(&self) -> String {
        self.to_string()
    }

    fn display_debug(&self) -> String {
        match self {
            SmallInt(i) => format!("<SmallInteger {}>", i),
            Int(integer) => format!("<BigInteger {}>", integer),
        }
    }

    fn format(&self, format_specifier: String) -> String {
        todo!()
    }
}

impl<'py> IntoPyObject<'py> for IntegerWrapper {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SmallInt(i) => i.into_pyobject(py),
            Int(integer) => Ok(PyModule::import(py, "builtins")
                .unwrap()
                .getattr("int")
                .unwrap()
                .call1((integer.to_string_radix(10),))
                .unwrap()
                .cast_into::<PyInt>()
                .unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::FirkinError, num::int::IntegerWrapper};
    use rug::Integer;

    #[test]
    fn wrapper_arith() {
        let small1 = IntegerWrapper::from(2);
        let big1 = IntegerWrapper::from(Integer::from(i64::MAX) * 2);
        assert_eq!(&small1 + &small1, IntegerWrapper::from(4));
        assert_eq!(&big1 - &big1, IntegerWrapper::from(0));
        assert_eq!(
            &big1 * &small1,
            IntegerWrapper::from(36893488147419103228_i128)
        );
        assert_eq!(&small1 / &big1, IntegerWrapper::from(i64::MAX));
        assert_eq!(&big1 % &small1, IntegerWrapper::from(0));
    }

    #[test]
    fn from_str() -> Result<(), FirkinError> {
        let smalli = "1284";
        let bigi = "1242310597164756218374132";
        let noti = "19283jf";
        let negi = "-91273";
        assert_eq!(
            IntegerWrapper::from(1284),
            smalli.parse::<IntegerWrapper>()?
        );
        assert_eq!(
            IntegerWrapper::from(1242310597164756218374132_i128),
            bigi.parse::<IntegerWrapper>()?
        );
        assert!(noti.parse::<IntegerWrapper>().is_err());
        assert_eq!(
            IntegerWrapper::from(-91273),
            negi.parse::<IntegerWrapper>()?
        );
        Ok(())
    }
}
