//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod expression;
pub(crate) mod fmt;
mod number;
pub(crate) mod py;
mod symbol;

pub use expression::Expression;

#[cfg(test)]
mod tests {}
