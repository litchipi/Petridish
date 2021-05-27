use crate::genalgo::Genalgo;
use crate::lab::LabConfig;
use crate::utils::JsonData;
use crate::errors::Errcode;
use crate::dataset::EmptyDataset;
use crate::cell::Cell;
use crate::algo::Algo;

pub use paste::paste;
pub use pyo3::prelude::*;
pub use pyo3::{wrap_pyfunction, wrap_pymodule};
pub use pyo3::Python;

#[macro_export]
macro_rules! generate_py_ifaces {
    [$([$name:ident] $celltype:tt => ($($algoname:ident => $algotype:ty),+)),* $(,)?] => {
        $(
            paste!{
                #[pyclass(unsendable, dict)]
                pub struct [<Lab $name PyIface>] {
                    genalgo: Genalgo<$celltype>,
                }

                impl [<Lab $name PyIface>]{
                    pub fn new(labcfg_json: JsonData) -> Result<[<Lab $name PyIface>], Errcode>{
                        let labcfg = LabConfig::from_json(labcfg_json)?;
                        Ok([<Lab $name PyIface>] {
                            genalgo: Genalgo::new(labcfg)
                        })
                    }
                }

                #[pymethods]
                impl [<Lab $name PyIface>]{
                    pub fn start(&mut self, ngen: usize){
                        self.genalgo.register_dataset(String::from("empty"), Box::new(EmptyDataset::new(3)));
                        let ret = self.genalgo.start(ngen);
                        println!("genalgo finished");
                        if let Err(e) = ret{
                            println!("Error");
                            println!("{}", e);
                        }
                    }

                    pub fn apply_map(&mut self, map: JsonData){
                        $(
                            println!("{} => {}", stringify!($algoname), stringify!($algotype));
                        )*
                        if let Err(e) = self.genalgo.apply_json_map(map){
                            println!("{}", e);
                        }
                    }

                    $(
                        pub fn [<register_algo_ $algoname>](&mut self){
                            if let Err(e) = self.genalgo.lab.register_new_algo(Box::new(<$algotype as Algo>::new())){
                                println!("Error: {}", e);
                            }
                        }
                    )*
                }

                #[pyfunction]
                pub fn [<create_lab_ $name>](labcfg: JsonData) -> [<Lab $name PyIface>]{
                    match [<Lab $name PyIface>]::new(labcfg){
                        Ok(g) => g,
                        Err(e) => panic!("Not implemented"),
                    }
                }
            }
        )*


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
            $(
                paste! {
                    m.add_function(wrap_pyfunction!([<create_lab_ $name>], m)?).unwrap();
                }
            )*
            Ok(())
        }
    };
}

//TODO IMPORTANT Migrate builtin_algos in separate exemples to be used

use crate::builtin_algos::benchmark_fcts::{BenchmarkCell, BenchmarkAlgo};
use crate::builtin_algos::algo_test::{TestCell, TestAlgo, TestAlgo2};

generate_py_ifaces!(
    [test] TestCell => (test => TestAlgo, test2 => TestAlgo2),
    [benchmark] BenchmarkCell => (benchmark => BenchmarkAlgo),
);
