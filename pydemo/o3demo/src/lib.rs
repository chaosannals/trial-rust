use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b:usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn o3demo_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}