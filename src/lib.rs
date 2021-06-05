#![deny(unsafe_code)]

pub mod algo;
pub mod cell;
pub mod dataset;
pub mod errors;
pub mod genalgo;
pub mod genalgomethods;
pub mod lab;
pub mod labmaps;
pub mod utils;

#[macro_use]
pub mod py_iface;

pub extern crate enum_dispatch;
pub extern crate paste;
pub extern crate pyo3;
