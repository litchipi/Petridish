#![allow(dead_code, unused_variables, unused_imports)]

extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod genalgo_lab;
mod algorithms;


/*          API             */

#[pyfunction]
fn create_engine() -> genalgo_lab::GenalgoEngine {
    genalgo_lab::GenalgoEngine::new()
}

#[pymodule]
fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_engine, m)?).unwrap();
    Ok(())
}

/*              TESTS               */

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
