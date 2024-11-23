use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

mod common;
mod system_queries;

#[pyfunction]
fn has_gpus(_py: Python) -> PyResult<bool> {
    match system_queries::gpus::has_gpus() {
        Ok(result) => Ok(result),
        Err(e) => Err(PyErr::new::<PyException, _>(format!("Error: {}", e))),
    }
}

#[pyfunction]
fn get_gpu_devices(py: Python) -> PyResult<PyObject> {
    match system_queries::gpus::get_gpu_devices() {
        Ok(devices) => {
            // Convert Vec<(String, String)> to Python list
            let py_devices = PyList::new(py, devices.iter().map(|(name, vendor)| {
                (name.clone(), vendor.clone())
            }));
            Ok(py_devices.into())
        },
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))),
    }
}

#[pyfunction]
fn greet() -> &'static str {
    common::something_common()
}

#[pymodule]
fn gormless_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(get_gpu_devices, m)?)?;
    m.add_function(wrap_pyfunction!(has_gpus, m)?)?;
    Ok(())
}