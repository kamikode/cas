//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod py;
pub(crate) mod rs;

pub use rs::ast::Expression;

#[cfg(test)]
mod tests {}
