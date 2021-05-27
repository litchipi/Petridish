#![deny(unsafe_code)]
#![allow(dead_code, unused_variables, unused_imports)]

mod genalgomethods;
mod genalgo;
mod builtin_algos;
mod utils;
mod lab;
mod errors;
mod dataset;
mod algo;
mod cell;

#[macro_use]
pub mod py_iface;

pub extern crate paste;
#[macro_use]
pub extern crate pyo3;
