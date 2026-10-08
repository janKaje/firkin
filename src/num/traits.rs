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

use rug::{Float, Integer};

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
    fn pow(
        &self,
        other: &NumberWrapper,
        modulus: &NumberWrapper,
    ) -> Result<NumberWrapper, &'static str>;
    fn abs(&self) -> Result<NumberWrapper, &'static str>;
    fn int(&self) -> Result<IntegerWrapper, &'static str>;
    fn to_f64(&self) -> Result<f64, &'static str>;
    fn round(&self, ndigits: &IntegerWrapper) -> Result<IntegerWrapper, &'static str>;
    fn floor(&self) -> Result<IntegerWrapper, &'static str>;
    fn ceil(&self) -> Result<IntegerWrapper, &'static str>;
    fn trunc(&self) -> Result<IntegerWrapper, &'static str>;
}

pub(crate) trait NumpyUFuncs: PythonMathFunctions {}

pub(crate) trait FromFloats: From<f64> + From<f32> + From<Float> {}

pub(crate) trait TryFromFloats: TryFrom<f64> + TryFrom<f32> + TryFrom<Float> {}

pub(crate) trait FromInts:
    From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<i128>
    + From<isize>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<u128>
    + From<usize>
    + From<Integer>
{
}
