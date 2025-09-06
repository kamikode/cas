use crate::number::Number;
use crate::symbol::Symbol;
use std::collections::VecDeque;

/// Implementation of traits for printing.
mod fmt;
/// Implementation of operator traits.
mod ops;

/// A mathematical term.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A constant value specified by a number.
    Constant(Number),
    /// A variable specified by a symbol, like 'x'.
    Variable(Symbol),
    /// Unary negation operator (additive inverse).
    Neg(Box<Term>),
    /// A sum over subexpressions (n-ary addition).
    Sum(VecDeque<Term>),
    // TODO: Product, Power, etc. Note that Power can also be used for representing
    // division/reciprocals, so this does not need to be an extra case.
}

#[cfg(test)]
mod tests {}
