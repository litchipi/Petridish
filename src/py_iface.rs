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
        use $petridish::algo::{AlgoConfiguration, Algo};

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
                                println!("Error");
                                println!("{}", e);
                                return (vec![], 0.0);
                            },
                            Ok(c) => {
                                return (c.genome, c.score);
                            }
                        }
                    }

                    pub fn register_empty_dataset(&mut self, ndata: usize){
                        self.genalgo.register_dataset(String::from("empty"), Box::new(EmptyDataset::new(ndata)));
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
                            match self.genalgo.lab.register_new_algo(Box::new(<$algotype as Algo>::new())){
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

        #[pymodule]
        fn genalgo(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_function(wrap_pyfunction!(get_lab_default, m)?).unwrap();
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
