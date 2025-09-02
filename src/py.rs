//! Python bindings.

use pyo3::prelude::*;

mod ast;

/// This module is implemented in Rust.
#[pymodule]
fn cas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ast::PyExpression>()?;
    m.add_function(wrap_pyfunction!(ast::symbol, m)?)
}

#[cfg(test)]
mod tests {}
