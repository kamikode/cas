//! Computer Algebra System.
#![warn(missing_debug_implementations, missing_docs, clippy::pedantic)]
pub(crate) mod fmt;
mod number;
mod symbol;
pub(crate) mod term;

#[cfg(feature = "python")]
mod py;

pub use term::Term;

#[cfg(test)]
mod tests {}
