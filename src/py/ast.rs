//! Abstract Syntax Tree.
use crate::ast::Expression;
use crate::fmt::ToLatex;
use crate::types::Symbol;
use num_bigint::BigInt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "Expression", module = "cas")]
#[derive(Debug)]
pub struct PyExpression(pub Expression);

#[derive(FromPyObject)]
enum PyExpressionOrNumber<'py> {
    PyExpression(PyRef<'py, PyExpression>),
    Int(i64),
    BigInt(BigInt),
}

impl From<PyExpressionOrNumber<'_>> for Expression {
    fn from(other: PyExpressionOrNumber<'_>) -> Expression {
        match other {
            PyExpressionOrNumber::PyExpression(e) => e.0.clone(),
            PyExpressionOrNumber::Int(i) => Expression::Integer(i.into()),
            PyExpressionOrNumber::BigInt(i) => Expression::Integer(i.into()),
        }
    }
}

#[pymethods]
impl PyExpression {
    fn __add__(&self, other: PyExpressionOrNumber) -> PyExpression {
        PyExpression(Expression::Add(
            Box::new(self.0.clone()),
            Box::new(other.into()),
        ))
    }

    fn __radd__(&self, other: PyExpressionOrNumber) -> PyExpression {
        PyExpression(Expression::Add(
            Box::new(other.into()),
            Box::new(self.0.clone()),
        ))
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
    }
}

/// Creates a symbol.
#[pyfunction(name = "Variable")]
pub(in crate::py) fn variable(name: String) -> PyResult<PyExpression> {
    match Symbol::try_from(name) {
        Ok(s) => Ok(PyExpression(Expression::Variable(s))),
        Err(e) => Err(PyErr::new::<PyValueError, String>(e)),
    }
}
