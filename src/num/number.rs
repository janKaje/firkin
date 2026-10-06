#![allow(unused)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::num::{float::FloatWrapper, int::IntegerWrapper, rat::RationalWrapper};

/// Single number, whether it's integer, rational, or irrational
#[derive(Clone)]
pub(crate) enum Number {
    Int(IntegerWrapper),
    Rat(RationalWrapper),
    Float(FloatWrapper),
}

use Number::{Float, Int, Rat};

impl Number {
    pub(crate) fn add(&self, other: &Number) -> Number {
        match (self, other) {
            (Int(i), Int(i2)) => Int(i + i2),
            (Int(i), Rat(r)) => Rat(i + r),
            (Int(i), Float(f)) => Float(i + f),
            (Rat(r), Int(i)) => Rat(r + i),
            (Rat(r), Rat(r2)) => Rat(r + r2),
            (Rat(r), Float(f)) => Float(r + f),
            (Float(f), Int(i)) => Float(f + i),
            (Float(f), Rat(r)) => Float(f + r),
            (Float(f), Float(f2)) => Float(f + f2),
        }
    }

    pub(crate) fn sub(&self, other: &Number) -> Number {
        match (self, other) {
            (Int(i), Int(i2)) => Int(i - i2),
            (Int(i), Rat(r)) => Rat(i - r),
            (Int(i), Float(f)) => Float(i - f),
            (Rat(r), Int(i)) => Rat(r - i),
            (Rat(r), Rat(r2)) => Rat(r - r2),
            (Rat(r), Float(f)) => Float(r - f),
            (Float(f), Int(i)) => Float(f - i),
            (Float(f), Rat(r)) => Float(f - r),
            (Float(f), Float(f2)) => Float(f - f2),
        }
    }

    pub(crate) fn mul(&self, other: &Number) -> Number {
        match (self, other) {
            (Int(i), Int(i2)) => Int(i * i2),
            (Int(i), Rat(r)) => Rat(i * r),
            (Int(i), Float(f)) => Float(i * f),
            (Rat(r), Int(i)) => Rat(r * i),
            (Rat(r), Rat(r2)) => Rat(r * r2),
            (Rat(r), Float(f)) => Float(r * f),
            (Float(f), Int(i)) => Float(f * i),
            (Float(f), Rat(r)) => Float(f * r),
            (Float(f), Float(f2)) => Float(f * f2),
        }
    }

    pub(crate) fn div(&self, other: &Number) -> Number {
        match (self, other) {
            (Int(i), Int(i2)) => Int(i / i2),
            (Int(i), Rat(r)) => Rat(i / r),
            (Int(i), Float(f)) => Float(i / f),
            (Rat(r), Int(i)) => Rat(r / i),
            (Rat(r), Rat(r2)) => Rat(r / r2),
            (Rat(r), Float(f)) => Float(r / f),
            (Float(f), Int(i)) => Float(f / i),
            (Float(f), Rat(r)) => Float(f / r),
            (Float(f), Float(f2)) => Float(f / f2),
        }
    }

    pub(crate) fn pow(&self, other: &Number) -> Number {
        todo!()
    }

    pub(crate) fn log(&self, base: &Number) -> Number {
        todo!()
    }

    pub(crate) fn abs(&self) -> Number {
        match self {
            Int(i) => Int(i.abs()),
            Rat(r) => Rat(r.abs()),
            Float(f) => Float(f.abs()),
        }
    }
    pub(crate) fn floor(&self) -> Number {
        match self {
            Int(i) => Int(i.floor()),
            Rat(r) => Int(r.floor()),
            Float(f) => Int(f.floor()),
        }
    }
    pub(crate) fn ceil(&self) -> Number {
        match self {
            Int(i) => Int(i.ceil()),
            Rat(r) => Int(r.ceil()),
            Float(f) => Int(f.ceil()),
        }
    }
    pub(crate) fn round(&self) -> Number {
        match self {
            Int(i) => Int(i.round()),
            Rat(r) => Int(r.round()),
            Float(f) => Int(f.round()),
        }
    }

    pub(crate) fn to_f64(&self) -> f64 {
        match self {
            Int(i) => i.to_f64(),
            Rat(r) => r.to_f64(),
            Float(f) => f.to_f64(),
        }
    }
}

macro_rules! arith_binary {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<Number> for Number {
            type Output = Number;

            fn $method(self, rhs: Number) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<&Number> for &Number {
            type Output = Number;

            fn $method(self, rhs: &Number) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<&Number> for Number {
            type Output = Number;

            fn $method(self, rhs: &Number) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
                }
            }
        }

        impl $Imp<Number> for &Number {
            type Output = Number;

            fn $method(self, rhs: Number) -> Self::Output {
                match (self, rhs) {
                    (Int(i), Int(i2)) => Int(i.$method(i2)),
                    (Int(i), Rat(r)) => Rat(i.$method(r)),
                    (Int(i), Float(f)) => Float(i.$method(f)),
                    (Rat(r), Int(i)) => Rat(r.$method(i)),
                    (Rat(r), Rat(r2)) => Rat(r.$method(r2)),
                    (Rat(r), Float(f)) => Float(r.$method(f)),
                    (Float(f), Int(i)) => Float(f.$method(i)),
                    (Float(f), Rat(r)) => Float(f.$method(r)),
                    (Float(f), Float(f2)) => Float(f.$method(f2)),
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

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Int(i) => Int(-i),
            Rat(r) => Rat(-r),
            Float(f) => Float(-f),
        }
    }
}

impl From<IntegerWrapper> for Number {
    fn from(value: IntegerWrapper) -> Self {
        Int(value)
    }
}

impl From<RationalWrapper> for Number {
    fn from(value: RationalWrapper) -> Self {
        Rat(value)
    }
}

impl From<FloatWrapper> for Number {
    fn from(value: FloatWrapper) -> Self {
        Float(value)
    }
}
