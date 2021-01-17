#![deny(unsafe_code)]
#![allow(dead_code, unused_variables, unused_imports)]

extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod genalgomethods;
mod genalgo;
mod algorithms;
mod utils;

/*          API             */

#[pyfunction]
fn create(name: &str) -> genalgo::Genalgo {
    genalgo::Genalgo::create_algo(name)
}

#[pymodule]
fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create, m)?).unwrap();
    Ok(())
}
