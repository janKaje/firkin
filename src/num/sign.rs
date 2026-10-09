#[derive(Clone, Eq, PartialEq, Copy, Hash)]
pub enum Sign {
    Positive,
    Negative,
}

use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use Sign::{Negative, Positive};

impl From<Sign> for u8 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => 0,
        }
    }
}

impl From<&Sign> for u8 {
    fn from(value: &Sign) -> Self {
        match value {
            Positive => 1,
            Negative => 0,
        }
    }
}

impl From<u8> for Sign {
    fn from(value: u8) -> Self {
        if value == 0 { Negative } else { Positive }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Negative {
            write!(f, "-")?;
        }
        Ok(())
    }
}

impl From<Sign> for f64 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1.0,
            Negative => -1.0,
        }
    }
}

impl From<Sign> for f32 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1.0,
            Negative => -1.0,
        }
    }
}

impl From<Sign> for i8 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

impl From<Sign> for i16 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

impl From<Sign> for i32 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

impl From<Sign> for i64 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

impl From<Sign> for i128 {
    fn from(value: Sign) -> Self {
        match value {
            Positive => 1,
            Negative => -1,
        }
    }
}

impl Not for Sign {
    type Output = Sign;

    fn not(self) -> Self::Output {
        if self == Positive { Negative } else { Positive }
    }
}

macro_rules! bitwise_binary {
    (
        $Imp:ident { $method:ident }
        $ImpAssign:ident { $method_assign:ident }
    ) => {
        impl $Imp<Sign> for Sign {
            type Output = Sign;

            fn $method(self, rhs: Sign) -> Self::Output {
                (&self).$method(&rhs)
            }
        }

        impl $ImpAssign<Sign> for Sign {
            fn $method_assign(&mut self, rhs: Sign) {
                *self = (*self).$method(rhs)
            }
        }

        impl $Imp<&Sign> for Sign {
            type Output = Sign;

            fn $method(self, rhs: &Sign) -> Self::Output {
                (&self).$method(rhs)
            }
        }

        impl $ImpAssign<&Sign> for Sign {
            fn $method_assign(&mut self, rhs: &Sign) {
                *self = (*self).$method(rhs)
            }
        }

        impl $Imp<Sign> for &Sign {
            type Output = Sign;

            fn $method(self, rhs: Sign) -> Self::Output {
                self.$method(&rhs)
            }
        }

        impl $Imp<&Sign> for &Sign {
            type Output = Sign;

            fn $method(self, rhs: &Sign) -> Self::Output {
                u8::from(self).$method(u8::from(rhs)).into()
            }
        }
    };
}

bitwise_binary! {
    BitAnd {bitand}
    BitAndAssign {bitand_assign}
}
bitwise_binary! {
    BitOr {bitor}
    BitOrAssign {bitor_assign}
}
bitwise_binary! {
    BitXor {bitxor}
    BitXorAssign {bitxor_assign}
}
