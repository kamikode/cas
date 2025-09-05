//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod expression;
pub(crate) mod fmt;
pub(crate) mod py;
pub(crate) mod types;

pub use expression::Expression;

#[cfg(test)]
mod tests {}
