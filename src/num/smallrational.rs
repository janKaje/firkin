#![allow(unused)]

use std::{
    cmp::{Ord, Ordering},
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, Sub, SubAssign,
    },
    str::FromStr,
};

use gcd::{binary_u64, binary_u128};
use rug::{Assign, Integer, Rational};

use crate::{
    error::FirkinError, num::{
        float::FloatWrapper, int::IntegerWrapper, number::NumberWrapper, rat::RationalWrapper, sign::Sign::{self, Negative, Positive}, traits::{PythonMathFunctions, RichDisplay},
    },
};

/// Small rational number, to be used in unit powers and Number enum
#[derive(Clone, Copy, Hash)]
pub(crate) struct SmallRational {
    pub numerator: u64,
    pub denominator: u64,
    pub sign: Sign,
}

impl SmallRational {
    const MAX: SmallRational = SmallRational {
        numerator: u64::MAX,
        denominator: 1,
        sign: Positive,
    };

    const MIN: SmallRational = SmallRational {
        numerator: u64::MAX,
        denominator: 1,
        sign: Negative,
    };

    const ZERO: SmallRational = SmallRational {
        numerator: 0,
        denominator: 1,
        sign: Positive,
    };

    pub(crate) fn new(numerator: u64, denominator: u64, is_positive: bool) -> Self {
        if denominator == 0 {
            panic!("denominator cannot be zero");
        }
        if numerator == 0 {
            return SmallRational {
                numerator: 0,
                denominator: 1,
                sign: Positive,
            };
        }
        let result = SmallRational {
            numerator,
            denominator,
            sign: { if is_positive { Positive } else { Negative } },
        };
        result.reduce_inplace()
    }

    pub(crate) fn try_new(numerator: u128, denominator: u128, is_positive: bool) -> Option<Self> {
        if denominator == 0 {
            panic!("denominator cannot be zero");
        }
        if numerator == 0 {
            return Some(SmallRational {
                numerator: 0,
                denominator: 1,
                sign: Positive,
            });
        }
        let gcd = binary_u128(numerator, denominator);
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;

        if numerator > u64::MAX as u128 || denominator > u64::MAX as u128 {
            None // overflow
        } else {
            Some(
                SmallRational {
                    numerator: numerator as u64,
                    denominator: denominator as u64,
                    sign: { if is_positive { Positive } else { Negative } },
                }
                .reduce(),
            )
        }
    }

    pub(crate) fn is_positive(&self) -> bool {
        return self.sign == Positive;
    }

    pub(crate) fn is_negative(&self) -> bool {
        return self.sign == Negative;
    }

    pub(crate) const fn is_zero(&self) -> bool {
        return self.numerator == 0;
    }

    pub(crate) const fn is_integer(&self) -> bool {
        return self.denominator == 1;
    }

    pub(crate) fn to_integer(&self) -> IntegerWrapper {
        return IntegerWrapper::from(self.get_whole_number_part());
    }

    fn reduce_inplace(mut self) -> Self {
        let gcd = binary_u64(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
        self
    }

    pub(crate) fn reduce(&self) -> Self {
        let mut new = self.clone();
        new.reduce_inplace()
    }

    pub(crate) fn checked_pow(&self, other: i32) -> Option<Self> {
        if other >= 0 {
            SmallRational::try_new(
                (self.numerator as u128).checked_pow(other.unsigned_abs())?,
                (self.denominator as u128).checked_pow(other.unsigned_abs())?,
                self.is_positive() || other % 2 == 0,
            )
        } else {
            SmallRational::try_new(
                (self.denominator as u128).checked_pow(other.unsigned_abs())?,
                (self.numerator as u128).checked_pow(other.unsigned_abs())?,
                self.is_positive() || other % 2 == 0,
            )
        }
    }

    pub(crate) fn checked_add<RhsAdd>(&self, rhs: RhsAdd) -> Option<Self>
    where
        SmallRational: From<RhsAdd>,
    {
        let rhs: SmallRational = rhs.into();
        let snum = self.numerator as u128;
        let sden = self.denominator as u128;
        let rnum = rhs.numerator as u128;
        let rden = rhs.denominator as u128;
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                true,
            ),
            (Positive, Negative) => {
                let rhsabs = rhs.abs();
                if self == &rhsabs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self > &rhsabs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                }
            }
            (Negative, Positive) => {
                let selfabs = self.abs();
                if selfabs == rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if selfabs > rhs {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                } else {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                }
            }
            (Negative, Negative) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                false,
            ),
        }
    }

    pub(crate) fn checked_sub<RhsSub>(&self, rhs: RhsSub) -> Option<Self>
    where
        SmallRational: From<RhsSub>,
    {
        let rhs: SmallRational = rhs.into();
        let snum = self.numerator as u128;
        let sden = self.denominator as u128;
        let rnum = rhs.numerator as u128;
        let rden = rhs.denominator as u128;
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => {
                if self == &rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self > &rhs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                }
            }
            (Positive, Negative) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                true,
            ),
            (Negative, Positive) => SmallRational::try_new(
                snum.checked_mul(rden)?
                    .checked_add(rnum.checked_mul(sden)?)?,
                sden.checked_mul(rden)?,
                false,
            ),
            (Negative, Negative) => {
                if self == &rhs {
                    Some(SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    })
                } else if self < &rhs {
                    SmallRational::try_new(
                        snum.checked_mul(rden)?
                            .checked_sub(rnum.checked_mul(sden)?)?,
                        sden.checked_mul(rden)?,
                        false,
                    )
                } else {
                    SmallRational::try_new(
                        rnum.checked_mul(sden)?
                            .checked_sub(snum.checked_mul(rden)?)?,
                        sden.checked_mul(rden)?,
                        true,
                    )
                }
            }
        }
    }

    pub(crate) fn checked_mul<RhsMul>(&self, rhs: RhsMul) -> Option<Self>
    where
        SmallRational: From<RhsMul>,
    {
        let rhs: SmallRational = rhs.into();
        SmallRational::try_new(
            (self.numerator as u128).checked_mul(rhs.numerator as u128)?,
            (self.denominator as u128).checked_mul(rhs.denominator as u128)?,
            self.sign == rhs.sign,
        )
    }

    pub(crate) fn checked_div<RhsDiv>(&self, rhs: RhsDiv) -> Option<Self>
    where
        SmallRational: From<RhsDiv>,
    {
        let rhs: SmallRational = rhs.into();
        SmallRational::try_new(
            (self.numerator as u128).checked_mul(rhs.denominator as u128)?,
            (self.denominator as u128).checked_mul(rhs.numerator as u128)?,
            self.sign == rhs.sign,
        )
    }

    pub(crate) fn get_whole_number_part(&self) -> i128 {
        ((self.numerator - (self.numerator % self.denominator)) / self.denominator) as i128
            * i128::from(self.sign)
    }

    pub(crate) fn get_remainder(&self) -> Self {
        SmallRational::new(
            self.numerator % self.denominator,
            self.denominator,
            self.is_positive(),
        )
    }

    pub(crate) fn floor(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else if *self >= SmallRational::ZERO {
            self.get_whole_number_part()
        } else {
            self.get_whole_number_part() - 1
        }
    }

    pub(crate) fn ceil(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else if *self >= SmallRational::ZERO {
            self.get_whole_number_part() + 1
        } else {
            self.get_whole_number_part()
        }
    }

    pub(crate) fn round(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else {
            let rem = self.get_remainder();
            if (rem > SmallRational::ZERO && rem < SmallRational::new(1, 2, true)
                || rem < SmallRational::ZERO && rem > SmallRational::new(1, 2, false))
            {
                self.get_whole_number_part()
            } else {
                self.get_whole_number_part() + i128::from(self.sign)
            }
        }
    }

    pub(crate) fn abs(&self) -> Self {
        SmallRational {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: Positive,
        }
    }
}

impl From<i8> for SmallRational {
    fn from(value: i8) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i16> for SmallRational {
    fn from(value: i16) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i32> for SmallRational {
    fn from(value: i32) -> Self {
        SmallRational {
            numerator: value.unsigned_abs().into(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<i64> for SmallRational {
    fn from(value: i64) -> Self {
        SmallRational {
            numerator: value.unsigned_abs(),
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<isize> for SmallRational {
    fn from(value: isize) -> Self {
        SmallRational {
            numerator: value.unsigned_abs() as u64,
            denominator: 1,
            sign: {
                if value.is_positive() {
                    Positive
                } else {
                    Negative
                }
            },
        }
    }
}

impl From<u8> for SmallRational {
    fn from(value: u8) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u16> for SmallRational {
    fn from(value: u16) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u32> for SmallRational {
    fn from(value: u32) -> Self {
        SmallRational {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u64> for SmallRational {
    fn from(value: u64) -> Self {
        SmallRational {
            numerator: value,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<usize> for SmallRational {
    fn from(value: usize) -> Self {
        SmallRational {
            numerator: value as u64,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl<Num, Den> From<(Num, Den)> for SmallRational
where
    i64: From<Num> + From<Den>,
{
    fn from(value: (Num, Den)) -> Self {
        let num: i64 = value.0.into();
        let den: i64 = value.1.into();
        SmallRational::new(
            num.unsigned_abs(),
            den.unsigned_abs(),
            num.is_positive() == den.is_positive(),
        )
    }
}

impl From<SmallRational> for f64 {
    fn from(value: SmallRational) -> Self {
        f64::from(value.sign) * (value.numerator as f64) / (value.denominator as f64)
    }
}

impl From<&SmallRational> for f64 {
    fn from(value: &SmallRational) -> Self {
        f64::from(value.sign) * (value.numerator as f64) / (value.denominator as f64)
    }
}

impl From<SmallRational> for f32 {
    fn from(value: SmallRational) -> Self {
        f32::from(value.sign) * (value.numerator as f32) / (value.denominator as f32)
    }
}

impl From<&SmallRational> for f32 {
    fn from(value: &SmallRational) -> Self {
        f32::from(value.sign) * (value.numerator as f32) / (value.denominator as f32)
    }
}

impl From<SmallRational> for Rational {
    fn from(value: SmallRational) -> Rational {
        let mut result = Rational::new();
        result.assign((
            value.numerator as i128 * i128::from(value.sign),
            value.denominator,
        ));
        result
    }
}

impl From<&SmallRational> for Rational {
    fn from(value: &SmallRational) -> Rational {
        let mut result = Rational::new();
        result.assign((
            value.numerator as i128 * i128::from(value.sign),
            value.denominator,
        ));
        result
    }
}

impl From<&SmallRational> for SmallRational {
    fn from(value: &SmallRational) -> Self {
        *value
    }
}

impl TryFrom<i128> for SmallRational {
    type Error = &'static str;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match SmallRational::try_new(value.unsigned_abs(), 1, value.is_positive()) {
            Some(res) => Ok(res),
            None => Err("Overflow"),
        }
    }
}

impl PartialEq for SmallRational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator
            && other.denominator == self.denominator
            && self.sign == other.sign
    }
}

impl PartialEq<Rational> for SmallRational {
    fn eq(&self, other: &Rational) -> bool {
        if self.numerator == 0 && other.is_zero() {
            return true;
        }
        self.numerator == *other.numer()
            && self.denominator == *other.denom()
            && self.is_positive() == other.is_positive()
    }
}

impl PartialEq<SmallRational> for Rational {
    fn eq(&self, other: &SmallRational) -> bool {
        if self.is_zero() && other.numerator == 0 {
            return true;
        }
        *self.numer() == other.numerator
            && *self.denom() == other.denominator
            && self.is_positive() == other.is_positive()
    }
}

impl Eq for SmallRational {}

impl PartialOrd for SmallRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.numerator == 0 && other.numerator == 0 {
            return Some(Ordering::Equal); // -0 and 0 are equal
        }
        match (&self.sign, &other.sign) {
            (Positive, Positive) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Some(Ordering::Equal)
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Positive, Negative) => Some(Ordering::Greater),
            (Negative, Positive) => Some(Ordering::Less),
            (Negative, Negative) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Some(Ordering::Equal)
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl PartialOrd<Rational> for SmallRational {
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self.sign, other.is_positive()) {
            (Positive, true) => {
                if Integer::from(self.numerator * other.denom())
                    > Integer::from(other.numer() * self.denominator)
                {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Positive, false) => Some(Ordering::Greater),
            (Negative, true) => Some(Ordering::Less),
            (Negative, false) => {
                if Integer::from(self.numerator * other.denom())
                    > Integer::from(other.numer() * self.denominator)
                {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl PartialOrd<SmallRational> for Rational {
    fn partial_cmp(&self, other: &SmallRational) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self.is_positive(), other.sign) {
            (true, Positive) => {
                if Integer::from(self.numer() * other.denominator)
                    > Integer::from(other.numerator * self.denom())
                {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (true, Negative) => Some(Ordering::Greater),
            (false, Positive) => Some(Ordering::Less),
            (false, Negative) => {
                if Integer::from(self.numer() * other.denominator)
                    > Integer::from(other.numerator * self.denom())
                {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Ord for SmallRational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.numerator == 0 && other.numerator == 0 {
            return Ordering::Equal; // -0 and 0 are equal
        }
        match (&self.sign, &other.sign) {
            (Positive, Positive) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Ordering::Equal
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Positive, Negative) => Ordering::Greater,
            (Negative, Positive) => Ordering::Less,
            (Negative, Negative) => {
                if self.numerator * other.denominator == other.numerator * self.denominator {
                    Ordering::Equal
                } else if self.numerator * other.denominator > other.numerator * self.denominator {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }

    fn max(self, other: Self) -> Self {
        if other < self { self } else { other }
    }

    fn min(self, other: Self) -> Self {
        if other < self { other } else { self }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

macro_rules! arith_binary_rational {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<Rational> for SmallRational {
            type Output = Rational;

            fn $method(self, rhs: Rational) -> Self::Output {
                Rational::from(self).$method(rhs)
            }
        }

        impl $Imp<&Rational> for SmallRational {
            type Output = Rational;

            fn $method(self, rhs: &Rational) -> Self::Output {
                Rational::from(self).$method(rhs)
            }
        }

        impl $Imp<&Rational> for &SmallRational {
            type Output = Rational;

            fn $method(self, rhs: &Rational) -> Self::Output {
                Rational::from(self).$method(rhs)
            }
        }

        impl $Imp<SmallRational> for Rational {
            type Output = Rational;

            fn $method(self, rhs: SmallRational) -> Self::Output {
                self.$method(Rational::from(rhs))
            }
        }

        impl $Imp<&SmallRational> for Rational {
            type Output = Rational;

            fn $method(self, rhs: &SmallRational) -> Self::Output {
                self.$method(Rational::from(rhs))
            }
        }

        impl $Imp<SmallRational> for &Rational {
            type Output = Rational;

            fn $method(self, rhs: SmallRational) -> Self::Output {
                self.$method(Rational::from(rhs))
            }
        }

        impl $Imp<&SmallRational> for &Rational {
            type Output = Rational;

            fn $method(self, rhs: &SmallRational) -> Self::Output {
                self.$method(Rational::from(rhs))
            }
        }

        impl $ImpAssign<SmallRational> for Rational {
            fn $method_assign(&mut self, rhs: SmallRational) {
                self.$method_assign(Rational::from(rhs));
            }
        }

        impl $ImpAssign<&SmallRational> for Rational {
            fn $method_assign(&mut self, rhs: &SmallRational) {
                self.$method_assign(Rational::from(rhs));
            }
        }
    };
}

arith_binary_rational! {
    Add { add }
    AddAssign { add_assign }
}

arith_binary_rational! {
    Sub { sub }
    SubAssign { sub_assign }
}

arith_binary_rational! {
    Mul { mul }
    MulAssign { mul_assign }
}

arith_binary_rational! {
    Div { div }
    DivAssign { div_assign }
}

impl Neg for &SmallRational {
    type Output = SmallRational;

    fn neg(self) -> Self::Output {
        SmallRational {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: !self.sign,
        }
    }
}

impl Neg for SmallRational {
    type Output = SmallRational;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Not for &SmallRational {
    type Output = SmallRational;

    fn not(self) -> Self::Output {
        SmallRational {
            numerator: !self.numerator,
            denominator: !self.denominator,
            sign: !self.sign,
        }
    }
}

impl Not for SmallRational {
    type Output = SmallRational;

    fn not(self) -> Self::Output {
        -&self
    }
}

impl Rem<SmallRational> for SmallRational {
    type Output = SmallRational;

    fn rem(self, rhs: SmallRational) -> Self::Output {
        (&self).rem(&rhs)
    }
}

impl Rem<&SmallRational> for SmallRational {
    type Output = SmallRational;

    fn rem(self, rhs: &SmallRational) -> Self::Output {
        (&self).rem(rhs)
    }
}

impl Rem<SmallRational> for &SmallRational {
    type Output = SmallRational;

    fn rem(self, rhs: SmallRational) -> Self::Output {
        self.rem(&rhs)
    }
}

impl Rem<&SmallRational> for &SmallRational {
    type Output = SmallRational;

    fn rem(self, rhs: &SmallRational) -> Self::Output {
        todo!()
    }
}

impl Debug for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive => write!(f, "Positive"),
            Self::Negative => write!(f, "Negative"),
        }
    }
}

impl Debug for SmallRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmallRational")
            .field("numerator", &self.numerator)
            .field("denominator", &self.denominator)
            .field("sign", &self.sign)
            .finish()
    }
}

impl Display for SmallRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sign)?;
        if self.denominator != 1 {
            write!(f, "{}/{}", self.numerator, self.denominator)?;
        } else {
            write!(f, "{}", self.numerator)?;
        }
        Ok(())
    }
}

impl PythonMathFunctions for SmallRational {
    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(IntegerWrapper::ZERO);
        } else if self.is_integer() {
            return self.to_integer().py_floordiv(other);
        }
        match other {
            NumberWrapper::Int(int) => {
                if int.is_positive() {
                    match int {
                        IntegerWrapper::SmallInt(i) => match self.checked_div(i.unsigned_abs()) {
                            Some(res) => {
                                let mut result: i128 = (res.numerator / res.denominator) as i128;
                                if self.is_negative() {
                                    result = -(result as i128) - 1;
                                }
                                Ok(IntegerWrapper::from(result))
                            }
                            None => Rational::from(self).py_floordiv(other),
                        },
                        IntegerWrapper::Int(integer) => Rational::from(self).py_floordiv(other),
                    }
                } else {
                    match int {
                        IntegerWrapper::SmallInt(i) => match self.checked_div(i.unsigned_abs()) {
                            Some(res) => {
                                let mut result: i128 = (res.numerator / res.denominator) as i128;
                                if self.is_positive() {
                                    result = -(result as i128) - 1;
                                }
                                Ok(IntegerWrapper::from(result))
                            }
                            None => Rational::from(self).py_floordiv(other),
                        },
                        IntegerWrapper::Int(integer) => Rational::from(self).py_floordiv(other),
                    }
                }
            }
            NumberWrapper::Rat(rat) => match rat {
                RationalWrapper::SmallRat(r) => match self.checked_div(r) {
                    Some(res) => Ok(IntegerWrapper::from(res.floor())),
                    None => Rational::from(self).py_floordiv(other),
                },
                RationalWrapper::Rat(rational) => Rational::from(self).py_floordiv(other),
            },
            NumberWrapper::Float(float) => FloatWrapper::from(*self).py_floordiv(other),
        }
    }

    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        } else if self.is_integer() {
            return self.to_integer().py_pow(other, modulus);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => {
                    if i.unsigned_abs() > i32::MAX as u64 {
                        Rational::from(self).py_pow(other, modulus)
                    } else {
                        match self.checked_pow(*i as i32) {
                            Some(res) => Ok(NumberWrapper::from(res)),
                            None => Rational::from(self).py_pow(other, modulus),
                        }
                    }
                }
                IntegerWrapper::Int(integer) => FloatWrapper::from(*self).py_pow(other, modulus),
            },
            _ => FloatWrapper::from(*self).py_pow(other, modulus),
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
                IntegerWrapper::SmallInt(i) => match self.checked_div(*i) {
                    Some(res) => Ok(NumberWrapper::from(res)),
                    None => Rational::from(self).py_truediv(other),
                },
                IntegerWrapper::Int(integer) => Rational::from(self).py_truediv(other),
            },
            NumberWrapper::Rat(rational_wrapper) => match rational_wrapper {
                RationalWrapper::SmallRat(r) => match self.checked_div(r) {
                    Some(res) => Ok(NumberWrapper::from(res)),
                    None => Rational::from(self).py_truediv(other),
                },
                RationalWrapper::Rat(rational) => Ok(NumberWrapper::from(self / rational)),
            },

            NumberWrapper::Float(float_wrapper) => FloatWrapper::from(*self).py_truediv(other),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(self.abs().into())
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(self.to_integer())
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(self.into())
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        Ok(self.round().into())
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(self.floor().into())
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(self.ceil().into())
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(self.get_whole_number_part().into())
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => {
                let (frac, trunc) = (self.get_remainder(), self.get_whole_number_part());
                let rem_trunc = match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => trunc % *i as i128,
                    IntegerWrapper::Int(integer) => {
                        Integer::from(trunc % integer).to_i128_wrapping()
                    }
                };
                match SmallRational::try_new(rem_trunc.unsigned_abs(), 1, rem_trunc.is_positive()) {
                    Some(rem_trunc) => match frac.checked_add(rem_trunc) {
                        Some(res) => Ok(NumberWrapper::from(res)),
                        None => unreachable!(),
                    },
                    None => unreachable!(),
                }
            }
            NumberWrapper::Rat(rational_wrapper) => match rational_wrapper {
                RationalWrapper::SmallRat(r) => {
                    let adivb = match self.checked_div(r) {
                        Some(adivb) => adivb,
                        None => return Rational::from(self).py_mod(other),
                    };
                    let adivb_floor = match SmallRational::try_from(adivb.floor()) {
                        Ok(adivb_floor) => adivb_floor,
                        Err(_) => return Rational::from(self).py_mod(other),
                    };
                    let bfloor = match r.checked_mul(adivb_floor) {
                        Some(bfloor) => bfloor,
                        None => return Rational::from(self).py_mod(other),
                    };
                    match self.checked_sub(bfloor) {
                        Some(res) => Ok(NumberWrapper::from(res)),
                        None => Rational::from(self).py_mod(other),
                    }
                }
                RationalWrapper::Rat(rational) => Rational::from(self).py_mod(other),
            },
            NumberWrapper::Float(float_wrapper) => FloatWrapper::from(*self).py_mod(other),
        }
    }
}

impl RichDisplay for SmallRational {
    fn display_default(&self) -> String {
        // this will display as float, switching to e notation when necessary
        format!("{:?}", f64::from(self))
    }

    fn display_debug(&self) -> String {
        format!("<SmallRational {}>", self.to_string())
    }

    fn format(&self, format_specifier:String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::num::traits::RichDisplay;

use super::SmallRational;

    #[test]
    fn new() {
        let a = SmallRational::new(25, 5, true);
        let b = SmallRational::new(5, 1, true);
        assert_eq!(a, b);
        let c = SmallRational::new(875, 175, true);
        assert_eq!(a, c);
    }

    #[test]
    fn from_int() {
        let smallrat = SmallRational::new(1, 1, true);
        assert_eq!(smallrat, SmallRational::from(1_u8));
        assert_eq!(smallrat, SmallRational::from(1_u16));
        assert_eq!(smallrat, SmallRational::from(1_u32));
        assert_eq!(smallrat, SmallRational::from(1_u64));
        assert_eq!(smallrat, SmallRational::from(1_usize));
        assert_eq!(smallrat, SmallRational::from(1_i8));
        assert_eq!(smallrat, SmallRational::from(1_i16));
        assert_eq!(smallrat, SmallRational::from(1_i32));
        assert_eq!(smallrat, SmallRational::from(1_i64));
        assert_eq!(smallrat, SmallRational::from(1_isize));
        assert_ne!(smallrat, SmallRational::from(-5_i32));
        let smallrat = SmallRational::new(5, 1, false);
        assert_eq!(smallrat, SmallRational::from(-5_i32));
    }

    #[test]
    fn add() {
        let smallpos = SmallRational::new(17, 5, true);
        let bigpos = SmallRational::new(27, 5, true);
        let smallneg = SmallRational::new(17, 5, false);
        let bigneg = SmallRational::new(27, 5, false);
        assert_eq!(
            smallpos.checked_add(&smallneg).unwrap(),
            SmallRational::ZERO
        );
        assert_eq!(
            smallpos.checked_add(&bigpos).unwrap(),
            SmallRational::new(44, 5, true)
        );
        assert_eq!(
            smallpos.checked_add(&bigneg).unwrap(),
            SmallRational::new(10, 5, false)
        );
        assert_eq!(
            bigpos.checked_add(&smallneg).unwrap(),
            SmallRational::new(10, 5, true)
        );
        assert_eq!(
            smallneg.checked_add(&bigneg).unwrap(),
            SmallRational::new(44, 5, false)
        );
    }

    #[test]
    fn sub() {
        let smallpos = SmallRational::new(17, 5, true);
        let bigpos = SmallRational::new(27, 5, true);
        let smallneg = SmallRational::new(17, 5, false);
        let bigneg = SmallRational::new(27, 5, false);
        assert_eq!(
            smallpos.checked_sub(&smallneg).unwrap(),
            smallpos.checked_add(&smallpos).unwrap()
        );
        assert_eq!(
            smallpos.checked_sub(&bigpos).unwrap(),
            SmallRational::new(10, 5, false)
        );
        assert_eq!(
            smallpos.checked_sub(&bigneg).unwrap(),
            SmallRational::new(44, 5, true)
        );
        assert_eq!(
            bigpos.checked_sub(&smallneg).unwrap(),
            SmallRational::new(44, 5, true)
        );
        assert_eq!(
            smallneg.checked_sub(&bigneg).unwrap(),
            SmallRational::new(10, 5, true)
        );
    }

    #[test]
    fn mul() {
        let a = SmallRational::new(25, 4, true);
        let b = SmallRational::new(3, 5, true);
        assert_eq!(a.checked_mul(&b).unwrap(), SmallRational::new(15, 4, true));
        let c = SmallRational::new(1, 2, false);
        assert_eq!(a.checked_mul(&c).unwrap(), SmallRational::new(25, 8, false));
    }

    #[test]
    fn div() {
        let a = SmallRational::new(25, 4, true);
        let b = SmallRational::new(3, 5, true);
        assert_eq!(
            a.checked_div(&b).unwrap(),
            SmallRational::new(125, 12, true)
        );
        let c = SmallRational::new(1, 2, false);
        assert_eq!(a.checked_div(&c).unwrap(), SmallRational::new(25, 2, false));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SmallRational::new(48, 37, false)), "-48/37");
        assert_eq!(format!("{}", SmallRational::new(48, 3, true)), "16");
    }

    #[test]
    fn pow() {
        assert_eq!(
            SmallRational::new(3, 2, false).checked_pow(2).unwrap(),
            SmallRational::new(9, 4, true)
        );
        assert_eq!(
            SmallRational::new(3, 2, false).checked_pow(3).unwrap(),
            SmallRational::new(27, 8, false)
        );
    }

    #[test]
    fn floor_ceil_round() {
        assert_eq!(SmallRational::new(4, 5, true).floor(), 0);
        assert_eq!(SmallRational::new(4, 5, true).ceil(), 1);
        assert_eq!(SmallRational::new(4, 5, false).round(), -1);
        assert_eq!(
            SmallRational::new(u64::MAX, 1, true).round(),
            u64::MAX as i128
        );
    }

    #[test]
    fn rich_display() {
        assert_eq!(SmallRational::new(4, 5, true).display_default(), "0.8");
        assert_eq!(SmallRational::new(4, 5, true).display_debug(), "<SmallRational 4/5>");
        assert_eq!(SmallRational::new(45822172, 7831, false).display_default(), "-5851.381943557655");
        
    }
}
