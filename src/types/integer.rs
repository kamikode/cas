use core::fmt;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct Integer(rug::Integer);

impl From<i64> for Integer {
    #[inline]
    fn from(value: i64) -> Self {
        Integer(rug::Integer::from(value))
    }
}

impl From<BigInt> for Integer {
    #[inline]
    fn from(value: BigInt) -> Self {
        // TODO: Make this conversion more efficient.
        Self(rug::Integer::from_str_radix(&value.to_str_radix(36), 36).unwrap())
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bigint_works() {
        let value = BigInt::from(100);
        let integer = Integer::from(value);
        assert_eq!(integer.0, rug::Integer::from(100));
    }
}
