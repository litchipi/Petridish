//TODO  Change JsonData in/out to PyDict

#[macro_export]
macro_rules! generate_py_ifaces {
    [$petridish:ident, $([$name:ident] $celltype:tt => ($($algoname:ident => $algotype:ty),+)),*
        $(,)?] => {
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
        use $petridish::algo::{AlgoConfiguration, Algo, AlgoID};
        use $petridish::genalgomethods::GenalgoMethodsAvailable;
        use $petridish::labmaps::LabMapAssistant;

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

                    pub fn start(&mut self, ngen: usize) -> (Vec<f64>, f64){
                        let c = py_err_if_fail!(self.genalgo.start(ngen));
                        (c.genome, c.score)
                    }

                    pub fn get_special_data(&mut self, id: AlgoID, data: JsonData) -> JsonData{
                        py_err_if_fail!(self.genalgo.send_special_data(id, data))
                    }

                    pub fn push_special_data(&mut self, id: AlgoID, data: JsonData){
                        py_err_if_fail!(self.genalgo.recv_special_data(id, data));
                    }

                    pub fn register_empty_dataset(&mut self, ndata: usize){
                        self.genalgo.register_dataset(String::from("empty"),
                            Box::new(EmptyDataset::new(ndata))
                            );
                    }
                    
                    pub fn set_output_algorithm(&mut self, ind: AlgoID){
                        self.genalgo.set_output_algorithm(ind);
                    }

                    pub fn apply_map(&mut self, map: JsonData){
                        py_err_if_fail!(self.genalgo.apply_json_map(map));
                    }


                    $(

                        pub fn [<apply_map_with_algo_ $algoname>](&mut self, map: JsonData){
                            py_err_if_fail!(self.genalgo
                                .apply_map_with_algo::<$algotype>(map));
                        }

                        pub fn [<register_algo_ $algoname>](&mut self) -> usize{
                            py_err_if_fail!(self.genalgo.lab.register_new_algo(
                                Box::new(<$algotype as Algo>::new())
                                ))
                        }
                    )*

                    pub fn configure_algo(&mut self, ind: usize, conf: JsonData){
                        println!("Configuring algorithm {}: {}", ind, conf);
                        py_err_if_fail!(self.genalgo.lab.configure_algo(ind,
                            py_err_if_fail!(AlgoConfiguration::from_json(conf))
                        ));
                    }
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
            py_err_if_fail!(LabConfig::default().to_json())
        }

        #[pyfunction]
        pub fn get_algo_default() -> JsonData{
            py_err_if_fail!(AlgoConfiguration::default().to_json())
        }

        #[pyfunction]
        pub fn create_labmap_assistant(mapformat: String) -> LabMapAssistant {
            LabMapAssistant::new(mapformat)
        }

        #[pymodule]
        fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_function(wrap_pyfunction!(get_lab_default, m)?).unwrap();
            m.add_function(wrap_pyfunction!(create_labmap_assistant, m)?).unwrap();
            m.add_function(wrap_pyfunction!(get_algo_default, m)?).unwrap();
            $(
                paste! {
                    m.add_function(wrap_pyfunction!([<create_lab_ $name>], m)?).unwrap();
                }
            )*
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! raise_python_error{
    [$msg:expr] => {
        panic!($msg)    //TODO raise_python_error!
    }
}

#[macro_export]
macro_rules! py_err_if_none{
    [$x:expr, $msg:expr] => {
        match $x{
            Some(data) => data,
            None => raise_python_error!($msg),
        }
    };
}

#[macro_export]
macro_rules! py_err_if_fail{
    [$x:expr, $msg:expr] => {
        match $x{
            Ok(data) => data,
            Err(e) => raise_python_error!(format!("{}: \"{}\"", $msg, e)),
        }
    };
    [$x:expr] => {
        match $x{
            Ok(data) => data,
            Err(e) => raise_python_error!(format!("Error: \"{}\"", e)),
        }
    };
}
