use std::{
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
    },
};

use rug::{Float, Integer};

use crate::{
    error::FirkinError,
    num::{int::IntegerWrapper, number::NumberWrapper},
};

pub(crate) trait PythonNum:
    Sized + Sub + SubAssign + Add + AddAssign + Div + DivAssign + Mul + MulAssign + PythonMathFunctions
{
}

pub(crate) trait PythonRat: Sized + Hash + PythonNum {}

#[allow(unused)]
pub(crate) trait PythonInt:
    Sized
    + Rem
    + RemAssign
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Not
    + PythonNum
{
    fn py_index(&self) -> Result<IntegerWrapper, FirkinError>;
}

pub(crate) trait PythonMathFunctions: Sized + Neg + PartialOrd + PartialEq {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError>;
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError>;
    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError>;
    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError>;
    fn py_divmod(
        &self,
        other: &NumberWrapper,
    ) -> Result<(IntegerWrapper, NumberWrapper), FirkinError> {
        Ok((self.py_floordiv(other)?, self.py_mod(other)?))
    }
    fn py_abs(&self) -> Result<NumberWrapper, FirkinError>;
    fn py_int(&self) -> Result<IntegerWrapper, FirkinError>;
    fn py_float(&self) -> Result<f64, FirkinError>;
    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError>;
    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError>;
    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError>;
    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError>;
}

#[allow(unused)]
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
