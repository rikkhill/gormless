use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod common;

#[pyfunction]
fn greet() -> &'static str {
    common::something_common()
}

#[pymodule]
fn gormless_core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}