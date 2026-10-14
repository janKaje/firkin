#![allow(unused)]

use std::ops::{Div, Mul, Rem, Shl};

use rug::{
    Float, Integer, Rational,
    integer::IntegerExt64,
    ops::{Pow, PowAssign},
};

use crate::{
    error::FirkinError, num::{
        float::{DEFAULT_PREC, FloatWrapper::{self, BigFloat, SmallFloat}}, int::IntegerWrapper::{self, Int, SmallInt}, number::NumberWrapper, rat::RationalWrapper, traits::NumpyUFuncs,
    },
};

impl NumpyUFuncs for i64 {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &0 {
            Ok(NumberWrapper::ONE)
        } else {
            FloatWrapper::from(*self).np_exp()
        }
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &0 {
            Ok(NumberWrapper::ONE)
        } else if self < &0 || *self > u32::MAX as i64 {
            FloatWrapper::from(*self).np_exp2()
        } else {
            match 1_i64.checked_shl(self.unsigned_abs() as u32) {
                Some(r) => Ok(r.into()),
                None => Integer::from(*self).np_exp2(),
            }
        }
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else {
            FloatWrapper::from(*self).np_log()
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else if self
            .checked_sub(1)
            .and_then(|i| Some(self & i == 0))
            .unwrap_or(false)
        {
            // perfect power of 2
            Ok(self.ilog2().into())
        } else {
            FloatWrapper::from(*self).np_log()
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else {
            let as_log10 = self.ilog10();
            if &10_i64.pow(as_log10) == self {
                Ok(as_log10.into())
            } else {
                FloatWrapper::from(*self).np_log()
            }
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        if self < &0 {
            Err(FirkinError::Undefined("sqrt".into(), self.to_string()))
        } else {
            let as_sqrt = self.isqrt() as u32;
            if &(as_sqrt as i64).pow(as_sqrt) == self {
                Ok(as_sqrt.into())
            } else {
                FloatWrapper::from(*self).np_sqrt()
            }
        }
    }
}

impl NumpyUFuncs for Integer {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &0 {
            Ok(NumberWrapper::ONE)
        } else {
            FloatWrapper::from(self).np_exp()
        }
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &0 {
            Ok(NumberWrapper::ONE)
        } else if self < &0 || self > &u32::MAX {
            FloatWrapper::from(self).np_exp2()
        } else {
            Ok(Integer::from(2).pow(self.to_u32_wrapping()).into())
        }
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else {
            FloatWrapper::from(self).np_log()
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else if self.is_power_of_two() {
            // perfect power of 2
            Ok(self.find_one(0).unwrap().into())
        } else {
            FloatWrapper::from(self).np_log()
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        if self == &1 {
            Ok(NumberWrapper::ZERO)
        } else if self <= &0 {
            Err(FirkinError::Undefined(
                "log (natural log)".into(),
                self.to_string(),
            ))
        } else {
            FloatWrapper::from(self).np_log()
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        if self < &0 {
            Err(FirkinError::Undefined("sqrt".into(), self.to_string()))
        } else if self.is_perfect_square() {
            Ok(Integer::from(self.sqrt_ref()).into())
        } else {
            FloatWrapper::from(self).np_sqrt()
        }
    }
}

impl NumpyUFuncs for f64 {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        let as_exp = self.exp();
        if as_exp.is_finite() {
            Ok(self.exp().into())
        }
        else {
            Float::with_val(DEFAULT_PREC, self).np_exp()
        }
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        let as_exp = self.exp();
        if as_exp.is_finite() {
            Ok(self.exp2().into())
        }
        else {
            Float::with_val(DEFAULT_PREC, self).np_exp2()
        }
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("ln".into(), self.to_string()))
        } else {
            Ok(self.ln().into())
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("log2".into(), self.to_string()))
        } else {
            Ok(self.log2().into())
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("log10".into(), self.to_string()))
        } else {
            Ok(self.log10().into())
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        if self < &0.0 {
            Err(FirkinError::Undefined("sqrt".into(), self.to_string()))
        } else {
            Ok(self.sqrt().into())
        }
    }
}

impl NumpyUFuncs for Float {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(FloatWrapper::from_incomplete_float(self.exp_ref()).into())
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(FloatWrapper::from_incomplete_float(self.exp2_ref()).into())
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("ln".into(), self.to_string()))
        } else {
            Ok(FloatWrapper::from_incomplete_float(self.ln_ref()).into())
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("log2".into(), self.to_string()))
        } else {
            Ok(FloatWrapper::from_incomplete_float(self.log2_ref()).into())
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        if self <= &0.0 {
            Err(FirkinError::Undefined("log10".into(), self.to_string()))
        } else {
            Ok(FloatWrapper::from_incomplete_float(self.log10_ref()).into())
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        if self < &0.0 {
            Err(FirkinError::Undefined("sqrt".into(), self.to_string()))
        } else {
            Ok(FloatWrapper::from_incomplete_float(self.sqrt_ref()).into())
        }
    }
}

impl NumpyUFuncs for Rational {
    fn np_exp(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 0 {
            Ok(NumberWrapper::ONE)
        } else {
            FloatWrapper::from(self).np_exp()
        }
    }

    fn np_exp2(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 0 {
            Ok(NumberWrapper::ONE)
        } else {
            FloatWrapper::from(self).np_exp2()
        }
    }

    fn np_log(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 1 {
            Ok(NumberWrapper::ZERO)
        } else if *self <= 0 {
            Err(FirkinError::Undefined("ln".into(), self.to_string()))
        } else {
            FloatWrapper::from(self).np_log()
        }
    }

    fn np_log2(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 1 {
            Ok(NumberWrapper::ZERO)
        } else if *self <= 0 {
            Err(FirkinError::Undefined("log2".into(), self.to_string()))
        } else {
            FloatWrapper::from(self).np_log2()
        }
    }

    fn np_log10(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 1 {
            Ok(NumberWrapper::ZERO)
        } else if *self <= 0 {
            Err(FirkinError::Undefined("log10".into(), self.to_string()))
        } else {
            FloatWrapper::from(self).np_log10()
        }
    }

    fn np_sqrt(&self) -> Result<NumberWrapper, FirkinError> {
        if *self == 1 {
            Ok(NumberWrapper::ONE)
        } else if *self < 0 {
            Err(FirkinError::Undefined("sqrt".into(), self.to_string()))
        } else if self.is_integer() {
            Ok(IntegerWrapper::from(self.numer()).into())
        } else {
            FloatWrapper::from(self).np_log()
        }
    }
}
