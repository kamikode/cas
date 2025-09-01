use pyo3::prelude::*;

#[pyclass]
struct TestClass;

#[pymethods]
impl TestClass {
    #[new]
    fn new() -> Self {
        TestClass
    }

    #[allow(clippy::unused_self)]
    fn _repr_latex_(&self) -> String {
        "$x^2 + y^2 + z^2$".to_string()
    }
}

#[pymodule]
mod cas {
    #[pymodule_export]
    use super::TestClass;
}
