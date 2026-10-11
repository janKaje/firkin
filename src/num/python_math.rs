use std::ops::{Div, Mul, Rem};

use rug::{
    Float, Integer, Rational,
    integer::IntegerExt64,
    ops::{Pow, PowAssign},
};

use crate::{
    error::FirkinError,
    num::{
        float::FloatWrapper::{self, BigFloat, SmallFloat},
        int::IntegerWrapper::{self, Int, SmallInt},
        number::NumberWrapper,
        rat::RationalWrapper,
        traits::PythonMathFunctions,
    },
};

impl PythonMathFunctions for i64 {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        if *self == 0 {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                SmallInt(i_exp) => match modulus {
                    Some(m) => match m {
                        NumberWrapper::Int(integer_mod) => {
                            match Integer::from(*self)
                                .pow_mod(&Integer::from(*i_exp), &integer_mod.as_integer())
                            {
                                Ok(result) => Ok(NumberWrapper::from(result)),
                                Err(_) => Err(FirkinError::PowModDoesNotExist(
                                    self.to_string(),
                                    other.to_string(),
                                    match modulus {
                                        Some(m) => m.to_string(),
                                        None => String::from("no modulus"),
                                    },
                                )),
                            }
                        }
                        _ => FloatWrapper::from(*self).py_pow(other, modulus),
                    },
                    None => {
                        if i_exp.is_positive() {
                            Ok(NumberWrapper::from(Integer::from(Integer::i64_pow_u64(
                                *self,
                                i_exp.unsigned_abs(),
                            ))))
                        } else {
                            Ok(NumberWrapper::Rat(RationalWrapper::from((
                                Integer::ONE,
                                Integer::i64_pow_u64(*self, i_exp.unsigned_abs()),
                            ))))
                        }
                    }
                },
                Int(_) => FloatWrapper::from(*self).py_pow(other, modulus),
            },
            _ => FloatWrapper::from(*self).py_pow(other, modulus),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(NumberWrapper::from(self.abs()))
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(*self as f64)
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if ndigits < 0 {
            let pow = match 10_i64.checked_pow(ndigits.unsigned_abs()) {
                Some(r) => r,
                None => {
                    if ndigits == 20 {
                        if self >= &5000000000000000000 {
                            return Ok(NumberWrapper::from(10000000000000000000_i128));
                        } else {
                            return Ok(NumberWrapper::from(0));
                        }
                    } else {
                        return Ok(NumberWrapper::from(0));
                    }
                }
            };
            let rem = self % pow;
            let result;
            if rem >= pow / 2 {
                result = self - rem + pow;
            } else {
                result = self - rem;
            }
            Ok(NumberWrapper::from(result))
        } else {
            Ok(NumberWrapper::from(*self))
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(*self))
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        }
        match other {
            NumberWrapper::Int(i) => Ok(NumberWrapper::Rat(RationalWrapper::from((*self, i)))),
            NumberWrapper::Rat(r) => Ok(NumberWrapper::Rat(RationalWrapper::from(*self) / r)),
            NumberWrapper::Float(f) => Ok(NumberWrapper::Float(FloatWrapper::from(*self) / f)),
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        }
        match other {
            NumberWrapper::Int(i) => Ok({
                match i {
                    SmallInt(int) => IntegerWrapper::from(self / int),
                    Int(integer) => IntegerWrapper::from(Integer::from(self / integer)),
                }
            }),
            NumberWrapper::Rat(r) => Ok((IntegerWrapper::from(*self) / r).to_integer()),
            NumberWrapper::Float(f) => Ok({
                match (IntegerWrapper::from(*self) / f).to_integer() {
                    Ok(r) => r,
                    Err(_) => IntegerWrapper::ZERO,
                }
            }),
        }
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(int) => match int {
                SmallInt(i) => Ok(NumberWrapper::from(self.rem(i))),
                Int(integer) => Ok(NumberWrapper::from(Integer::from(self.rem(integer)))),
            },
            NumberWrapper::Rat(_) => RationalWrapper::from(*self).py_mod(other),
            NumberWrapper::Float(_) => FloatWrapper::from(*self).py_mod(other),
        }
    }
}

impl PythonMathFunctions for Integer {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                SmallInt(i_exp) => match modulus {
                    Some(m) => match m {
                        NumberWrapper::Int(integer_mod) => {
                            match self
                                .clone()
                                .pow_mod(&Integer::from(*i_exp), &integer_mod.as_integer())
                            {
                                Ok(result) => Ok(NumberWrapper::from(result)),
                                Err(_) => Err(FirkinError::PowModDoesNotExist(
                                    self.to_string(),
                                    other.to_string(),
                                    match modulus {
                                        Some(m) => m.to_string(),
                                        None => String::from("no modulus"),
                                    },
                                )),
                            }
                        }
                        _ => FloatWrapper::from(self).py_pow(other, modulus),
                    },
                    None => {
                        if i_exp.unsigned_abs() > u32::MAX as u64 {
                            FloatWrapper::from(self).py_pow(other, modulus)
                        } else if i_exp.is_positive() {
                            Ok(NumberWrapper::from(Integer::from(
                                self.pow(i_exp.unsigned_abs() as u32),
                            )))
                        } else {
                            Ok(NumberWrapper::Rat(RationalWrapper::from((
                                Integer::ONE,
                                self.pow(i_exp.unsigned_abs() as u32),
                            ))))
                        }
                    }
                },
                Int(_) => FloatWrapper::from(self).py_pow(other, modulus),
            },
            _ => FloatWrapper::from(self).py_pow(other, modulus),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(NumberWrapper::from(self.clone().abs()))
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(self.clone()))
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(self.to_f64())
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if ndigits < 0 {
            let pow = Integer::from(10).pow(ndigits.unsigned_abs());
            let rem = Integer::from(self % &pow);
            let result;
            if rem >= Integer::from(&rem / 2) {
                result = self - rem + pow;
            } else {
                result = self - rem;
            }
            Ok(NumberWrapper::from(result))
        } else {
            Ok(NumberWrapper::from(self.clone()))
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(self.clone()))
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(self.clone()))
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(self.clone()))
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        }
        match other {
            NumberWrapper::Int(i) => Ok(NumberWrapper::Rat(RationalWrapper::from((self, i)))),
            NumberWrapper::Rat(r) => Ok(NumberWrapper::Rat(RationalWrapper::from(self) / r)),
            NumberWrapper::Float(f) => Ok(NumberWrapper::Float(FloatWrapper::from(self) / f)),
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        }
        match other {
            NumberWrapper::Int(i) => Ok({
                match i {
                    SmallInt(int) => IntegerWrapper::from(Integer::from(self / int)),
                    Int(integer) => IntegerWrapper::from(Integer::from(self / integer)),
                }
            }),
            NumberWrapper::Rat(r) => Ok((IntegerWrapper::from(self) / r).to_integer()),
            NumberWrapper::Float(f) => Ok({
                match (IntegerWrapper::from(self) / f).to_integer() {
                    Ok(r) => r,
                    Err(_) => IntegerWrapper::ZERO,
                }
            }),
        }
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(int) => match int {
                SmallInt(i) => Ok(NumberWrapper::from(Integer::from(self.rem(i)))),
                Int(integer) => Ok(NumberWrapper::from(Integer::from(self.rem(integer)))),
            },
            NumberWrapper::Rat(_) => RationalWrapper::from(self).py_mod(other),
            NumberWrapper::Float(_) => FloatWrapper::from(self).py_mod(other),
        }
    }
}

impl PythonMathFunctions for Rational {
    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        } else if self.is_integer() {
            return self
                .clone()
                .fract_trunc(Integer::new())
                .1
                .py_pow(other, modulus);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match modulus {
                Some(m) => self.py_pow(other, None)?.py_mod(m),
                None => match integer_wrapper {
                    SmallInt(i) => {
                        if i.unsigned_abs() > u32::MAX as u64 {
                            FloatWrapper::from(self).py_pow(other, modulus)
                        } else if i.is_positive() {
                            let mut clone = self.clone();
                            clone.mutate_numer_denom(|num, den| {
                                num.pow_assign(i.unsigned_abs() as u32);
                                den.pow_assign(i.unsigned_abs() as u32);
                            });
                            Ok(NumberWrapper::from(clone))
                        } else {
                            let mut clone = self.clone();
                            clone.mutate_numer_denom(|num, den| {
                                num.pow_assign(i.unsigned_abs() as u32);
                                den.pow_assign(i.unsigned_abs() as u32);
                            });
                            Ok(NumberWrapper::from(1 / clone))
                        }
                    }
                    Int(_) => FloatWrapper::from(self).py_pow(other, modulus),
                },
            },
            _ => FloatWrapper::from(self).py_pow(other, modulus),
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                SmallInt(i) => Ok(NumberWrapper::from(Rational::from(self.div(i)))),
                Int(integer) => Ok(NumberWrapper::from(Rational::from(self.div(integer)))),
            },
            NumberWrapper::Rat(rational_wrapper) => match rational_wrapper {
                RationalWrapper::SmallRat(r) => {
                    Ok(NumberWrapper::from(Rational::from(self.div(r))))
                }
                RationalWrapper::Rat(rational) => {
                    Ok(NumberWrapper::from(Rational::from(self.div(rational))))
                }
            },
            NumberWrapper::Float(_) => FloatWrapper::from(self).py_truediv(other),
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(IntegerWrapper::ZERO);
        } else if self.is_integer() {
            return self
                .clone()
                .fract_trunc(Integer::new())
                .1
                .py_floordiv(other);
        }
        match other {
            NumberWrapper::Int(int) => match int {
                IntegerWrapper::SmallInt(i) => {
                    Ok(RationalWrapper::from(Rational::from(self.div(i)).floor()).to_integer())
                }
                IntegerWrapper::Int(integer) => Ok(RationalWrapper::from(
                    Rational::from(self.div(integer)).floor(),
                )
                .to_integer()),
            },
            NumberWrapper::Rat(rat) => match rat {
                RationalWrapper::SmallRat(r) => {
                    Ok(RationalWrapper::from(Rational::from(self.div(r)).floor()).to_integer())
                }
                RationalWrapper::Rat(rational) => Ok(RationalWrapper::from(
                    Rational::from(self.div(rational)).floor(),
                )
                .to_integer()),
            },
            NumberWrapper::Float(_) => FloatWrapper::from(self).py_floordiv(other),
        }
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(self.clone().abs().into())
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(RationalWrapper::from(self).to_integer())
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(self.clone().to_f64())
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if ndigits < 0 {
            let div = Integer::from(10).pow(ndigits.unsigned_abs());
            Ok(NumberWrapper::from(
                RationalWrapper::from(Rational::from(self / &div).round() * div).to_integer(),
            ))
        } else if ndigits == 0 {
            Ok(NumberWrapper::from(
                RationalWrapper::from(self.clone().round()).to_integer(),
            ))
        } else {
            let mul = Integer::from(10).pow(ndigits.unsigned_abs());
            Ok(NumberWrapper::from(
                Rational::from(self * &mul).round() / mul,
            ))
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(RationalWrapper::from(self).floor())
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(RationalWrapper::from(self).ceil())
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(RationalWrapper::from(self).to_integer())
    }

    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => {
                let (frac, trunc) = self.clone().fract_trunc(Integer::new());
                let rem_trunc = match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => trunc % i,
                    IntegerWrapper::Int(integer) => trunc % integer,
                };
                Ok(NumberWrapper::from(frac + rem_trunc))
            }
            NumberWrapper::Rat(rational_wrapper) => match rational_wrapper {
                RationalWrapper::SmallRat(r) => {
                    Ok(NumberWrapper::from(self - r * &(self / r).floor()))
                }
                RationalWrapper::Rat(rational) => Ok(NumberWrapper::from(
                    self - rational * Rational::from(self / rational).floor(),
                )),
            },
            NumberWrapper::Float(_) => FloatWrapper::from(self).py_mod(other),
        }
    }
}

impl PythonMathFunctions for f64 {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok(NumberWrapper::from(
                    FloatWrapper::from_incomplete_float(self.rem(*i as f64)),
                )),
                IntegerWrapper::Int(integer) => Ok(NumberWrapper::from(self.rem(integer.to_f64()))),
            },
            NumberWrapper::Rat(_) => Ok(self.rem(other.to_f64()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(NumberWrapper::from(self.rem(f))),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self.rem(float)).into()),
            },
        }
    }

    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match modulus {
            Some(m) => self.py_pow(other, None)?.py_mod(m),
            None => match other {
                NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => Ok(self.powf(*i as f64).into()),
                    IntegerWrapper::Int(integer) => Ok(self.powf(integer.to_f64()).into()),
                },
                NumberWrapper::Rat(_) => Ok(self.pow(other.to_f64()).into()),
                NumberWrapper::Float(float_wrapper) => match float_wrapper {
                    SmallFloat(f) => Ok(self.powf(*f).into()),
                    BigFloat(float) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(float)).into())
                    }
                },
            },
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.abs() == 0.0 {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok((self / *i as f64).into()),
                IntegerWrapper::Int(integer) => Ok((self / integer.to_f64()).into()),
            },
            NumberWrapper::Rat(_) => Ok(self.div(other.to_f64()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok((self / *f).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self / float).into()),
            },
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.abs() == 0.0 {
            return Ok(IntegerWrapper::ZERO);
        }
        self.py_truediv(other)?.py_floor()
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(self.abs().into())
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.trunc()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot convert infinity to integer".into(),
            )),
        }
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(*self)
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        if self.is_infinite() {
            return Err(FirkinError::CannotConvertInfinity(
                "Cannot round infinity".into(),
            ));
        }
        if ndigits == 0 {
            Ok(self.round().into())
        } else if ndigits < 0 {
            let mul = 10.0_f64.powi(ndigits);
            Ok(IntegerWrapper::try_from(self.mul(mul).round().div(mul))
                .unwrap()
                .into())
        } else {
            let mul = 10.0_f64.powi(ndigits);
            Ok(self.mul(mul).round().div(mul).into())
        }
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.floor()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot floor infinity".into(),
            )),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.ceil()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot ceil infinity".into(),
            )),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match IntegerWrapper::try_from(self.trunc()) {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot trunc infinity".into(),
            )),
        }
    }
}

impl PythonMathFunctions for Float {
    fn py_mod(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => Ok(NumberWrapper::from(
                    FloatWrapper::from_incomplete_float(self.rem(i)),
                )),
                IntegerWrapper::Int(integer) => Ok(NumberWrapper::from(
                    self.rem(FloatWrapper::from(integer).to_float()),
                )),
            },
            NumberWrapper::Rat(_) => Ok(self.rem(other.to_float()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self.rem(f)).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self.rem(float)).into()),
            },
        }
    }

    fn py_pow(
        &self,
        other: &NumberWrapper,
        modulus: Option<&NumberWrapper>,
    ) -> Result<NumberWrapper, FirkinError> {
        match modulus {
            Some(m) => self.py_pow(other, None)?.py_mod(m),
            None => match other {
                NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                    IntegerWrapper::SmallInt(i) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(i)).into())
                    }
                    IntegerWrapper::Int(integer) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(integer)).into())
                    }
                },
                NumberWrapper::Rat(_) => Ok(self.pow(other.to_float()).into()),
                NumberWrapper::Float(float_wrapper) => match float_wrapper {
                    SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self.pow(f)).into()),
                    BigFloat(float) => {
                        Ok(FloatWrapper::from_incomplete_float(self.pow(float)).into())
                    }
                },
            },
        }
    }

    fn py_truediv(&self, other: &NumberWrapper) -> Result<NumberWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(NumberWrapper::ZERO);
        }
        match other {
            NumberWrapper::Int(integer_wrapper) => match integer_wrapper {
                IntegerWrapper::SmallInt(i) => {
                    Ok(FloatWrapper::from_incomplete_float(self / i).into())
                }
                IntegerWrapper::Int(integer) => {
                    Ok(FloatWrapper::from_incomplete_float(self / integer).into())
                }
            },
            NumberWrapper::Rat(_) => Ok(self.div(other.to_float()).into()),
            NumberWrapper::Float(float_wrapper) => match float_wrapper {
                SmallFloat(f) => Ok(FloatWrapper::from_incomplete_float(self / f).into()),
                BigFloat(float) => Ok(FloatWrapper::from_incomplete_float(self / float).into()),
            },
        }
    }

    fn py_floordiv(&self, other: &NumberWrapper) -> Result<IntegerWrapper, FirkinError> {
        if other.is_zero() {
            return Err(FirkinError::DivideByZero(
                self.to_string(),
                other.to_string(),
            ));
        } else if self.is_zero() {
            return Ok(IntegerWrapper::ZERO);
        }
        self.py_truediv(other)?.py_floor()
    }

    fn py_abs(&self) -> Result<NumberWrapper, FirkinError> {
        Ok(NumberWrapper::from(self.clone().abs()))
    }

    fn py_int(&self) -> Result<IntegerWrapper, FirkinError> {
        Ok(IntegerWrapper::from(
            match self.clone().trunc().to_integer() {
                Some(res) => res,
                None => {
                    return Err(FirkinError::CannotConvertInfinity(String::from(
                        "Cannot convert infinity to integer",
                    )));
                }
            },
        ))
    }

    fn py_float(&self) -> Result<f64, FirkinError> {
        Ok(self.to_f64())
    }

    fn py_round(&self, ndigits: i32) -> Result<NumberWrapper, FirkinError> {
        let mul = FloatWrapper::from(10).to_float().pow(ndigits);
        Ok(NumberWrapper::from(
            match FloatWrapper::from_incomplete_float(self * &mul).round() {
                Ok(res) => FloatWrapper::from(res).to_float() / mul,
                Err(_) => {
                    return Err(FirkinError::CannotConvertInfinity(
                        "Cannot round infinity".into(),
                    ));
                }
            },
        ))
    }

    fn py_floor(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().floor().try_into() {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot floor infinity".into(),
            )),
        }
    }

    fn py_ceil(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().ceil().try_into() {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot ceil infinity".into(),
            )),
        }
    }

    fn py_trunc(&self) -> Result<IntegerWrapper, FirkinError> {
        match self.clone().trunc().try_into() {
            Ok(res) => Ok(res),
            Err(_) => Err(FirkinError::CannotConvertInfinity(
                "Cannot trunc infinity".into(),
            )),
        }
    }
}
