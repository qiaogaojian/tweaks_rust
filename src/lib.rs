use std::collections::HashMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyfunction]
fn say_hello_to_python() -> PyResult<String> {
    Ok("Hello, Python!".to_string())
}

#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[pyfunction]
fn get_fibonacci(number: isize) -> PyResult<u128> {
    if number == 1 {
        return Ok(1);
    } else if number == 2 {
        return Ok(2);
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

#[pyfunction]
fn benchmark_get_fibonacci(num: isize) -> PyResult<u128> {
    for _i in 1..num {
        let _ = get_fibonacci(100);
    }
    Ok(num as u128)
}

#[pyfunction]
fn list_sum(arr: Vec<isize>) -> PyResult<isize> {
    let mut sum: isize = 0;
    for i in arr {
        sum += i;
    }
    Ok(sum)
}

#[pyfunction]
fn dict_printer(map: HashMap<String, String>) {
    for (key, value) in map {
        println!("{} {}", key, value);
    }
}

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
fn human_say_hi(human_data: String) {
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

#[pymodule]
fn rustcore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    let _ = m.add_class::<RustStruct>();
    m.add_function(wrap_pyfunction!(human_say_hi, m)?)?;

    Ok(())
}