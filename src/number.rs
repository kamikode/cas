use core::fmt;
mod integer;
use num_bigint::BigInt;

pub use integer::Integer;

/// A number.
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(Integer),
}

impl Number {
    pub fn zero() -> Self {
        Number::Integer(Integer::from(0))
    }
}

impl From<i64> for Number {
    #[inline]
    fn from(value: i64) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<BigInt> for Number {
    #[inline]
    fn from(value: BigInt) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{i}"),
        }
    }
}
