use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3_log;

#[pyclass] // above the struct definition, used to expose the class in Python.
pub struct RustStruct {
    #[pyo3(get, set)] // use these macros in case you want to be able to get or set the struct fields in Python.
    pub data: String,
    #[pyo3(get, set)]
    pub vector: Vec<u8>,
}

#[pymethods] // above the impl block, used to expose the struct methods in Python as class methods.
impl RustStruct {
    #[new] // above the constructor, this is to be able to contstruct the struct as a class in Python.
    pub fn new(data: String, vector: Vec<u8>) -> RustStruct {
        RustStruct { data, vector }
    }

    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }

    pub fn extend_vector(&mut self, extension: Vec<u8>) {
        for i in extension {
            self.vector.push(i);
        }
    }
}

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
pub struct Human {
    name: String,
    age: u8,
}

#[pymodule]
pub fn learn_struct(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_class::<RustStruct>();
    m.add_function(wrap_pyfunction!(human_say_hi, m)?)?;

    Ok(())
}
