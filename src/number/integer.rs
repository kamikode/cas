use delegate::delegate;
use derive_more::Display;
use num_bigint::BigInt;

/// An (arbitrarily large) integer.
#[derive(Debug, Display, Clone, PartialEq)]
pub struct Integer(rug::Integer);

impl Integer {
    pub const ZERO: Self = Self(rug::Integer::ZERO);

    pub const fn zero() -> Self {
        Self::ZERO
    }

    delegate! {
        to self.0 {
            pub fn is_zero(&self) -> bool;
        }
    }
}

/// Helper macro to implement `From` for `Integer` by delegating to `rug::Integer`.
macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer {
                fn from(value: $t) -> Self {
                    Self(value.into())
                }
            }
        )*
    };
}
impl_from!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        // TODO: Make this conversion more efficient.
        Self(rug::Integer::from_str_radix(&value.to_str_radix(36), 36).unwrap())
    }
}

impl From<Integer> for BigInt {
    fn from(value: Integer) -> Self {
        // TODO: Make this conversion more efficient.
        BigInt::parse_bytes(value.0.to_string_radix(36).as_bytes(), 36).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The value -2^1024 in base 10 for testing purposes.
    const BIG_INT_STR: &str = "-1797693134862315907729305190789024733617976978942306572734300811577\
        3267580550096313270847732240753602112011387987139335765878976881441662249284743063947412437\
        7767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237\
        163350510684586298239947245938479716304835356329624224137216";

    #[test]
    fn from_bigint_to_integer_works() {
        let x = BigInt::parse_bytes(BIG_INT_STR.as_bytes(), 10).unwrap();
        assert_eq!(
            Integer::from(x).0,
            rug::Integer::from_str_radix(BIG_INT_STR, 10).unwrap()
        );
    }

    #[test]
    fn from_integer_to_bigint_works() {
        let x = Integer(rug::Integer::from_str_radix(BIG_INT_STR, 10).unwrap());
        assert_eq!(
            BigInt::from(x),
            BigInt::parse_bytes(BIG_INT_STR.as_bytes(), 10).unwrap()
        );
    }
}
