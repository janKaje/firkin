use rug::Integer;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

/// Wraps around the logic for both small and large integers
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum IntegerWrapper {
    SmallInt(i64),
    Int(Integer),
}

use IntegerWrapper::{Int, SmallInt};

impl IntegerWrapper {
    pub(crate) fn as_i64(&self) -> i64 {
        match self {
            SmallInt(i) => *i,
            Int(i) => i.to_i64_wrapping(),
        }
    }

    pub(crate) fn as_integer(&self) -> Integer {
        match self {
            SmallInt(i) => (*i).into(),
            Int(i) => i.clone(),
        }
    }
}

macro_rules! arith_binary {
    (
        $checked_func:ident;
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(mut self, mut rhs: IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(mut self, rhs: &IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.$method(i)),
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(integer.$method(integer2))
                    }
                }
            }
        }

        impl $Imp<IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, mut rhs: IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.$method(integer)),
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(integer2)))
                    }
                }
            }
        }

        impl $Imp<&IntegerWrapper> for &IntegerWrapper {
            type Output = IntegerWrapper;

            fn $method(self, rhs: &IntegerWrapper) -> IntegerWrapper {
                match (self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => SmallInt(r),
                        None => Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), SmallInt(i)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), Int(integer2)) => {
                        IntegerWrapper::from(Integer::from(integer.$method(integer2)))
                    }
                }
            }
        }

        impl $ImpAssign<IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: IntegerWrapper) {
                self.$method_assign(&rhs);
            }
        }

        impl $ImpAssign<&IntegerWrapper> for IntegerWrapper {
            fn $method_assign(&mut self, rhs: &IntegerWrapper) {
                match (&self, rhs) {
                    (SmallInt(i), SmallInt(i2)) => match i.$checked_func(*i2) {
                        Some(r) => *self = SmallInt(r),
                        None => *self = Int(Integer::from(*i).$method(i2)),
                    },
                    (SmallInt(i), Int(integer)) => {
                        *self = IntegerWrapper::from(Integer::from(integer.$method(i)))
                    }
                    (Int(integer), SmallInt(i)) => {
                        *self = IntegerWrapper::from(Integer::from(i.$method(integer)))
                    }
                    (Int(integer), Int(integer2)) => {
                        *self = IntegerWrapper::from(Integer::from(integer2.$method(integer)))
                    }
                }
            }
        }
    };
}

arith_binary! {
    checked_add;
    Add { add }
    AddAssign { add_assign }
}

arith_binary! {
    checked_sub;
    Sub { sub }
    SubAssign { sub_assign }
}

arith_binary! {
    checked_mul;
    Mul { mul }
    MulAssign { mul_assign }
}

arith_binary! {
    checked_div;
    Div { div }
    DivAssign { div_assign }
}

impl Neg for IntegerWrapper {
    type Output = IntegerWrapper;

    fn neg(self) -> Self::Output {
        match self {
            SmallInt(i) => IntegerWrapper::from(-i),
            Int(integer) => IntegerWrapper::from(-integer),
        }
    }
}

impl Neg for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn neg(self) -> Self::Output {
        match self {
            SmallInt(i) => IntegerWrapper::from(-i),
            Int(integer) => IntegerWrapper::from(Integer::from(-integer)),
        }
    }
}

impl Rem<IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(i.rem(integer)),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.rem(i)),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl Rem<&IntegerWrapper> for IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: &IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(integer.rem(i)),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl Rem<IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(Integer::from(integer.rem(i))),
            (Int(integer), Int(integer1)) => IntegerWrapper::from(integer.rem(integer1)),
        }
    }
}

impl Rem<&IntegerWrapper> for &IntegerWrapper {
    type Output = IntegerWrapper;

    fn rem(self, rhs: &IntegerWrapper) -> Self::Output {
        match (self, rhs) {
            (SmallInt(i), SmallInt(i2)) => IntegerWrapper::from(i.rem(i2)),
            (SmallInt(i), Int(integer)) => IntegerWrapper::from(Integer::from(i.rem(integer))),
            (Int(integer), SmallInt(i)) => IntegerWrapper::from(Integer::from(integer.rem(i))),
            (Int(integer), Int(integer1)) => {
                IntegerWrapper::from(Integer::from(integer.rem(integer1)))
            }
        }
    }
}

impl PartialOrd for IntegerWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SmallInt(i), SmallInt(i2)) => i.partial_cmp(i2),
            _ => self.as_integer().partial_cmp(&other.as_integer()),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_lt)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_le)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_gt)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_ge)
    }
}

impl Ord for IntegerWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SmallInt(i), SmallInt(i2)) => i.cmp(i2),
            _ => self.as_integer().cmp(&other.as_integer()),
        }
    }
}

impl From<i128> for IntegerWrapper {
    fn from(value: i128) -> Self {
        if value > i64::MAX as i128 || value < i64::MIN as i128 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<u128> for IntegerWrapper {
    fn from(value: u128) -> Self {
        if value > i64::MAX as u128 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<u64> for IntegerWrapper {
    fn from(value: u64) -> Self {
        if value > i64::MAX as u64 {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<usize> for IntegerWrapper {
    fn from(value: usize) -> Self {
        if value > i64::MAX as usize {
            Int(value.into())
        } else {
            SmallInt(value as i64)
        }
    }
}

impl From<Integer> for IntegerWrapper {
    fn from(value: Integer) -> Self {
        if value > Integer::from(i64::MAX) || value < Integer::from(i64::MIN) {
            Int(value.into())
        } else {
            SmallInt(value.to_i64_wrapping())
        }
    }
}

impl From<i8> for IntegerWrapper {
    fn from(value: i8) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i16> for IntegerWrapper {
    fn from(value: i16) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i32> for IntegerWrapper {
    fn from(value: i32) -> Self {
        SmallInt(value as i64)
    }
}

impl From<i64> for IntegerWrapper {
    fn from(value: i64) -> Self {
        SmallInt(value)
    }
}

impl From<isize> for IntegerWrapper {
    fn from(value: isize) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u8> for IntegerWrapper {
    fn from(value: u8) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u16> for IntegerWrapper {
    fn from(value: u16) -> Self {
        SmallInt(value as i64)
    }
}

impl From<u32> for IntegerWrapper {
    fn from(value: u32) -> Self {
        SmallInt(value as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::num::int::IntegerWrapper;
    use rug::Integer;

    #[test]
    fn wrapper_arith() {
        let small1 = IntegerWrapper::from(2);
        let big1 = IntegerWrapper::from(Integer::from(i64::MAX) * 2);
        assert_eq!(&small1 + &small1, IntegerWrapper::from(4));
        assert_eq!(&big1 - &big1, IntegerWrapper::from(0));
        assert_eq!(
            &big1 * &small1,
            IntegerWrapper::from(36893488147419103228_i128)
        );
        assert_eq!(&small1 / &big1, IntegerWrapper::from(i64::MAX));
        assert_eq!(&big1 % &small1, IntegerWrapper::from(0));
    }
}
