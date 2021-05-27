use crate::genalgo::Genalgo;
use crate::lab::LabConfig;
use crate::utils::JsonData;
use crate::errors::Errcode;
use crate::dataset::EmptyDataset;
use crate::cell::Cell;
use crate::algo::Algo;

use paste::paste;

extern crate pyo3;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::Python;

macro_rules! create_genalgo_py_iface {
    ($name:ident, $celltype:expr) => {
        paste!{

            #[pyclass(unsendable)]
            pub struct [<Genalgo $name PyIface>] {
                genalgo: Genalgo<$celltype>,
            }

            impl [<Genalgo $name PyIface>]{
                pub fn new(labcfg_json: JsonData) -> Result<[<Genalgo $name PyIface>], Errcode>{
                    let labcfg = LabConfig::from_json(labcfg_json)?;
                    Ok([<Genalgo $name PyIface>] {
                        genalgo: Genalgo::new(labcfg)
                    })
                }
            }

            #[pymethods]
            impl [<Genalgo $name PyIface>]{
                pub fn start(&mut self, ngen: usize){
                    self.genalgo.register_dataset(String::from("empty"), Box::new(EmptyDataset::new(3)));
                    let ret = self.genalgo.start(ngen);
                    if let Err(e) = ret{
                        println!("{}", e);
                    }
                }

                pub fn apply_map(&mut self, map: JsonData){
                    if let Err(e) = self.genalgo.apply_json_map(map){
                        println!("{}", e);
                    }
                }
            }

            #[pyfunction]
            pub fn [<create_algo_ $name>](labcfg: JsonData) -> [<Genalgo $name PyIface>]{
                match [<Genalgo $name PyIface>]::new(labcfg){
                    Ok(g) => g,
                    Err(e) => panic!("Not implemented"),
                }
            }

            #[pymodule]
            fn [<algo_ $name>](_py: Python, m: &PyModule) -> PyResult<()> {
                m.add_function(wrap_pyfunction!([<create_algo_ $name>], m)?).unwrap();
                Ok(())
            }
        }
    };
}

use crate::builtin_algos::algo_test::TestCell;
create_genalgo_py_iface!(test, TestCell);



#[pyfunction]
pub fn get_lab_default() -> JsonData{
    match LabConfig::default().to_json(){
        Ok(d) => d,
        Err(e) => {println!("Error: {}", e); //TODO  Python Exception
            return "".to_string(); }
    }
}

#[pymodule]
fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_lab_default, m)?).unwrap();
    //TODO      Find a way to automatically generate this wrap
    m.add_wrapped(wrap_pymodule!(algo_test))?;
    Ok(())
}
