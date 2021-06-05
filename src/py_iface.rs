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
                        let ret = self.genalgo.start(ngen);
                        match ret{
                            Err(e) => {
                                println!("Error: {}", e);
                                return (vec![], 0.0);
                            },
                            Ok(c) => {
                                return (c.genome, c.score);
                            }
                        }
                    }

                    pub fn get_special_data(&mut self, id: AlgoID, data: JsonData) -> JsonData{
                        match self.genalgo.send_special_data(id, data){
                            Ok(d) => d,
                            Err(e) => {println!("{}", e); "".to_string()},
                        }
                    }

                    pub fn push_special_data(&mut self, id: AlgoID, data: JsonData){
                        if let Err(e) = self.genalgo.recv_special_data(id, data){
                            println!("{}", e);
                        }
                    }

                    pub fn register_empty_dataset(&mut self, ndata: usize){
                        self.genalgo.register_dataset(String::from("empty"),
                            Box::new(EmptyDataset::new(ndata))
                            );
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
                        pub fn [<register_algo_ $algoname>](&mut self) -> i32{
                            match self.genalgo.lab.register_new_algo(
                                Box::new(<$algotype as Algo>::new())
                                ){
                                Err(e) => {
                                    println!("Error: {}", e);
                                    return -1;
                                },
                                Ok(ind) => ind as i32,
                            }
                        }
                    )*

                    pub fn configure_algo(&mut self, ind: usize, conf: JsonData){
                        println!("Configuring algorithm {}: {}", ind, conf);
                        if let Err(e) = self.genalgo.lab.configure_algo(ind,
                            match AlgoConfiguration::from_json(conf){
                                Ok(j) => j,
                                Err(e) => {
                                    println!("Error: {}", e);
                                    return;
                                },
                            }
                        ){
                            println!("Error: {}", e);
                        }
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
            match LabConfig::default().to_json(){
                Ok(d) => d,
                Err(e) => {println!("Error: {}", e); //TODO  Python Exception
                    return "".to_string(); }
            }
        }

        #[pyfunction]
        pub fn get_algo_default() -> JsonData{
            match AlgoConfiguration::default().to_json(){
                Ok(d) => d,
                Err(e) => {println!("Error: {}", e);
                    return "".to_string(); }
            }
        }

        #[pyfunction]
        pub fn create_labmap_assistant(
            mapformat_str: String,
            isomethod_str: String,
            mixmethod_str: String,
            bundlemethod_str: String,
            finalmethod_str: String,
        ) -> LabMapAssistant {
            let isomethod = py_err_if_none!(
                GenalgoMethodsAvailable::get_by_name(isomethod_str),
                "Wrong iso method"
            );
            let mixmethod = py_err_if_none!(
                GenalgoMethodsAvailable::get_by_name(mixmethod_str),
                "Wrong mix method"
            );
            let bundlemethod = py_err_if_none!(
                GenalgoMethodsAvailable::get_by_name(bundlemethod_str),
                "Wrong bundle method"
            );
            let finalmethod = py_err_if_none!(
                GenalgoMethodsAvailable::get_by_name(finalmethod_str),
                "Wrong final method"
            );

            LabMapAssistant::new(
                mapformat_str,
                isomethod,
                mixmethod,
                bundlemethod,
                finalmethod,
            )
        }
        //TODO  Function raise Python error

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
        panic!($msg)
    }
}

#[macro_export]
macro_rules! py_err_if_none{
    [$x:expr, $msg:expr] => {
        match $x{
            Some(data) => data,
            None => raise_python_error!($msg),
        }
    }
}
