#[macro_export]
macro_rules! generate_py_ifaces {
    [$petridish:ident, $([$name:ident] $celltype:tt => ($($algoname:ident => $algotype:ty),+)),* $(,)?] => {
        use $petridish::*;
        use $petridish::paste::paste;
        use $petridish::pyo3::prelude::*;
        use $petridish::pyo3::{wrap_pymodule, wrap_pyfunction};
        use $petridish::pyo3::Python;

        use $petridish::genalgo::Genalgo;
        use $petridish::lab::LabConfig;
        use $petridish::utils::JsonData;
        use $petridish::errors::Errcode;
        use $petridish::dataset::EmptyDataset;
        use $petridish::cell::Cell;
        use $petridish::algo::Algo;

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
