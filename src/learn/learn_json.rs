use std::collections::HashMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use log::{debug, error, info, warn};
use pyo3_log;

// Json
#[pyfunction]
pub fn human_say_hi(human_data: String) {
    println!("{}", human_data);
    let human: Human = serde_json::from_str(&human_data).unwrap();

    println!(
        "Now we can work with the struct:\n {:#?}.\n {} is {} years old.",
        human, human.name, human.age,
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct Human {
    name: String,
    age: u8,
}

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
pub fn learn_json(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(human_say_hi, m)?)?;

    Ok(())
}
