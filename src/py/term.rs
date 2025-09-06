//! Expression.
use crate::Term;
use crate::fmt::ToLatex;
use crate::symbol::Symbol;
use num_bigint::BigInt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(name = "Term", module = "cas")]
#[derive(Debug)]
pub struct PyTerm(pub Term);

#[derive(FromPyObject)]
enum PyTermOrNumber<'py> {
    PyExpression(PyRef<'py, PyTerm>),
    Int(i64),
    BigInt(BigInt),
}

impl From<PyTermOrNumber<'_>> for Term {
    fn from(other: PyTermOrNumber<'_>) -> Term {
        match other {
            PyTermOrNumber::PyExpression(e) => e.0.clone(),
            PyTermOrNumber::Int(i) => Term::Constant(i.into()),
            PyTermOrNumber::BigInt(i) => Term::Constant(i.into()),
        }
    }
}

#[pymethods]
impl PyTerm {
    fn __add__(&self, other: PyTermOrNumber) -> PyTerm {
        PyTerm(self.0.clone() + other.into())
    }

    fn __radd__(&self, other: PyTermOrNumber) -> PyTerm {
        PyTerm(Term::from(other) + self.0.clone())
    }

    fn _repr_latex_(&self) -> String {
        format!("${}$", self.0.to_latex())
    }
}

/// Creates a symbol.
#[pyfunction(name = "Variable")]
pub(in crate::py) fn variable(name: String) -> PyResult<PyTerm> {
    match Symbol::try_from(name) {
        Ok(s) => Ok(PyTerm(Term::Variable(s))),
        Err(e) => Err(PyErr::new::<PyValueError, String>(e)),
    }
}
