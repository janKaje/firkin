#![allow(unused)]

use std::{
    cmp::{Ord, Ordering},
    fmt::{Debug, Display},
    ops,
};

use super::sign::Sign::{self, Negative, Positive};

/// Small rational number, to be used in unit powers and Number enum
#[derive(Clone, Copy)]
pub(crate) struct SmallRational {
    numerator: u64,
    denominator: u64,
    sign: Sign,
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
        let mut numerator = numerator;
        let mut denominator = denominator;
        if numerator % denominator == 0 {
            numerator /= denominator;
            denominator = 1;
        } else if denominator % numerator == 0 {
            denominator /= numerator;
            numerator = 0;
        } else {
            let min = (numerator.min(denominator) as f64).sqrt().floor() as u128;
            let mut i = 2_u128;
            while i <= min {
                while numerator % i == 0 && denominator % i == 0 {
                    numerator /= i;
                    denominator /= i;
                }
                i += 1;
            }
        }

        if numerator > u64::MAX as u128 || denominator > u64::MAX as u128 {
            None // overflow
        } else {
            Some(SmallRational {
                numerator: numerator as u64,
                denominator: denominator as u64,
                sign: { if is_positive { Positive } else { Negative } },
            })
        }
    }

    pub(crate) fn is_positive(&self) -> bool {
        return self.sign == Positive;
    }

    pub(crate) fn is_negative(&self) -> bool {
        return self.sign == Negative;
    }

    fn reduce_inplace(mut self) -> Self {
        if self.numerator % self.denominator == 0 {
            self.numerator /= self.denominator;
            self.denominator = 1;
            return self;
        } else if self.denominator % self.numerator == 0 {
            self.denominator /= self.numerator;
            self.numerator = 1;
            return self;
        }
        let min = (self.numerator.min(self.denominator) as f64).sqrt().floor() as u64;
        let mut i = 2_u64;
        while i <= min {
            while self.numerator % i == 0 && self.denominator % i == 0 {
                self.numerator /= i;
                self.denominator /= i;
                dbg!(&i);
            }
            i += 1;
        }
        self
    }

    pub(crate) fn reduce(&self) -> Self {
        let mut new = self.clone();
        new.reduce_inplace()
    }

    pub(crate) fn pow(&self, other: i32) -> Option<Self> {
        if other >= 0 {
            SmallRational::try_new(
                (self.numerator as u128).pow(other.unsigned_abs()),
                (self.denominator as u128).pow(other.unsigned_abs()),
                self.is_positive() || other % 2 == 0,
            )
        } else {
            SmallRational::try_new(
                (self.denominator as u128).pow(other.unsigned_abs()),
                (self.numerator as u128).pow(other.unsigned_abs()),
                self.is_positive() || other % 2 == 0,
            )
        }
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
        } else if *self >= 0.into() {
            self.get_whole_number_part()
        } else {
            self.get_whole_number_part() - 1
        }
    }

    pub(crate) fn ceil(&self) -> i128 {
        if self.numerator % self.denominator == 0 {
            self.get_whole_number_part()
        } else if *self >= 0.into() {
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
            if (rem > 0.into() && rem < SmallRational::new(1, 2, true)
                || rem < 0.into() && rem > SmallRational::new(1, 2, false))
            {
                self.get_whole_number_part()
            } else {
                self.get_whole_number_part() + i128::from(self.sign)
            }
        }
    }

    pub(crate) fn abs(&self) -> Self {
        SmallRational { numerator: self.numerator, denominator: self.denominator, sign: Positive }
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

impl From<f32> for SmallRational {
    fn from(value: f32) -> Self {
        let mut value1 = value.abs();
        let mut denom = 1_u64;
        while value1 % 1.0 != 0.0 {
            value1 *= 2.0;
            denom *= 2;
        }
        SmallRational::new(value1 as u64, denom, value > 0.0)
    }
}

impl From<f64> for SmallRational {
    fn from(value: f64) -> Self {
        let mut value1 = value.abs();
        let mut denom = 1_u64;
        while value1 % 1.0 != 0.0 {
            value1 *= 2.0;
            denom *= 2;
        }
        SmallRational::new(value1 as u64, denom, value > 0.0)
    }
}

impl From<SmallRational> for f64 {
    fn from(value: SmallRational) -> Self {
        f64::from(value.sign) * (value.numerator as f64) / (value.denominator as f64)
    }
}

impl From<SmallRational> for f32 {
    fn from(value: SmallRational) -> Self {
        f32::from(value.sign) * (value.numerator as f32) / (value.denominator as f32)
    }
}

impl PartialEq for SmallRational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
            && (self.sign == other.sign || (self.numerator == 0 && other.numerator == 0))
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

impl ops::Add<SmallRational> for SmallRational {
    type Output = Self;

    fn add(self, rhs: SmallRational) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => SmallRational::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                true,
            ),
            (Positive, Negative) => self - -rhs,
            (Negative, Positive) => rhs - -self,
            (Negative, Negative) => SmallRational::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                false,
            ),
        }
    }
}

impl ops::Sub<SmallRational> for SmallRational {
    type Output = Self;

    fn sub(self, rhs: SmallRational) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => {
                if self == rhs {
                    SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    }
                } else if self > rhs {
                    SmallRational::new(
                        self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                        self.denominator * rhs.denominator,
                        true,
                    )
                } else {
                    SmallRational::new(
                        rhs.numerator * self.denominator - self.numerator * rhs.denominator,
                        self.denominator * rhs.denominator,
                        false,
                    )
                }
            }
            (Positive, Negative) => SmallRational::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                true,
            ),
            (Negative, Positive) => SmallRational::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                false,
            ),
            (Negative, Negative) => {
                if self == rhs {
                    SmallRational {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    }
                } else if self < rhs {
                    SmallRational::new(
                        self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                        self.denominator * rhs.denominator,
                        false,
                    )
                } else {
                    SmallRational::new(
                        rhs.numerator * self.denominator - self.numerator * rhs.denominator,
                        self.denominator * rhs.denominator,
                        true,
                    )
                }
            }
        }
    }
}

impl ops::Mul<SmallRational> for SmallRational {
    type Output = Self;

    fn mul(self, rhs: SmallRational) -> Self::Output {
        SmallRational::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
            self.sign == rhs.sign,
        )
    }
}

impl ops::Div<SmallRational> for SmallRational {
    type Output = Self;

    fn div(self, rhs: SmallRational) -> Self::Output {
        SmallRational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
            self.sign == rhs.sign,
        )
    }
}

impl ops::Neg for SmallRational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        SmallRational {
            numerator: self.numerator,
            denominator: self.denominator,
            sign: {
                if self.sign == Positive {
                    Negative
                } else {
                    Positive
                }
            },
        }
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
        f.debug_struct("SmallRat")
            .field("numerator", &self.numerator)
            .field("denominator", &self.denominator)
            .field("sign", &self.sign)
            .finish()
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Negative {
            write!(f, "-")?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
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
        assert_eq!(smallrat, 1_u8.into());
        assert_eq!(smallrat, 1_u16.into());
        assert_eq!(smallrat, 1_u32.into());
        assert_eq!(smallrat, 1_u64.into());
        assert_eq!(smallrat, 1_usize.into());
        assert_eq!(smallrat, 1_i8.into());
        assert_eq!(smallrat, 1_i16.into());
        assert_eq!(smallrat, 1_i32.into());
        assert_eq!(smallrat, 1_i64.into());
        assert_eq!(smallrat, 1_isize.into());
        assert_ne!(smallrat, (-5_i32).into());
        let smallrat = SmallRational::new(5, 1, false);
        assert_eq!(smallrat, (-5_i32).into());
    }

    #[test]
    fn from_float() {
        let smallrat = SmallRational::new(1, 1, true);
        assert_eq!(smallrat, 1.0_f32.into());
        assert_eq!(smallrat, 1.0_f64.into());
        assert_ne!(smallrat, 2.0_f64.into());
        let smallrat = SmallRational::new(17, 4, true);
        assert_eq!(smallrat, 4.25_f64.into());
        let smallrat = SmallRational::new(17, 4, false);
        assert_eq!(smallrat, (-4.25_f64).into());
    }

    #[test]
    fn add() {
        let smallpos = SmallRational::new(17, 5, true);
        let bigpos = SmallRational::new(27, 5, true);
        let smallneg = SmallRational::new(17, 5, false);
        let bigneg = SmallRational::new(27, 5, false);
        assert_eq!(smallpos + smallneg, 0.into());
        assert_eq!(smallpos + bigpos, SmallRational::new(44, 5, true));
        assert_eq!(smallpos + bigneg, SmallRational::new(10, 5, false));
        assert_eq!(bigpos + smallneg, SmallRational::new(10, 5, true));
        assert_eq!(smallneg + bigneg, SmallRational::new(44, 5, false));
    }

    #[test]
    fn sub() {
        let smallpos = SmallRational::new(17, 5, true);
        let bigpos = SmallRational::new(27, 5, true);
        let smallneg = SmallRational::new(17, 5, false);
        let bigneg = SmallRational::new(27, 5, false);
        assert_eq!(smallpos - smallneg, smallpos + smallpos);
        assert_eq!(smallpos - bigpos, SmallRational::new(10, 5, false));
        assert_eq!(smallpos - bigneg, SmallRational::new(44, 5, true));
        assert_eq!(bigpos - smallneg, SmallRational::new(44, 5, true));
        assert_eq!(smallneg - bigneg, SmallRational::new(10, 5, true));
    }

    #[test]
    fn mul() {
        let a = SmallRational::new(25, 4, true);
        let b = SmallRational::new(3, 5, true);
        assert_eq!(a * b, SmallRational::new(15, 4, true));
        let c = SmallRational::new(1, 2, false);
        assert_eq!(a * c, SmallRational::new(25, 8, false));
    }

    #[test]
    fn div() {
        let a = SmallRational::new(25, 4, true);
        let b = SmallRational::new(3, 5, true);
        assert_eq!(a / b, SmallRational::new(125, 12, true));
        let c = SmallRational::new(1, 2, false);
        assert_eq!(a / c, SmallRational::new(25, 2, false));
    }

    #[test]
    fn to_float() {
        assert_eq!(3.4_f64, f64::from(SmallRational::from(3.4)))
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SmallRational::new(48, 37, false)), "-48/37");
        assert_eq!(format!("{}", SmallRational::new(48, 3, true)), "16");
    }

    #[test]
    fn pow() {
        assert_eq!(
            SmallRational::new(3, 2, false).pow(2).unwrap(),
            SmallRational::new(9, 4, true)
        );
        assert_eq!(
            SmallRational::new(3, 2, false).pow(3).unwrap(),
            SmallRational::new(27, 8, false)
        );
    }

    #[test]
    fn floor_ceil_round() {
        assert_eq!(SmallRational::new(4, 5, true).floor(), 0);
        assert_eq!(SmallRational::new(4, 5, true).ceil(), 1);
        assert_eq!(SmallRational::new(4, 5, false).round(), -1);
        assert_eq!(SmallRational::new(u64::MAX, 1, true).round(), u64::MAX as i128);
    }
}
