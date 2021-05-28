#![deny(unsafe_code)]

pub mod genalgomethods;
pub mod genalgo;
pub mod utils;
pub mod lab;
pub mod errors;
pub mod dataset;
pub mod algo;
pub mod cell;

#[macro_use]
pub mod py_iface;

pub extern crate paste;
pub extern crate pyo3;
