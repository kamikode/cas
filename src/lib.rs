//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod ast;
pub(crate) mod fmt;
pub(crate) mod py;

pub use ast::Expression;

#[cfg(test)]
mod tests {}
