use core::fmt;

/// A trait for types that can be represented as LaTeX.
/// This is analogous to `std::fmt::Display`.
pub trait Latex {
    /// Formats the value as LaTeX using the given formatter.
    fn latex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// A helper trait to convert a type to a LaTeX string (analogous to `ToString`).
#[allow(unused)] // TODO: Remove this at some point.
pub trait ToLatex {
    /// Converts a type to a LaTeX string.
    fn to_latex(&self) -> String;
}

// Helper struct to bridge `Latex` and `Display`.
struct LatexWrapper<'a, T: 'a>(&'a T);

impl<T: Latex> fmt::Display for LatexWrapper<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.latex(f)
    }
}

/// Blanket implementation to provide `.to_latex()` for any type that implements `Latex`.
impl<T: Latex> ToLatex for T {
    fn to_latex(&self) -> String {
        format!("{}", LatexWrapper(self))
    }
}
