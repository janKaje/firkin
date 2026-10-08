#![allow(unused)]

use crate::error::FirkinError;
use crate::num::traits::{FromInts, PythonMathFunctions, TryFromFloats};
use crate::num::{float::FloatWrapper, number::NumberWrapper, rat::RationalWrapper};
use rug::integer::IntegerExt64;
use rug::ops::Pow;
use rug::{Float, Integer, az::Cast, float::MiniFloat};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Wraps around the logic for both small and large integers
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum IntegerWrapper {
    SmallInt(i64),
    Int(Integer),
}

use IntegerWrapper::{Int, SmallInt};

impl IntegerWrapper {
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

    pub(crate) fn as_i64(&self) -> i64 {
        match self {
            SmallInt(i) => *i,
            Int(i) => i.to_i64_wrapping(),
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

impl Shl<IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shl(rhs)),
                Int(integer) => IntegerWrapper::from(integer.shl(rhs)),
            }
        } else {
            self.shr(rhs.abs())
        }
    }
}

impl Shr<IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shr(rhs)),
                Int(integer) => IntegerWrapper::from(integer.shr(rhs)),
            }
        } else {
            self.shl(rhs.abs())
        }
    }
}

impl ShlAssign<IntegerWrapper> for IntegerWrapper {
    fn shl_assign(&mut self, rhs: IntegerWrapper) {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => *self = SmallInt(i.shl(rhs)),
                Int(integer) => integer.shl_assign(rhs),
            }
        } else {
            self.shr_assign(rhs.abs())
        }
    }
}

impl ShrAssign<IntegerWrapper> for IntegerWrapper {
    fn shr_assign(&mut self, rhs: IntegerWrapper) {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => *self = SmallInt(i.shr(rhs)),
                Int(integer) => integer.shr_assign(rhs),
            }
        } else {
            self.shl_assign(rhs.abs())
        }
    }
}

impl Shl<&IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: &IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shl(rhs)),
                Int(integer) => IntegerWrapper::from(integer.shl(rhs)),
            }
        } else {
            self.shr(rhs.abs())
        }
    }
}

impl Shr<&IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: &IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shr(rhs)),
                Int(integer) => IntegerWrapper::from(integer.shr(rhs)),
            }
        } else {
            self.shl(rhs.abs())
        }
    }
}

impl ShlAssign<&IntegerWrapper> for IntegerWrapper {
    fn shl_assign(&mut self, rhs: &IntegerWrapper) {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => *self = SmallInt(i.shl(rhs)),
                Int(integer) => integer.shl_assign(rhs),
            }
        } else {
            self.shr_assign(rhs.abs())
        }
    }
}

impl ShrAssign<&IntegerWrapper> for IntegerWrapper {
    fn shr_assign(&mut self, rhs: &IntegerWrapper) {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => *self = SmallInt(i.shr(rhs)),
                Int(integer) => integer.shr_assign(rhs),
            }
        } else {
            self.shl_assign(rhs.abs())
        }
    }
}

impl Shl<IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shl(rhs)),
                Int(integer) => IntegerWrapper::from(Integer::from(integer.shl(rhs))),
            }
        } else {
            self.shr(rhs.abs())
        }
    }
}

impl Shr<IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shr(rhs)),
                Int(integer) => IntegerWrapper::from(Integer::from(integer.shr(rhs))),
            }
        } else {
            self.shl(rhs.abs())
        }
    }
}

impl Shl<&IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shl(self, rhs: &IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shl(rhs)),
                Int(integer) => IntegerWrapper::from(Integer::from(integer.shl(rhs))),
            }
        } else {
            self.shr(rhs.abs())
        }
    }
}

impl Shr<&IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn shr(self, rhs: &IntegerWrapper) -> Self::Output {
        if rhs.is_positive() {
            let rhs = rhs.abs().as_u32();
            match self {
                SmallInt(i) => SmallInt(i.shr(rhs)),
                Int(integer) => IntegerWrapper::from(Integer::from(integer.shr(rhs))),
            }
        } else {
            self.shl(rhs.abs())
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

impl FromInts for IntegerWrapper {}

impl TryFromFloats for IntegerWrapper {}

impl PythonMathFunctions for IntegerWrapper {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
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
}

impl PythonMathFunctions for i64 {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        todo!()
        // match other {
        //     NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
        //         SmallInt(i_exp) => match modulus {
        //             Some(m) => match m {
        //                 NumberWrapper::Int(integer_mod) => {
        //                     match Integer::from(*self).pow_mod(&Integer::from(*i_exp), &integer_mod.as_integer()) {
        //                         Ok(result) => Ok(NumberWrapper::from(result)),
        //                         Err(_) => Err(FirkinError::PowModDoesNotExist(self.to_string(), other.to_string(), modulus.to_string())),
        //                     }
        //                 },
        //                 _ => FloatWrapper::from(*self).py_pow(other, modulus),
        //             },
        //             None => if i_exp.is_positive() {
        //                 Ok(NumberWrapper::from(Integer::from(Integer::i64_pow_u64(
        //                     *self,
        //                     i_exp.unsigned_abs(),
        //                 ))))
        //             } else {
        //                 Ok(NumberWrapper::Rat(RationalWrapper::from((
        //                     Integer::ONE,
        //                     Integer::i64_pow_u64(*self, i_exp.unsigned_abs()),
        //                 ))))
        //             },
        //         }
        //         Int(integer_exp) => FloatWrapper::from(*self).py_pow(other, modulus),
        //     },
        //     _ => FloatWrapper::from(*self).py_pow(other, modulus),
        // }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(NumberWrapper::from(self.abs()))
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(*self as f64)
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if ndigits < 0 {
            let pow = match 10_i64.checked_pow(ndigits.unsigned_abs()) {
                Some(r) => r,
                None => {
                    if ndigits == 20 {
                        if self >= &5000000000000000000 {
                            return Ok(NumberWrapper::from(10000000000000000000_i128));
                        } else {
                            return Ok(NumberWrapper::from(0));
                        }
                    } else {
                        return Ok(NumberWrapper::from(0));
                    }
                }
            };
            let rem = self % pow;
            let result;
            if rem >= pow / 2 {
                result = self - rem + pow;
            } else {
                result = self - rem;
            }
            Ok(NumberWrapper::from(result))
        } else {
            Ok(NumberWrapper::from(*self))
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use crate::num::int::IntegerWrapper;
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
}
