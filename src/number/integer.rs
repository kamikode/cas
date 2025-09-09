use delegate::delegate;
use derive_more::Display;

#[cfg(feature = "python")]
use num_bigint::BigInt;

/// An (arbitrarily large) integer.
#[derive(Debug, Display, Clone, PartialEq)]
pub(super) struct Integer(rug::Integer);

impl Integer {
    pub const ZERO: Self = Self(rug::Integer::ZERO);

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

macro_rules! impl_partial_eq {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for Integer {
                fn eq(&self, other: &$t) -> bool {
                    self.0 == *other
                }
            }

            impl PartialEq<Integer> for $t {
                fn eq(&self, other: &Integer) -> bool {
                    *self == other.0
                }
            }
        )*
    };
}
impl_partial_eq!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

#[cfg(feature = "python")]
impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        // TODO: Make this conversion more efficient.
        Self(rug::Integer::from_str_radix(&value.to_str_radix(36), 36).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_implementations_work() {
        assert_eq!(Integer::from(false), Integer::ZERO);
        assert_eq!(Integer::from(0i8), Integer::ZERO);
        assert_eq!(Integer::from(0i16), Integer::ZERO);
        assert_eq!(Integer::from(0i32), Integer::ZERO);
        assert_eq!(Integer::from(0i64), Integer::ZERO);
        assert_eq!(Integer::from(0isize), Integer::ZERO);
        assert_eq!(Integer::from(0u8), Integer::ZERO);
        assert_eq!(Integer::from(0u16), Integer::ZERO);
        assert_eq!(Integer::from(0u32), Integer::ZERO);
        assert_eq!(Integer::from(0u64), Integer::ZERO);
        assert_eq!(Integer::from(0usize), Integer::ZERO);
    }

    #[test]
    fn partial_eq_lhs_implementations_work() {
        assert!(Integer::ZERO == 0i8);
        assert!(Integer::ZERO == 0i16);
        assert!(Integer::ZERO == 0i32);
        assert!(Integer::ZERO == 0i64);
        assert!(Integer::ZERO == 0isize);
        assert!(Integer::ZERO == 0u8);
        assert!(Integer::ZERO == 0u16);
        assert!(Integer::ZERO == 0u32);
        assert!(Integer::ZERO == 0u64);
        assert!(Integer::ZERO == 0usize);
    }

    #[test]
    fn partial_eq_rhs_implementations_work() {
        assert!(0i8 == Integer::ZERO);
        assert!(0i16 == Integer::ZERO);
        assert!(0i32 == Integer::ZERO);
        assert!(0i64 == Integer::ZERO);
        assert!(0isize == Integer::ZERO);
        assert!(0u8 == Integer::ZERO);
        assert!(0u16 == Integer::ZERO);
        assert!(0u32 == Integer::ZERO);
        assert!(0u64 == Integer::ZERO);
        assert!(0usize == Integer::ZERO);
    }

    #[test]
    #[cfg(feature = "python")]
    fn from_bigint_works() {
        // The value -2^1024 in base 10.
        const BIG_INT_STR: &str = "-179769313486231590772930519078902473361797697894230657273430081\
            157732675805500963132708477322407536021120113879871393357658789768814416622492847430639\
            474124377767893424865485276302219601246094119453082952085005768838150682342462881473913\
            110540827237163350510684586298239947245938479716304835356329624224137216";
        let x = BigInt::parse_bytes(BIG_INT_STR.as_bytes(), 10).unwrap();
        assert_eq!(
            Integer::from(x).0,
            rug::Integer::from_str_radix(BIG_INT_STR, 10).unwrap()
        );
    }
}
