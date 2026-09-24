use std::ops::{self, AddAssign, MulAssign};
use rug::{Float, Integer, Rational, ops::Pow};
use rug::{Assign, float};
use rug::float::{FreeCache, Round};
use rug::ops::{AddAssignRound, AssignRound, MulAssignRound};

use super::smallrational::SmallRational;

const DEFAULT_PREC: u32 = 53;

/// Single number, whether it's integer, rational, or irrational
#[derive(Clone)]
pub(crate) enum Number {
    SmallInt(i64),
    Int(Integer),
    SmallRat(SmallRational),
    Rat(Rational),
    Flt(Float),
}

use Number::{SmallInt, Int, SmallRat, Rat, Flt};

impl Number {

    pub(crate) fn add(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(integer) => match other {
                SmallInt(_) => todo!(),
                Int(_) => todo!(),
                SmallRat(small_rational) => todo!(),
                Rat(_) => todo!(),
                Flt(_) => todo!(),
            },
            Int(integer) => match other {
                Int(integer1) => Int((integer + integer1).into()),
                Rat(rational) => Rat((rational + integer).into()),
                _ => other.add(self, Some(prec))
            },
            SmallRat(small_rat) => todo!(),
            Rat(rational) => match other {
                Rat(rational1) => Rat((rational + rational1).into()),
                _ => other.add(self, Some(prec))
            },
            Flt(float) => Flt((float + other.to_float(prec)).into()),
        }
    }

    pub(crate) fn sub(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => match other {
                SmallInt(int) => todo!(),
                Int(integer1) => Int((integer-integer1).into()),
                SmallRat(small_rat) => todo!(),
                Rat(rational) => Rat((integer-rational).into()),
                Flt(float) => Flt((integer - float).into()),
            },
            SmallRat(small_rat) => todo!(),
            Rat(rational) => match other {
                SmallInt(int) => todo!(),
                Int(integer) => Rat((rational - integer).into()),
                SmallRat(small_rat) => todo!(),
                Rat(rational1) => Rat((rational - rational1).into()),
                Flt(float) => Flt((rational - float).into()),
            },
            Flt(float) => Flt((float - other.to_float(prec)).into()),
        }
    }

    pub(crate) fn mul(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => match other {
                Int(integer1) => Int((integer * integer1).into()),
                Rat(rational) => Rat((rational * integer).into()),
                _ => other.mul(self, Some(prec))
            },
            SmallRat(small_rat) => todo!(),
            Rat(rational) => match other {
                Rat(rational1) => Rat((rational * rational1).into()),
                _ => other.mul(self, Some(prec))
            },
            Flt(float) => Flt((float * other.to_float(prec)).into()),
        }
    }

    pub(crate) fn div(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => match other {
                SmallInt(int) => todo!(),
                Int(integer1) => Int((integer/integer1).into()),
                SmallRat(small_rat) => todo!(),
                Rat(rational) => Rat((integer/rational).into()),
                Flt(float) => Flt((integer/float).into()),
            },
            SmallRat(small_rat) => todo!(),
            Rat(rational) => match other {
                SmallInt(int) => todo!(),
                Int(integer) => Rat((rational/integer).into()),
                SmallRat(small_rat) => todo!(),
                Rat(rational1) => Rat((rational / rational1).into()),
                Flt(float) => Flt((rational / float).into()),
            },
            Flt(float) => Flt((float / other.to_float(prec)).into()),
        }
    }

    pub(crate) fn pow(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match other {
            SmallInt(i) => todo!(),
            Int(integer) => match self {
                SmallInt(int) => todo!(),
                Int(integer1) => Int((integer1.pow(integer)).into()),
                SmallRat(small_rat) => todo!(),
                Rat(rational) => Rat((rational.pow(integer)).into()),
                Flt(float) => Flt((float.pow(integer)).into()),
            },
            _ => Flt(self.to_float(prec).pow(other.to_float(prec)))
        }
    }

    pub(crate) fn log(&self, base: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            _ => todo!(),
        }
    }

    pub(crate) fn abs(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => Int(integer.abs()),
            SmallRat(small_rat) => todo!(),
            Rat(rational) => Rat(rational.abs()),
            Flt(float) => Flt(float.abs()),
        }
    }
    pub(crate) fn floor(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => self.clone(),
            SmallRat(small_rat) => todo!(),
            Rat(rational) => Int(rational.floor().into()),
            _ => Int(self.to_float(prec).floor().into())
        }
    }
    pub(crate) fn ceil(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => self.clone(),
            SmallRat(small_rat) => todo!(),
            Rat(rational) => Int(rational.ceil().into()),
            _ => Int(self.to_float(prec).ceil().into())
        }
    }
    pub(crate) fn round(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => self.clone(),
            SmallRat(small_rat) => todo!(),
            Rat(rational) => Int(rational.round().into()),
            _ => Int(self.to_float(prec).round().into())
        }
    }
    pub(crate) fn fac(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => {
                if integer <= &0 {
                    Flt(self.to_float(prec).gamma())
                } else {
                    Int({
                        let mut result = Integer::from(1);
                        let mut i = Integer::from(1);
                        while i < integer {
                            i.add_assign(1);
                            result.mul_assign(i);
                        }
                        result
                    })
                }
            },
            _ => Flt(self.to_float(prec).gamma())
        }
    }

    pub(crate) fn to_float(&self, prec:u32) -> Float {
        let mut result = Float::new(prec);
        match self {
            SmallInt(int) => todo!(),
            Int(integer) => result.assign(integer),
            SmallRat(small_rat) => todo!(),
            Rat(rational) => result.assign(rational),
            Flt(float) => result.assign(float),
        }
        result
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::SmallInt(value)
    }
}

impl From<Integer> for Number {
    fn from(value: Integer) -> Self {
        Number::Int(value)
    }
}

impl From<SmallRational> for Number {
    fn from(value: SmallRational) -> Self {
        Number::SmallRat(value)
    }
}

impl From<Rational> for Number {
    fn from(value: Rational) -> Self {
        Number::Rat(value)
    }
}

impl From<Float> for Number {
    fn from(value: Float) -> Self {
        Number::Flt(value)
    }
}