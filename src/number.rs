use core::fmt;
mod integer;
use derive_more::Display;
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

impl NumberComponent {
    pub fn is_zero(&self) -> bool {
        match self {
            NumberComponent::Integer(i) => i.is_zero(),
        }
    }
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
        self.real.is_zero() && self.imag.is_zero()
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self {
            real: NumberComponent::Integer(value.into()),
            imag: NumberComponent::Integer(Integer::ZERO),
        }
    }
}

impl From<BigInt> for NumberComponent {
    fn from(value: BigInt) -> Self {
        NumberComponent::Integer(Integer::from(value))
    }
}

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
        let Number { real, imag } = self;
        if imag.is_zero() {
            write!(f, "{real}")
        } else if real.is_zero() {
            write!(f, "{imag}i")
        } else {
            write!(f, "{real} + {imag}i")
        }
    }
}
