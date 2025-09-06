//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod fmt;
mod number;
pub(crate) mod py;
mod symbol;
pub(crate) mod term;

pub use term::Term;

#[cfg(test)]
mod tests {}
