use crate::rs::ast::Expression;
use pyo3::prelude::*;

/// A mathematical expression.
#[pyclass(name = "Expression", module = "cas")]
#[derive(Debug, Clone)]
pub(in crate::py) struct PyExpression(Expression);

#[pymethods]
impl PyExpression {
    fn __add__(&self, other: &PyExpression) -> PyExpression {
        PyExpression(Expression::Add(
            Box::new(self.0.clone()),
            Box::new(other.0.clone()),
        ))
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0)
    }
}

/// Creates a symbol.
#[pyfunction(name = "Symbol")]
pub(in crate::py) fn symbol(name: String) -> PyExpression {
    PyExpression(Expression::Symbol(name))
}
