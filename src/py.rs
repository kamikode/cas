//! Python bindings.

use pyo3::prelude::*;

mod expression;

/// This module is implemented in Rust.
#[pymodule]
fn cas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<expression::PyExpression>()?;
    m.add_function(wrap_pyfunction!(expression::variable, m)?)
}

#[cfg(test)]
mod tests {}
