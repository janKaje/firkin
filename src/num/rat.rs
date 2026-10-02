use rug::Rational;
use crate::num::smallrational::SmallRational;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

/// Wraps around the logic for both small and large rationals
#[derive(Debug)]
pub(crate) enum RationalWrapper {
    SmallRat(SmallRational),
    Rat(Rational),
}

use RationalWrapper::{SmallRat, Rat};

macro_rules! arith_binary {
    (
        $checked_func:ident;
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {

        impl $Imp<RationalWrapper> for RationalWrapper {
            type Output = RationalWrapper;
            #[inline]
            fn $method(mut self, mut rhs: RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(i), SmallRat(i2)) => {
                        match i.$checked_func(i2) {
                            Some(r) => SmallRat(r),
                            None => Rat(Integer::from(i).$method(i2)),
                        }
                    },
                    (SmallRat(i), Rat(integer)) => {
                        RationalWrapper::from(i.$method(integer))
                    },
                    (Rat(integer), SmallRat(i)) => {
                        RationalWrapper::from(integer.$method(i))
                    },
                    (Rat(integer), Rat(integer2)) => {
                        RationalWrapper::from(integer.$method(integer2))
                    },
                }
            }
        }

        impl $Imp<&RationalWrapper> for RationalWrapper {
            type Output = RationalWrapper;
            #[inline]
            fn $method(mut self, rhs: &RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(i), SmallRat(i2)) => {
                        match i.$checked_func(*i2) {
                            Some(r) => SmallRat(r),
                            None => Rat(Integer::from(i).$method(i2)),
                        }
                    },
                    (SmallRat(i), Rat(integer)) => {
                        RationalWrapper::from(Integer::from(i.$method(integer)))
                    },
                    (Rat(integer), SmallRat(i)) => {
                        RationalWrapper::from(integer.$method(i))
                    },
                    (Rat(integer), Rat(integer2)) => {
                        RationalWrapper::from(integer.$method(integer2))
                    },
                }
            }
        }

        impl $Imp<RationalWrapper> for &RationalWrapper {
            type Output = RationalWrapper;
            #[inline]
            fn $method(self, mut rhs: RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(i), SmallRat(i2)) => {
                        match i.$checked_func(i2) {
                            Some(r) => SmallRat(r),
                            None => Rat(Integer::from(*i).$method(i2)),
                        }
                    },
                    (SmallRat(i), Rat(integer)) => {
                        RationalWrapper::from(i.$method(integer))
                    },
                    (Rat(integer), SmallRat(i)) => {
                        RationalWrapper::from(Integer::from(integer.$method(i)))
                    },
                    (Rat(integer), Rat(integer2)) => {
                        RationalWrapper::from(Integer::from(integer.$method(integer2)))
                    },
                }
            }
        }

        impl $Imp<&RationalWrapper> for &RationalWrapper {
            type Output = RationalWrapper;
            #[inline]
            fn $method(self, rhs: &RationalWrapper) -> RationalWrapper {
                match (self, rhs) {
                    (SmallRat(i), SmallRat(i2)) => {
                        match i.$checked_func(*i2) {
                            Some(r) => SmallRat(r),
                            None => Rat(Integer::from(*i).$method(i2)),
                        }
                    },
                    (SmallRat(i), Rat(integer)) => {
                        RationalWrapper::from(Integer::from(integer.$method(i)))
                    },
                    (Rat(integer), SmallRat(i)) => {
                        RationalWrapper::from(Integer::from(integer.$method(i)))
                    },
                    (Rat(integer), Rat(integer2)) => {
                        RationalWrapper::from(Integer::from(integer.$method(integer2)))
                    },
                }
            }
        } 

        impl $ImpAssign<RationalWrapper> for RationalWrapper {
            #[inline]
            fn $method_assign(&mut self, rhs: RationalWrapper) {
                self.$method_assign(&rhs);
            }
        }

        impl $ImpAssign<&RationalWrapper> for RationalWrapper {
            #[inline]
            fn $method_assign(&mut self, rhs: &RationalWrapper) {
                match (&self, rhs) {
                    (SmallRat(i), SmallRat(i2)) => {
                        match i.$checked_func(*i2) {
                            Some(r) => *self = SmallRat(r),
                            None => *self = Rat(Integer::from(*i).$method(i2)),
                        }
                    },
                    (SmallRat(i), Rat(integer)) => {
                        *self = RationalWrapper::from(Integer::from(integer.$method(i)))
                    },
                    (Rat(integer), SmallRat(i)) => {
                        *self = RationalWrapper::from(Integer::from(i.$method(integer)))
                    },
                    (Rat(integer), Rat(integer2)) => {
                        *self = RationalWrapper::from(Integer::from(integer2.$method(integer)))
                    },
                }
            }
        }
    };
}

impl From<SmallRational> for RationalWrapper {
    fn from(value: SmallRational) -> Self {
        SmallRat(value)
    }
}

impl From<Rational> for RationalWrapper {
    fn from(value: Rational) -> Self {
        Rat(value)
    }
}

impl From<isize> for RationalWrapper {
    fn from(value: isize) -> Self {
        todo!()
    }
}