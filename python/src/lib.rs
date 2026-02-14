use pyo3::prelude::*;
use filigineacht_rs::infer_network;

#[pyfunction]
fn infer(input: &str) -> String {
    let network = infer_network(input);
    network.to_string()
}

#[pymodule]
fn physquirrel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(infer, m)?)?;
    Ok(())
}