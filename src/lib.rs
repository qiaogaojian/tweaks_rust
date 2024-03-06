use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use serde::{Deserialize, Serialize};
use pyo3_log;

mod learn;

use learn::hello_world::hello_world;
use learn::learn_json::learn_json;
use learn::learn_logger::learn_logger;
use learn::learn_struct::learn_struct;

#[pymodule]
fn rustcore(_py: Python, m: &PyModule) -> PyResult<()> {
    // 添加子模块到主模块
    m.add_wrapped(wrap_pymodule!(hello_world))?;
    m.add_wrapped(wrap_pymodule!(learn_json))?;
    m.add_wrapped(wrap_pymodule!(learn_logger))?;
    m.add_wrapped(wrap_pymodule!(learn_struct))?;

    Ok(())
}