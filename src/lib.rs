use pyo3::prelude::*;

#[pyfunction]
fn say_hello_to_python() -> PyResult<String> {
    Ok("Hello, Python!".to_string())
}

#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[pymodule]
fn rustcore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}