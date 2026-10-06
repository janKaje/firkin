#![allow(unused)]

use std::{
    hash::Hash,
    io::Bytes,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

use crate::num::{
    float::FloatWrapper, int::IntegerWrapper, number::NumberWrapper, rat::RationalWrapper,
};

pub(crate) trait PythonMathFunctions:
    Sized
    + Add
    + AddAssign
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + Neg
    + Rem
    + RemAssign
    + Sub
    + SubAssign
    + PartialOrd
    + PartialEq
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Not
    + Shl
    + ShlAssign
    + Shr
    + ShrAssign
    + Hash
{
    fn pow(&self, other: &NumberWrapper, modulus: &NumberWrapper) -> NumberWrapper;
    fn abs(&self) -> NumberWrapper;
    fn int(&self) -> IntegerWrapper;
    fn float(&self) -> FloatWrapper;
    fn round(&self, ndigits: &IntegerWrapper) -> IntegerWrapper;
    fn floor(&self) -> IntegerWrapper;
    fn ceil(&self) -> IntegerWrapper;
    fn trunc(&self) -> IntegerWrapper;
    fn index(&self);
}

pub(crate) trait NumpyUFuncs: PythonMathFunctions {}
