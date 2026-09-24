use rug::Integer;

/// Wraps around the logic for both small and large integers
pub(crate) enum IntegerWrapper {
    SmallInt(i64),
    Int(Integer),
}

use IntegerWrapper::{SmallInt, Int};

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