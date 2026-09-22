use std::ops::{self, AddAssign, MulAssign};
// use rug::{Float, Integer, Rational, ops::Pow};
// use rug::{Assign, float};
// use rug::float::{FreeCache, Round};
// use rug::ops::{AddAssignRound, AssignRound, MulAssignRound};

use super::smallrat::SmallRat;

const DEFAULT_PREC: u32 = 53;

/// Single number, whether it's integer, rational, or irrational
#[derive(Clone)]
pub(crate) enum Number {
    SmallInt(i64),
    Int(Integer),
    SmallRat(SmallRat),
    Rat(Rational),
    Float(Float),
}

impl Number {

    pub(crate) fn add(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => match other {
                Number::Int(integer1) => Number::Int(Integer::from(integer + integer1)),
                Number::Rat(rational) => Number::Rat(Rational::from(rational + integer)),
                _ => other.add(self, Some(prec))
            },
            Number::Rat(rational) => match other {
                Number::Rat(rational1) => Number::Rat(Rational::from(rational + rational1)),
                _ => other.add(self, Some(prec))
            },
            Number::Float(float) => Number::Float(Float::from(float + other.to_float(prec))),
            Number::SmallInt(integer) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
    }

    pub(crate) fn sub(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => match other {
                Number::Int(integer1) => Number::Int(Integer::from(integer-integer1)),
                Number::Rat(rational) => Number::Rat(Rational::from(integer-rational)),
                Number::Float(float) => Number::Float(Float::from(integer - float)),
                Number::SmallInt(_) => todo!(),
                Number::SmallRat(small_rat) => todo!(),
            },
            Number::Rat(rational) => match other {
                Number::Int(integer) => Number::Rat(Rational::from(rational - integer)),
                Number::Rat(rational1) => Number::Rat(Rational::from(rational - rational1)),
                Number::Float(float) => Number::Float(Float::from(rational - float)),
                Number::SmallInt(_) => todo!(),
                Number::SmallRat(small_rat) => todo!(),
            },
            Number::Float(float) => Number::Float(Float::from(float - other.to_float(prec))),
            Number::SmallInt(_) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
    }

    pub(crate) fn mul(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => match other {
                Number::Int(integer1) => Number::Int(Integer::from(integer * integer1)),
                Number::Rat(rational) => Number::Rat(Rational::from(rational * integer)),
                _ => other.mul(self, Some(prec))
            },
            Number::Rat(rational) => match other {
                Number::Rat(rational1) => Number::Rat(Rational::from(rational * rational1)),
                _ => other.mul(self, Some(prec))
            },
            Number::Float(float) => Number::Float(Float::from(float * other.to_float(prec))),
            Number::SmallInt(_) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
    }

    pub(crate) fn div(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => match other {
                Number::Int(integer1) => Number::Int(Integer::from(integer/integer1)),
                Number::Rat(rational) => Number::Rat(Rational::from(integer/rational)),
                Number::Float(float) => Number::Float(Float::from(integer/float)),
                Number::SmallInt(_) => todo!(),
                Number::SmallRat(small_rat) => todo!(),
            },
            Number::Rat(rational) => match other {
                Number::Int(integer) => Number::Rat(Rational::from(rational/integer)),
                Number::Rat(rational1) => Number::Rat(Rational::from(rational / rational1)),
                Number::Float(float) => Number::Float(Float::from(rational / float)),
                Number::SmallInt(_) => todo!(),
                Number::SmallRat(small_rat) => todo!(),
            },
            Number::Float(float) => Number::Float(Float::from(float / other.to_float(prec))),
            Number::SmallInt(_) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
    }

    pub(crate) fn pow(&self, other: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match other {
            Number::Int(integer) => match self {
                Number::Int(integer1) => Number::Int(Integer::from(integer1.pow(integer))),
                Number::Rat(rational) => Number::Rat(Rational::from(rational.pow(integer))),
                Number::Float(float) => Number::Float(Float::from(float.pow(integer))),
                Number::SmallInt(_) => todo!(),
                Number::SmallRat(small_rat) => todo!(),
            },
            _ => Number::Float(self.to_float(prec).pow(other.to_float(prec)))
        }
    }

    pub(crate) fn log(&self, base: &Number, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::EToThe(number) => todo!(),
            _ => todo!(),
        }
    }

    pub(crate) fn abs(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => Number::Int(integer.abs()),
            Number::Rat(rational) => Number::Rat(rational.abs()),
            Number::Float(float) => Number::Float(float.abs()),
            Number::SmallInt(_) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
    }
    pub(crate) fn floor(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => self.clone(),
            Number::Rat(rational) => Number::Int(rational.floor().into()),
            _ => Number::Int(self.to_float(prec).floor().into())
        }
    }
    pub(crate) fn ceil(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => self.clone(),
            Number::Rat(rational) => Number::Int(rational.ceil().into()),
            _ => Number::Int(self.to_float(prec).ceil().into())
        }
    }
    pub(crate) fn round(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => self.clone(),
            Number::Rat(rational) => Number::Int(rational.round().into()),
            _ => Number::Int(self.to_float(prec).round().into())
        }
    }
    pub(crate) fn fac(&self, prec: Option<u32>) -> Number {
        let prec: u32 = prec.unwrap_or(DEFAULT_PREC);
        match self {
            Number::Int(integer) => {
                if integer <= &0 {
                    Number::Float(self.to_float(prec).gamma())
                } else {
                    Number::Int({
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
            _ => Number::Float(self.to_float(prec).gamma())
        }
    }

    pub(crate) fn to_float(&self, prec:u32) -> Float {
        let mut result = Float::new(prec);
        match self {
            Number::Int(integer) => result.assign(integer),
            Number::Rat(rational) => result.assign(rational),
            Number::Float(float) => result.assign(float),
            Number::SmallInt(_) => todo!(),
            Number::SmallRat(small_rat) => todo!(),
        }
        result
    }
}
