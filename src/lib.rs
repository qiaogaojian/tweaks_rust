use std::collections::HashMap;
use pyo3::prelude::*;

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

#[pymodule]
fn rustcore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello_to_python, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;

    Ok(())
}