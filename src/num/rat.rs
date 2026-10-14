#![allow(unused)]

use crate::error::FirkinError;
use crate::num::int::IntegerWrapper::{self, Int, SmallInt};
use crate::num::smallrational::SmallRational;
use crate::num::traits::{
    FromFloats, FromInts, NumpyUFuncs, PythonMathFunctions, PythonNum, PythonRat, RichDisplay,
    TryFromFloats, auto_impl_numpy_ufunc_trait, auto_impl_pymath_trait,
};
use crate::num::{float::FloatWrapper, number::NumberWrapper};

use regex::regex;
use rug::Float;
use rug::ops::PowAssign;
use rug::{Integer, Rational, ops::Pow};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Wraps around the logic for both small and large rationals
#[derive(Clone, Debug, Hash)]
pub(crate) enum RationalWrapper {
    SmallRat(SmallRational),
    Rat(Rational),
}

use RationalWrapper::{Rat, SmallRat};

impl RationalWrapper {
    pub(crate) fn from_f32(value: f32) -> Option<Self> {
        Some(RationalWrapper::from(Rational::from_f32(value)?))
    }

    pub(crate) fn from_f64(value: f64) -> Option<Self> {
        Some(RationalWrapper::from(Rational::from_f64(value)?))
    }

    pub(crate) fn from_str_radix(src: &str, radix: i32) -> Option<Self> {
        match Rational::from_str_radix(src, radix) {
            Ok(result) => Some(RationalWrapper::from(result)),
            Err(_) => None,
        }
    }

    pub(crate) fn from_small_num_denom<Num, Den>(num: Num, den: Den) -> Self
    where
        i64: From<Num> + From<Den>,
    {
        SmallRat(SmallRational::from((num, den)))
    }

    pub(crate) fn from_decimal_str(src: &str) -> Result<Self, FirkinError> {
        let m = match regex!(r"\s*([-+]?\d*).?(\d*)[eE]?([-+]?)(\d*)\s*").captures(src) {
            Some(m) => m,
            None => {
                return Err(FirkinError::CannotConvertToNumber(
                    src.into(),
                    Some("from_decimal_str regex failed to capture".into()),
                ));
            }
        };

        // be sure to test this thoroughly
        let exp: u32;
        if (&m[4]).len() > 0 {
            exp = match (&m[4]).parse::<u32>() {
                Ok(e) => e,
                Err(e) => {
                    return Err(FirkinError::CannotConvertToNumber(
                        src.into(),
                        Some(e.to_string()),
                    ));
                }
            };
        } else {
            exp = 0
        }

        let neg_exp = &m[3] == "-";

        let whole: Integer = Integer::parse(&m[1]).unwrap().into();

        let frac_denom: Integer;
        let as_rat: Rational;
        if (&m[2]).len() > 0 {
            frac_denom = Integer::from(10).pow((&m[2]).len() as u32);
            if !whole.is_negative() {
                as_rat = Rational::from((
                    whole * &frac_denom + Integer::from(Integer::parse(&m[2]).unwrap()),
                    frac_denom,
                ));
            } else {
                as_rat = Rational::from((
                    whole * &frac_denom - Integer::from(Integer::parse(&m[2]).unwrap()),
                    frac_denom,
                ));
            }
        } else {
            as_rat = Rational::from((whole, 1));
        }

        if exp == 0 {
            Ok(RationalWrapper::from(as_rat))
        } else if !neg_exp {
            Ok(RationalWrapper::from(as_rat * Integer::from(10).pow(exp)))
        } else {
            Ok(RationalWrapper::from(as_rat / Integer::from(10).pow(exp)))
        }
    }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            SmallRat(r) => r.into(),
            Rat(rational) => rational.to_f64(),
        }
    }

    pub(crate) fn abs(&self) -> RationalWrapper {
        match self {
            SmallRat(r) => SmallRat(r.abs()),
            Rat(rational) => Rat(rational.clone().abs()),
        }
    }

    pub(crate) fn floor(&self) -> IntegerWrapper {
        match self {
            SmallRat(r) => IntegerWrapper::from(r.floor()),
            Rat(rational) => IntegerWrapper::from(rational.clone().floor().into_numer_denom().0),
        }
    }

    pub(crate) fn ceil(&self) -> IntegerWrapper {
        match self {
            SmallRat(r) => IntegerWrapper::from(r.ceil()),
            Rat(rational) => IntegerWrapper::from(rational.clone().ceil().into_numer_denom().0),
        }
    }

    pub(crate) fn round(&self) -> IntegerWrapper {
        match self {
            SmallRat(r) => IntegerWrapper::from(r.round()),
            Rat(rational) => IntegerWrapper::from(rational.clone().round().into_numer_denom().0),
        }
    }

    pub(crate) fn is_integer(&self) -> bool {
        match self {
            SmallRat(r) => r.is_integer(),
            Rat(rational) => rational.is_integer(),
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self {
            SmallRat(r) => r.is_zero(),
            Rat(rational) => rational.is_zero(),
        }
    }

    pub(crate) fn to_integer(&self) -> IntegerWrapper {
        match self {
            SmallRat(r) => IntegerWrapper::from(r.get_whole_number_part()),
            Rat(rational) => IntegerWrapper::from(rational.clone().fract_trunc(Integer::new()).1),
        }
    }
}

macro_rules! arith_binary {
    (
        $checked_func:ident;
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<RationalWrapper> for RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: RationalWrapper) -> RationalWrapper {
                (&self).$method(&rhs)
            }
        }

        impl $Imp<&RationalWrapper> for RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &RationalWrapper) -> RationalWrapper {
                (&self).$method(rhs)
            }
        }

        impl $Imp<RationalWrapper> for &RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: RationalWrapper) -> RationalWrapper {
                self.$method(&rhs)
            }
        }

        impl $Imp<&RationalWrapper> for &RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(r), SmallRat(r2)) => match r.$checked_func(r2) {
                        Some(res) => SmallRat(res),
                        None => RationalWrapper::from(Rational::from(r).$method(r2)),
                    },
                    (SmallRat(r), Rat(rational)) => RationalWrapper::from(r.$method(rational)),
                    (Rat(rational), SmallRat(r)) => RationalWrapper::from(rational.$method(r)),
                    (Rat(rational), Rat(rational2)) => {
                        RationalWrapper::from(Rational::from(rational.$method(rational2)))
                    }
                }
            }
        }

        impl $Imp<IntegerWrapper> for RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: IntegerWrapper) -> RationalWrapper {
                (&self).$method(&rhs)
            }
        }

        impl $Imp<&IntegerWrapper> for RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> RationalWrapper {
                (&self).$method(rhs)
            }
        }

        impl $Imp<IntegerWrapper> for &RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: IntegerWrapper) -> RationalWrapper {
                self.$method(&rhs)
            }
        }

        impl $Imp<&IntegerWrapper> for &RationalWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(r), SmallInt(i)) => match r.$checked_func(*i) {
                        Some(res) => SmallRat(res),
                        None => RationalWrapper::from(Rational::from(r).$method(i)),
                    },
                    (SmallRat(r), Int(integer)) => {
                        RationalWrapper::from(Rational::from(r).$method(integer))
                    }
                    (Rat(rational), SmallInt(i)) => {
                        RationalWrapper::from(Rational::from(rational.$method(i)))
                    }
                    (Rat(rational), Int(integer)) => {
                        RationalWrapper::from(Rational::from(rational.$method(integer)))
                    }
                }
            }
        }
        impl $Imp<RationalWrapper> for IntegerWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: RationalWrapper) -> RationalWrapper {
                (&self).$method(&rhs)
            }
        }

        impl $Imp<&RationalWrapper> for IntegerWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &RationalWrapper) -> RationalWrapper {
                (&self).$method(rhs)
            }
        }

        impl $Imp<RationalWrapper> for &IntegerWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: RationalWrapper) -> RationalWrapper {
                self.$method(&rhs)
            }
        }

        impl $Imp<&RationalWrapper> for &IntegerWrapper {
            type Output = RationalWrapper;

            fn $method(self, rhs: &RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallRat(r)) => {
                        RationalWrapper::from(Rational::from(*i).$method(r))
                    }
                    (SmallInt(i), Rat(rational)) => {
                        RationalWrapper::from(Rational::from(i.$method(rational)))
                    }
                    (Int(integer), SmallRat(r)) => {
                        RationalWrapper::from(Rational::from(integer.$method(Rational::from(r))))
                    }
                    (Int(integer), Rat(rational)) => {
                        RationalWrapper::from(Rational::from(integer.$method(rational)))
                    }
                }
            }
        }

        impl $ImpAssign<RationalWrapper> for RationalWrapper {
            fn $method_assign(&mut self, rhs: RationalWrapper) {
                self.$method_assign(&rhs);
            }
        }

        impl $ImpAssign<&RationalWrapper> for RationalWrapper {
            fn $method_assign(&mut self, rhs: &RationalWrapper) {
                match (&self, rhs) {
                    (SmallRat(r), SmallRat(r2)) => match r.$checked_func(r2) {
                        Some(res) => *self = SmallRat(res),
                        None => *self = RationalWrapper::from(Rational::from(r).$method(r2)),
                    },
                    (SmallRat(r), Rat(rational)) => {
                        *self = RationalWrapper::from(Rational::from(rational.$method(r)))
                    }
                    (Rat(rational), SmallRat(r)) => {
                        *self = RationalWrapper::from(Rational::from(r.$method(rational)))
                    }
                    (Rat(rational), Rat(rational2)) => {
                        *self = RationalWrapper::from(Rational::from(rational2.$method(rational)))
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

impl Neg for RationalWrapper {
    type Output = RationalWrapper;

    fn neg(self) -> Self::Output {
        match self {
            SmallRat(small_rational) => SmallRat(-small_rational),
            Rat(rational) => Rat(-rational),
        }
    }
}

impl Display for RationalWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmallRat(small_rational) => small_rational.fmt(f),
            Rat(rational) => rational.fmt(f),
        }
    }
}

impl PartialEq for RationalWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SmallRat(r), SmallRat(r2)) => r == r2,
            (SmallRat(r), Rat(rational)) => {
                r.numerator == *rational.numer()
                    && r.denominator == *rational.denom()
                    && r.is_positive() == rational.is_positive()
            }
            (Rat(rational), SmallRat(r)) => {
                r.numerator == *rational.numer()
                    && r.denominator == *rational.denom()
                    && r.is_positive() == rational.is_positive()
            }
            (Rat(rational), Rat(rational2)) => rational == rational2,
        }
    }
}

impl PartialEq<IntegerWrapper> for RationalWrapper {
    fn eq(&self, other: &IntegerWrapper) -> bool {
        if !self.is_integer() {
            return false;
        }
        self.to_integer().eq(other)
    }
}

impl PartialEq<RationalWrapper> for IntegerWrapper {
    fn eq(&self, other: &RationalWrapper) -> bool {
        if !other.is_integer() {
            return false;
        }
        other.to_integer().eq(self)
    }
}

impl PartialOrd<IntegerWrapper> for RationalWrapper {
    fn partial_cmp(&self, other: &IntegerWrapper) -> Option<std::cmp::Ordering> {
        if self.is_integer() {
            self.to_integer().partial_cmp(other)
        } else {
            if self.floor() >= other {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        }
    }
}

impl PartialOrd<RationalWrapper> for IntegerWrapper {
    fn partial_cmp(&self, other: &RationalWrapper) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl From<SmallRational> for RationalWrapper {
    fn from(value: SmallRational) -> Self {
        SmallRat(value)
    }
}

impl From<Rational> for RationalWrapper {
    fn from(value: Rational) -> Self {
        let numer = value.numer().clone().abs();
        if numer < u64::MAX && value.denom() < &u64::MAX {
            SmallRat(SmallRational::new(
                numer.to_u64_wrapping(),
                value.denom().to_u64_wrapping(),
                value.is_positive(),
            ))
        } else {
            Rat(value)
        }
    }
}

impl From<i8> for RationalWrapper {
    fn from(value: i8) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<i16> for RationalWrapper {
    fn from(value: i16) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<i32> for RationalWrapper {
    fn from(value: i32) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<i64> for RationalWrapper {
    fn from(value: i64) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<i128> for RationalWrapper {
    fn from(value: i128) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl From<isize> for RationalWrapper {
    fn from(value: isize) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<u8> for RationalWrapper {
    fn from(value: u8) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<u16> for RationalWrapper {
    fn from(value: u16) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<u32> for RationalWrapper {
    fn from(value: u32) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<u64> for RationalWrapper {
    fn from(value: u64) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<u128> for RationalWrapper {
    fn from(value: u128) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl From<usize> for RationalWrapper {
    fn from(value: usize) -> Self {
        SmallRat(SmallRational::from(value))
    }
}

impl From<Integer> for RationalWrapper {
    fn from(value: Integer) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl From<&Integer> for RationalWrapper {
    fn from(value: &Integer) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl From<&Rational> for RationalWrapper {
    fn from(value: &Rational) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl<Num, Den> From<(Num, Den)> for RationalWrapper
where
    Integer: From<Num> + From<Den>,
{
    fn from(value: (Num, Den)) -> Self {
        RationalWrapper::from(Rational::from(value))
    }
}

impl TryFrom<f32> for RationalWrapper {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<RationalWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else {
            Ok(RationalWrapper::from(
                Rational::from_f32(value).expect("unreachable"),
            ))
        }
    }
}

impl TryFrom<f64> for RationalWrapper {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<RationalWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else {
            Ok(RationalWrapper::from(
                Rational::from_f64(value).expect("unreachable"),
            ))
        }
    }
}

impl TryFrom<Float> for RationalWrapper {
    type Error = &'static str;

    fn try_from(value: Float) -> Result<RationalWrapper, &'static str> {
        if value.is_infinite() {
            Err("Value is infinite.")
        } else {
            Ok(RationalWrapper::from(
                Rational::try_from(value).expect("unreachable"),
            ))
        }
    }
}

impl FromInts for RationalWrapper {}

impl TryFromFloats for RationalWrapper {}

impl PartialOrd for RationalWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SmallRat(r), SmallRat(r2)) => r.partial_cmp(r2),
            (SmallRat(r), Rat(rational)) => r.partial_cmp(rational),
            (Rat(rational), SmallRat(r)) => rational.partial_cmp(r),
            (Rat(rational), Rat(rational2)) => rational.partial_cmp(rational2),
        }
    }
}

impl PythonNum for RationalWrapper {}
impl PythonRat for RationalWrapper {}

auto_impl_pymath_trait! {RationalWrapper {Rat SmallRat}}
auto_impl_numpy_ufunc_trait! {RationalWrapper {Rat SmallRat}}

impl RichDisplay for RationalWrapper {
    fn display_default(&self) -> String {
        match self {
            SmallRat(r) => r.display_default(),
            Rat(rational) => format!("{:?}", rational.to_f64()),
        }
    }

    fn display_debug(&self) -> String {
        match self {
            SmallRat(r) => r.display_debug(),
            Rat(rational) => format!("<BigRational {}>", rational),
        }
    }

    fn format(&self, format_specifier: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::num::rat::RationalWrapper;

    #[test]
    fn froms() {
        assert_eq!(RationalWrapper::from(-1_i8), RationalWrapper::from(-1_i16));
        assert_eq!(RationalWrapper::from(3_i32), RationalWrapper::from(3_i64));
        assert_eq!(
            RationalWrapper::from(5_isize),
            RationalWrapper::from(5_i128)
        );
        assert_eq!(RationalWrapper::from(15_u8), RationalWrapper::from(15_u16));
        assert_eq!(RationalWrapper::from(14_u32), RationalWrapper::from(14_u64));
        assert_eq!(
            RationalWrapper::from(13_usize),
            RationalWrapper::from(13_u128)
        );
    }

    #[test]
    fn from_float() {
        assert_eq!(
            RationalWrapper::from_f64(3.125),
            RationalWrapper::from_f32(3.125)
        );
    }

    #[test]
    fn from_str_num_denom() {
        assert_ne!(
            RationalWrapper::from_str_radix("123/4", 10),
            RationalWrapper::from_str_radix("124/4", 10)
        );
        assert_eq!(
            RationalWrapper::from_str_radix("64/3", 10).unwrap(),
            RationalWrapper::from(64) / RationalWrapper::from(3)
        );
        assert_eq!(
            RationalWrapper::from_str_radix("-ff/a", 16).unwrap(),
            RationalWrapper::from_small_num_denom(-255, 10)
        );
    }

    #[test]
    fn from_decimal_str() {
        assert_eq!(
            RationalWrapper::from_decimal_str("+12.498e12").unwrap(),
            RationalWrapper::from_f64(12.498e12).unwrap()
        );
        assert_eq!(
            RationalWrapper::from_decimal_str("-1.12").unwrap(),
            RationalWrapper::from_small_num_denom(-112, 100)
        );
    }

    #[test]
    fn arith() {
        let test1 = RationalWrapper::from(-345);
        let test2 = RationalWrapper::from_f64(51.625).unwrap();
        let test3 = RationalWrapper::from_decimal_str("513.43").unwrap();
        let test4 = RationalWrapper::from_str_radix("-5/124", 10).unwrap();

        assert_eq!(
            dbg!(&test1 + &test2),
            RationalWrapper::from_f64(51.625 - 345.0).unwrap()
        );
        assert_eq!(
            &test2 - &test3,
            RationalWrapper::from_decimal_str("-461.805").unwrap()
        );
        assert_eq!(
            &test3 * &test4,
            RationalWrapper::from_str_radix("-51343/2480", 10).unwrap()
        );
        assert_eq!(
            &test1 / &test4,
            RationalWrapper::from_str_radix("42780/5", 10).unwrap()
        );
    }
}
