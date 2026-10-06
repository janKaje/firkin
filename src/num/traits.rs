#![allow(unused)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use crate::num::{
    float::FloatWrapper, int::IntegerWrapper, number::NumberWrapper, rat::RationalWrapper,
};

pub(crate) trait PythonMathFunctions: Sized + Add + AddAssign + Div + DivAssign + Mul + MulAssign + Neg + Rem + Sub + SubAssign {
    fn pow(&self, other: &NumberWrapper, modulus: &NumberWrapper) -> NumberWrapper;
    fn abs(&self) -> NumberWrapper;
    fn int(&self) -> NumberWrapper;
    fn float(&self) -> NumberWrapper;
    fn round(&self, ndigits: &IntegerWrapper) -> NumberWrapper;
}

pub(crate) trait NumpyUFuncs {}