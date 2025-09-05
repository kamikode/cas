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

impl From<Integer> for BigInt {
    #[inline]
    fn from(value: Integer) -> Self {
        // TODO: Make this conversion more efficient.
        BigInt::parse_bytes(value.0.to_string_radix(36).as_bytes(), 36).unwrap()
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
        let big_int = BigInt::from(100);
        let integer = Integer::from(big_int);
        assert_eq!(integer.0, rug::Integer::from(100));
    }

    #[test]
    fn from_integer_works() {
        let integer = Integer::from(100);
        let big_int = BigInt::from(integer);
        assert_eq!(big_int, BigInt::from(100));
    }
}
