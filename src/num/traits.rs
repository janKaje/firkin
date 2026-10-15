use std::{
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
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

#[allow(unused)]
pub(crate) trait PythonRat: Sized + Hash + PythonNum {}

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
    + Shl<u32>
    + ShlAssign<u32>
    + Shr<u32>
    + ShrAssign<u32>
    + PythonNum
{
    fn py_index(&self) -> Result<IntegerWrapper, FirkinError>;
}

pub(crate) trait PythonMathFunctions: Sized + Neg + PartialOrd + PartialEq {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError>;
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<NumberWrapper>,
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

pub(crate) trait NumpyUFuncs: PythonMathFunctions {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError>;
    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError>;
    fn np_log(&self) -> Result<NumberWrapper, FirkinError>;
    fn np_log2(&self) -> Result<NumberWrapper, FirkinError>;
    fn np_log10(&self) -> Result<NumberWrapper, FirkinError>;
    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError>;
}

#[allow(unused)]
pub(crate) trait FromFloats: From<f64> + From<f32> + From<Float> {}

#[allow(unused)]
pub(crate) trait TryFromFloats: TryFrom<f64> + TryFrom<f32> + TryFrom<Float> {}

#[allow(unused)]
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

pub(crate) trait RichDisplay {
    fn display_default(&self) -> String;
    fn display_debug(&self) -> String;
    fn format(&self, format_specifier: String) -> String;
}

macro_rules! auto_impl_pymath_trait {
    (
        $Ty:ty {$($Var:ident)+}
    ) => {
        impl PythonMathFunctions for $Ty {
            fn py_pow(
                &self,
                other: &NumberWrapper,
                modulus: Option<NumberWrapper>,
            ) -> Result<NumberWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_pow(other, modulus)
                    ),+,
                }
            }

            fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_abs()
                    ),+,
                }
            }

            fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_int()
                    ),+,
                }
            }

            fn py_float(&self) -> Result<f64, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_float()
                    ),+,
                }
            }

            fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_round(ndigits)
                    ),+,
                }
            }

            fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_floor()
                    ),+,
                }
            }

            fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_ceil()
                    ),+,
                }
            }

            fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_trunc()
                    ),+,
                }
            }

            fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_truediv(other)
                    ),+,
                }
            }

            fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_floordiv(other)
                    ),+,
                }
            }

            fn py_divmod(
                &self,
                other: &NumberWrapper,
            ) -> Result<(IntegerWrapper, NumberWrapper), FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_divmod(other)
                    ),+,
                }
            }

            fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
                match self {
                    $(
                        $Var(i) => i.py_mod(other)
                    ),+,
                }
            }
        }
    }
}
pub(crate) use auto_impl_pymath_trait;

macro_rules! auto_impl_numpy_ufunc_trait {
    (
        $Ty:ty {$($Var:ident)+}
    ) => {

    impl NumpyUFuncs for $Ty {

    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_exp()
            ),+
        }
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_exp2()
            ),+
        }
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_log()
            ),+
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_log2()
            ),+
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_log10()
            ),+
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        match self {
            $(
                $Var(i) => i.np_sqrt()
            ),+
        }
    }
}
    };
}
pub(crate) use auto_impl_numpy_ufunc_trait;
