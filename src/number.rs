use core::fmt;
mod integer;
use derive_more::Display;

#[cfg(feature = "python")]
use num_bigint::BigInt;

use integer::Integer;

#[derive(Debug, Display, Clone, PartialEq)]
enum NumberComponent {
    Integer(Integer),
}

/// A number.
#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    real: NumberComponent,
    imag: NumberComponent,
}

impl Number {
    pub const ZERO: Self = Self {
        real: NumberComponent::Integer(Integer::ZERO),
        imag: NumberComponent::Integer(Integer::ZERO),
    };

    pub const fn zero() -> Self {
        Self::ZERO
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Number {
                real: NumberComponent::Integer(r),
                imag: NumberComponent::Integer(i),
            } => r.is_zero() && i.is_zero(),
        }
    }
}

/// Helper macro to implement `From` for `Number` by delegating to `Integer`.
macro_rules! impl_from_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Number {
                fn from(value: $t) -> Self {
                    Self {
                        real: NumberComponent::Integer(value.into()),
                        imag: NumberComponent::Integer(Integer::ZERO),
                    }
                }
            }
        )*
    };
}
impl_from_integer!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

#[cfg(feature = "python")]
impl From<BigInt> for Number {
    fn from(value: BigInt) -> Self {
        Self {
            real: NumberComponent::Integer(Integer::from(value)),
            imag: NumberComponent::Integer(Integer::ZERO),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number {
                real: NumberComponent::Integer(r),
                imag: NumberComponent::Integer(i),
            } => {
                if i.is_zero() {
                    write!(f, "{r}")
                } else if r.is_zero() {
                    write!(f, "{i}i")
                } else {
                    write!(f, "{r} + {i}i")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_integer_implementations_work() {
        assert_eq!(Number::from(false), Number::ZERO);
        assert_eq!(Number::from(0i8), Number::ZERO);
        assert_eq!(Number::from(0i16), Number::ZERO);
        assert_eq!(Number::from(0i32), Number::ZERO);
        assert_eq!(Number::from(0i64), Number::ZERO);
        assert_eq!(Number::from(0isize), Number::ZERO);
        assert_eq!(Number::from(0u8), Number::ZERO);
        assert_eq!(Number::from(0u16), Number::ZERO);
        assert_eq!(Number::from(0u32), Number::ZERO);
        assert_eq!(Number::from(0u64), Number::ZERO);
        assert_eq!(Number::from(0usize), Number::ZERO);
    }

    #[test]
    fn display_works() {
        assert_eq!(Number::ZERO.to_string(), "0");
        assert_eq!(
            Number {
                real: NumberComponent::Integer(Integer::ZERO),
                imag: NumberComponent::Integer(2.into()),
            }
            .to_string(),
            "i"
        );
        assert_eq!(
            Number {
                real: NumberComponent::Integer(Integer::ZERO),
                imag: NumberComponent::Integer(2.into()),
            }
            .to_string(),
            "2i"
        );
        assert_eq!(
            Number {
                real: NumberComponent::Integer(3.into()),
                imag: NumberComponent::Integer(2.into()),
            }
            .to_string(),
            "3 + 2i"
        );
    }

    #[test]
    #[cfg(feature = "python")]
    fn from_bigint_works() {
        assert_eq!(Number::from(BigInt::ZERO), Number::ZERO);
    }
}
