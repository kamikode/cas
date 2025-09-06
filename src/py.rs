//! Python bindings.

use pyo3::prelude::*;

mod term;

/// This module is implemented in Rust.
#[pymodule]
fn cas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<term::PyTerm>()?;
    m.add_function(wrap_pyfunction!(term::variable, m)?)
}

#[cfg(test)]
mod tests {}
