use std::{
    cmp::{Ord, Ordering},
    fmt::{Debug, Display},
    ops,
};

#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

use Sign::{Negative, Positive};

/// Small rational number, to be used in unit powers and Number enum
#[derive(Clone, Copy)]
pub(crate) struct SmallRat {
    numerator: u64,
    denominator: u64,
    sign: Sign,
}

impl SmallRat {
    pub(crate) fn new(numerator: u64, denominator: u64, is_positive: bool) -> Self {
        let result = SmallRat {
            numerator,
            denominator,
            sign: { if is_positive { Positive } else { Negative } },
        };
        result.reduce_inplace()
    }

    pub(crate) fn try_new(numerator: u128, denominator: u128, is_positive: bool) -> Option<Self> {
        let mut numerator = numerator;
        let mut denominator = denominator;
        let min = (numerator.min(denominator) as f64).sqrt().floor() as u128;
        let mut i = 2_u128;
        while i <= min {
            while numerator % i == 0 && denominator % i == 0 {
                numerator /= i;
                denominator /= i;
            }
            i += 1;
        }

        if numerator > u64::MAX.into() || denominator > u64::MAX.into() {
            None // overflow
        } else {
            Some(SmallRat {
                numerator: numerator as u64,
                denominator: denominator as u64,
                sign: { if is_positive { Positive } else { Negative } },
            })
        }
    }

    fn is_positive(&self) -> bool {
        return self.sign == Positive;
    }

    fn is_negative(&self) -> bool {
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
        let min = (self.numerator.min(self.denominator) as f64).sqrt().floor() as u64 + 1;
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

    fn reduce(&self) -> Self {
        let mut new = self.clone();
        let min = (new.numerator.min(new.denominator) as f64).sqrt().floor() as u64;
        let mut i = 2_u64;
        while i <= min {
            while new.numerator % i == 0 && new.denominator % i == 0 {
                new.numerator /= i;
                new.denominator /= i;
            }
            i += 1;
        }
        new
    }

    pub(crate) fn pow(&self, other: i32) -> Option<Self> {
        if other >= 0 {
            SmallRat::try_new(
                (self.numerator as u128).pow(other.unsigned_abs()),
                (self.denominator as u128).pow(other.unsigned_abs()),
                self.is_positive() || other % 2 == 0,
            )
        } else {
            SmallRat::try_new(
                (self.denominator as u128).pow(other.unsigned_abs()),
                (self.numerator as u128).pow(other.unsigned_abs()),
                self.is_positive() || other % 2 == 0,
            )
        }
    }
}

impl From<i8> for SmallRat {
    fn from(value: i8) -> Self {
        SmallRat {
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

impl From<i16> for SmallRat {
    fn from(value: i16) -> Self {
        SmallRat {
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

impl From<i32> for SmallRat {
    fn from(value: i32) -> Self {
        SmallRat {
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

impl From<i64> for SmallRat {
    fn from(value: i64) -> Self {
        SmallRat {
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

impl From<isize> for SmallRat {
    fn from(value: isize) -> Self {
        SmallRat {
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

impl From<u8> for SmallRat {
    fn from(value: u8) -> Self {
        SmallRat {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u16> for SmallRat {
    fn from(value: u16) -> Self {
        SmallRat {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u32> for SmallRat {
    fn from(value: u32) -> Self {
        SmallRat {
            numerator: value.into(),
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<u64> for SmallRat {
    fn from(value: u64) -> Self {
        SmallRat {
            numerator: value,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<usize> for SmallRat {
    fn from(value: usize) -> Self {
        SmallRat {
            numerator: value as u64,
            denominator: 1,
            sign: Positive,
        }
    }
}

impl From<f32> for SmallRat {
    fn from(value: f32) -> Self {
        let mut value1 = value.abs();
        let mut denom = 1_u64;
        while value1 % 1.0 != 0.0 {
            value1 *= 2.0;
            denom *= 2;
        }
        SmallRat::new(value1 as u64, denom, value > 0.0)
    }
}

impl From<f64> for SmallRat {
    fn from(value: f64) -> Self {
        let mut value1 = value.abs();
        let mut denom = 1_u64;
        while value1 % 1.0 != 0.0 {
            value1 *= 2.0;
            denom *= 2;
        }
        SmallRat::new(value1 as u64, denom, value > 0.0)
    }
}

impl From<SmallRat> for f64 {
    fn from(value: SmallRat) -> Self {
        let sign = if value.is_positive() { 1.0 } else { -1.0 };
        sign * (value.numerator as f64) / (value.denominator as f64)
    }
}

impl From<SmallRat> for f32 {
    fn from(value: SmallRat) -> Self {
        let sign = if value.is_positive() { 1.0 } else { -1.0 };
        sign * (value.numerator as f32) / (value.denominator as f32)
    }
}

impl PartialEq for SmallRat {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == other.numerator * self.denominator
            && (self.sign == other.sign || (self.numerator == 0 && other.numerator == 0))
    }
}

impl Eq for SmallRat {}

impl PartialOrd for SmallRat {
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

impl Ord for SmallRat {
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

impl ops::Add<SmallRat> for SmallRat {
    type Output = Self;

    fn add(self, rhs: SmallRat) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => SmallRat::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                true,
            ),
            (Positive, Negative) => self - -rhs,
            (Negative, Positive) => rhs - -self,
            (Negative, Negative) => SmallRat::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                false,
            ),
        }
    }
}

impl ops::Sub<SmallRat> for SmallRat {
    type Output = Self;

    fn sub(self, rhs: SmallRat) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Positive, Positive) => {
                if self == rhs {
                    SmallRat {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    }
                } else if self > rhs {
                    SmallRat::new(
                        self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                        self.denominator * rhs.denominator,
                        true,
                    )
                } else {
                    SmallRat::new(
                        rhs.numerator * self.denominator - self.numerator * rhs.denominator,
                        self.denominator * rhs.denominator,
                        false,
                    )
                }
            }
            (Positive, Negative) => SmallRat::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                true,
            ),
            (Negative, Positive) => SmallRat::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
                false,
            ),
            (Negative, Negative) => {
                if self == rhs {
                    SmallRat {
                        numerator: 0,
                        denominator: 1,
                        sign: Positive,
                    }
                } else if self < rhs {
                    SmallRat::new(
                        self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                        self.denominator * rhs.denominator,
                        false,
                    )
                } else {
                    SmallRat::new(
                        rhs.numerator * self.denominator - self.numerator * rhs.denominator,
                        self.denominator * rhs.denominator,
                        true,
                    )
                }
            }
        }
    }
}

impl ops::Mul<SmallRat> for SmallRat {
    type Output = Self;

    fn mul(self, rhs: SmallRat) -> Self::Output {
        SmallRat::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
            self.sign == rhs.sign,
        )
    }
}

impl ops::Div<SmallRat> for SmallRat {
    type Output = Self;

    fn div(self, rhs: SmallRat) -> Self::Output {
        SmallRat::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
            self.sign == rhs.sign,
        )
    }
}

impl ops::Neg for SmallRat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        SmallRat {
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

impl Debug for SmallRat {
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

impl Display for SmallRat {
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
    use super::SmallRat;

    #[test]
    fn new() {
        let a = SmallRat::new(25, 5, true);
        let b = SmallRat::new(5, 1, true);
        assert_eq!(a, b);
        let c = SmallRat::new(875, 175, true);
        assert_eq!(a, c);
    }

    #[test]
    fn from_int() {
        let smallrat = SmallRat::new(1, 1, true);
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
        let smallrat = SmallRat::new(5, 1, false);
        assert_eq!(smallrat, (-5_i32).into());
    }

    #[test]
    fn from_float() {
        let smallrat = SmallRat::new(1, 1, true);
        assert_eq!(smallrat, 1.0_f32.into());
        assert_eq!(smallrat, 1.0_f64.into());
        assert_ne!(smallrat, 2.0_f64.into());
        let smallrat = SmallRat::new(17, 4, true);
        assert_eq!(smallrat, 4.25_f64.into());
        let smallrat = SmallRat::new(17, 4, false);
        assert_eq!(smallrat, (-4.25_f64).into());
    }

    #[test]
    fn add() {
        let smallpos = SmallRat::new(17, 5, true);
        let bigpos = SmallRat::new(27, 5, true);
        let smallneg = SmallRat::new(17, 5, false);
        let bigneg = SmallRat::new(27, 5, false);
        assert_eq!(smallpos + smallneg, 0.into());
        assert_eq!(smallpos + bigpos, SmallRat::new(44, 5, true));
        assert_eq!(smallpos + bigneg, SmallRat::new(10, 5, false));
        assert_eq!(bigpos + smallneg, SmallRat::new(10, 5, true));
        assert_eq!(smallneg + bigneg, SmallRat::new(44, 5, false));
    }

    #[test]
    fn sub() {
        let smallpos = SmallRat::new(17, 5, true);
        let bigpos = SmallRat::new(27, 5, true);
        let smallneg = SmallRat::new(17, 5, false);
        let bigneg = SmallRat::new(27, 5, false);
        assert_eq!(smallpos - smallneg, smallpos + smallpos);
        assert_eq!(smallpos - bigpos, SmallRat::new(10, 5, false));
        assert_eq!(smallpos - bigneg, SmallRat::new(44, 5, true));
        assert_eq!(bigpos - smallneg, SmallRat::new(44, 5, true));
        assert_eq!(smallneg - bigneg, SmallRat::new(10, 5, true));
    }

    #[test]
    fn mul() {
        let a = SmallRat::new(25, 4, true);
        let b = SmallRat::new(3, 5, true);
        assert_eq!(a * b, SmallRat::new(15, 4, true));
        let c = SmallRat::new(1, 2, false);
        assert_eq!(a * c, SmallRat::new(25, 8, false));
    }

    #[test]
    fn div() {
        let a = SmallRat::new(25, 4, true);
        let b = SmallRat::new(3, 5, true);
        assert_eq!(a / b, SmallRat::new(125, 12, true));
        let c = SmallRat::new(1, 2, false);
        assert_eq!(a / c, SmallRat::new(25, 2, false));
    }

    #[test]
    fn to_float() {
        assert_eq!(3.4_f64, f64::from(SmallRat::from(3.4)))
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SmallRat::new(48, 37, false)), "-48/37");
        assert_eq!(format!("{}", SmallRat::new(48, 3, true)), "16");
    }

    #[test]
    fn pow() {
        assert_eq!(
            SmallRat::new(3, 2, false).pow(2).unwrap(),
            SmallRat::new(9, 4, true)
        );
        assert_eq!(
            SmallRat::new(3, 2, false).pow(3).unwrap(),
            SmallRat::new(27, 8, false)
        );
    }
}
