use std::collections::HashMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use log::{debug, error, info, warn};
use pyo3_log;

// Logger
#[pyfunction]
pub fn log_different_levels() {
    info!("logging an info message");
    warn!("logging a warning");
    debug!("logging a debug message");
    error!("logging an error");
}

#[pyfunction]
pub fn log_example() {
    info!("A log message from {}!", "Rust");
}

#[pymodule]
pub fn learn_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_wrapped(wrap_pyfunction!(log_example))?;
    m.add_wrapped(wrap_pyfunction!(log_different_levels))?;

    Ok(())
}
