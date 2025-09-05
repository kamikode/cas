//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod expression;
pub(crate) mod fmt;
mod integer;
pub(crate) mod py;
mod symbol;

pub use expression::Expression;
pub use integer::Integer;
pub use symbol::Symbol;

#[cfg(test)]
mod tests {}
