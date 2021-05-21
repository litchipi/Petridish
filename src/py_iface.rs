use crate::genalgo::Genalgo;
use crate::lab::{LabConfig, Cell};
use crate::utils::JsonData;
use crate::errors::Errcode;
use crate::dataset::EmptyDataset;

use paste::paste;

extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
pub struct PyGenalgoInterface<T: Cell>{
    genalgo: Genalgo<T>,
}

impl<T: 'static + Cell> PyGenalgoInterface<T>{
    pub fn new(labcfg_json: JsonData, cfg_json:JsonData) -> Result<PyGenalgoInterface<T>, Errcode>{
        let labcfg = LabConfig::from_json(labcfg_json)?;
        let cfg = LabConfig::from_json(cfg_json)?;
        Ok(PyGenalgoInterface {
            genalgo: Genalgo::new(labcfg)
        })
    }
}
use crate::builtin_algos::algo_test::TestCell;


macro_rules! create_genalgo_py_iface {
    ($name:ident, $celltype:expr) => {
        paste!{

            #[pyclass(unsendable)]
            pub struct [<Genalgo $name PyIface>] {
                genalgo: Genalgo<$celltype>,
            }

            impl [<Genalgo $name PyIface>]{
                pub fn new(labcfg_json: JsonData, cfg_json:JsonData) -> Result<[<Genalgo $name PyIface>], Errcode>{
                    let labcfg = LabConfig::from_json(labcfg_json)?;
                    let cfg = LabConfig::from_json(cfg_json)?;
                    Ok([<Genalgo $name PyIface>] {
                        genalgo: Genalgo::new(labcfg)
                    })
                }
            }

            #[pymethods]
            impl [<Genalgo $name PyIface>]{
                pub fn start(&mut self, data: JsonData){
                    self.genalgo.register_dataset(String::from("empty"), Box::new(EmptyDataset::new(3)));
                    self.genalgo.start(5);
                }

                pub fn apply_map(&mut self, map: JsonData){
                    if let Err(e) = self.genalgo.apply_json_map(map){
                        println!("{}", e);
                    }
                }
            }

            #[pyfunction]
            pub fn [<create_algo_ $name>](labcfg: JsonData, cfg: JsonData) -> [<Genalgo $name PyIface>]{
                match [<Genalgo $name PyIface>]::new(labcfg, cfg){
                    Ok(g) => g,
                    Err(e) => panic!("Not implemented"),
                }
            }
        }
    };
}

create_genalgo_py_iface!(test, TestCell);

#[pymodule]
fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
    /* TODO     Find a way to automatically generate this with a macro
     * m.add_function(wrap_pyfunction!(, m)?).unwrap();
     */
    m.add_function(wrap_pyfunction!(create_algo_test, m)?).unwrap();
    Ok(())
}
