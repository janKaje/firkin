#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

use Sign::{Negative, Positive};

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
