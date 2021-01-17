use std::sync::{Condvar, Mutex};
use std::time::Instant;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

use rand::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::algorithms;
use crate::genalgomethods;
use crate::utils::{MeanCompute, JsonData};
use log::{info, trace, warn};

extern crate serde;
extern crate pyo3;
use serde::{Serialize, Deserialize};

pub type Genome = Vec<f64>;
pub type Score = f64;
pub type GenalgoData = Vec<f64>;

pub fn export_cell(cell: &CellData) -> JsonData{
    serde_json::to_string(cell).unwrap()
}

pub fn __genome_from_json(jsdata: JsonData, key_list: &Vec<&str>) -> Genome{
    let data: serde_json::Value = serde_json::from_str(jsdata.as_str()).expect("Error while parsing json");
    let mut res = Genome::new();
    for key in key_list.iter(){
        res.push(data[key].as_f64().expect("Error while decoding genome, cannot convert as floating number"));
    }
    res
}

pub fn __genome_to_json(genome: Genome, key_list: &Vec<&str>) -> JsonData{
    assert_eq!(genome.len(), key_list.len());
    let mut result : HashMap<&str, f64> = HashMap::new();
    for (nb, key) in key_list.into_iter().enumerate(){
        result.insert(key, genome[nb]);
    }
    serde_json::to_string(&result).expect("Cannot serialize genome to Json")
}

pub (crate) trait GenalgoMethod{
    fn load_config(&mut self, cfg: &GenalgoConfiguration, set: &GenalgoSettings);
    fn init_method(&mut self, bestcell: &CellData, algo: &algorithms::AlgoAvailable) -> Vec<Genome>;
    fn process_results(&mut self, maximize: bool, cells: Vec<&CellData>, var: &GenalgoVardata, algo: &algorithms::AlgoAvailable) -> Vec<Genome>;
    fn reset(&mut self);
}

pub trait Algo{
    fn genome_from_json(&self, jsdata: JsonData) -> Genome;
    fn genome_to_json(&self, genome: Genome) -> JsonData;
    fn data_from_json(&self, jsdata: JsonData, vec: Vec<f64>);
    fn recv_special_data(&mut self, data: &serde_json::Value);
    fn send_special_data(&self, params: &serde_json::Value) -> JsonData;
    fn create_cell_from_genome(&self, genome: &Genome) -> algorithms::AllCellsTypes;
    fn get_genome_length(&self) -> usize;
    fn check_generation_over(&self, genalgo: &Genalgo) -> bool;
    fn get_cell_size(&self) -> usize;
    fn initialize_cells(&mut self, pop: &mut Vec<algorithms::AllCellsTypes>);
    fn perform_action_on_data(&mut self, pop: &mut Vec<algorithms::AllCellsTypes>, data: &GenalgoData);
    fn reset(&mut self);
}

#[derive(Clone, Serialize)]
pub struct CellData{
    pub genome: Genome,
    pub score: Score
}

pub trait Cell{
    fn get_data(&self) -> &CellData;
    fn action(&mut self, data: &GenalgoData);
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GenalgoConfiguration{
    //TODO Implement a way to choose between a limitation in number of cells, mem used or both
    //(first limitation reached)
    max_nb_cells: Option<u32>,
    max_mem_used: usize,            // in bytes
    databuffer_max_length: u16,
    genalgo_method: genalgomethods::GenalgoMethodsAvailable,
    pub (crate) genalgo_method_config: genalgomethods::GenalgoMethodsConfigurations,
    maximize_score: bool
}

impl GenalgoConfiguration{
    fn default() -> GenalgoConfiguration{
        GenalgoConfiguration {
            maximize_score: true,
            max_nb_cells: Some(1000),
            max_mem_used: (200*1024),
            databuffer_max_length: 10,
            genalgo_method: genalgomethods::GenalgoMethodsAvailable::Darwin,
            genalgo_method_config: genalgomethods::load_default_config(genalgomethods::GenalgoMethodsAvailable::Darwin)
        }
    }
}

pub (crate) struct GenalgoVardata {
    new_data: Mutex<bool>,
    new_data_signal: Condvar,
}

impl GenalgoVardata{
    fn from(cfg: &GenalgoConfiguration, set: &GenalgoSettings) -> GenalgoVardata{
        GenalgoVardata {
            new_data_signal: Condvar::new(),
            new_data: Mutex::new(false),
        }

    }
}

#[derive(Clone)]
pub (crate) struct GenalgoSettings {
    pub (crate) nb_cells: u32,
}

impl GenalgoSettings{
    fn from_config(cfg: &GenalgoConfiguration, cell_memsize: usize) -> GenalgoSettings{
        let nb_cells: u32;
        if let Some(nb) = cfg.max_nb_cells {
            nb_cells = cmp::min(nb as u32, (cfg.max_mem_used/cell_memsize) as u32);
        }else{
            nb_cells = (cfg.max_mem_used/cell_memsize) as u32;
        }
        GenalgoSettings {
            nb_cells: nb_cells,
        }
    }
}

#[pyclass]
pub struct Genalgo {
    /*          PUBLIC              */
    #[pyo3(get)]
    pub init: bool,
    #[pyo3(get)]
    pub epoch: u64,
    pub bestcell: CellData,
    pub avgtime: MeanCompute,

    /*          PRIVATE             */
    algo: algorithms::AlgoAvailable,
    pub (crate) config: GenalgoConfiguration,
    pub (crate) settings: GenalgoSettings,
    pub (crate) ga_method: genalgomethods::AllGenalgoMethod,

    /*          VARIABLES           */
    pub (crate) population: Vec<algorithms::AllCellsTypes>,
    pub (crate) vardata: GenalgoVardata,
    pub (crate) databuff: VecDeque<GenalgoData>
}


impl Genalgo {
    pub fn create_algo(name: &str) -> Genalgo{
        let algo = algorithms::get_algo(name);
        let config = GenalgoConfiguration::default();
        let settings = GenalgoSettings::from_config(&config, algo.unwrap().get_cell_size());
        let vardata = GenalgoVardata::from(&config, &settings);
        let method = genalgomethods::get_method(config.genalgo_method);

        Genalgo {
            init: false,
            epoch: 0,
            avgtime: MeanCompute::new(),
            bestcell: CellData {genome: vec![], score: 0.0},
            algo: algo,
            ga_method: method,
            config: config,
            settings: settings,
            vardata: vardata,
            population: vec![],
            databuff: VecDeque::new()
        }
    }

    fn reset_algo(&mut self){
        self.epoch = 0;
        self.avgtime = MeanCompute::new();
        self.bestcell = CellData {genome: vec![], score: 0.0};
        self.population = vec![];
        self.databuff = VecDeque::new();
        self.algo.unwrap_mut().reset();
        self.ga_method.unwrap().reset();

    }

    fn setup_configuration(&mut self, cfg: GenalgoConfiguration){
        self.config = cfg;
        self.settings = GenalgoSettings::from_config(&self.config, self.algo.unwrap().get_cell_size());
        if !self.init{
            self.vardata = GenalgoVardata::from(&self.config, &self.settings);
        }
    }

    fn __receive_datum(&mut self, data: &serde_json::Value) -> bool {
        if self.databuff.len() >= (self.config.databuffer_max_length as usize){
            false
        }else{
            //let data: serde_json::Value = serde_json::from_str(jsdata.as_str()).expect("Error while parsing json");
            if data.get("algo_special_data") != Option::None {
                self.algo.unwrap_mut().recv_special_data(&data["algo_special_data"]);
            }

            if data.get("data") != Option::None {
                let mut vec: GenalgoData = vec![];
                let data_array = data["data"].as_array().unwrap();
                for d in data_array.iter(){
                    vec.push(d.as_f64().expect("Cannot load data from data array parameter"));
                }
                self.databuff.push_back(vec);
                *self.vardata.new_data.try_lock().unwrap() = true;
                self.vardata.new_data_signal.notify_all();
            }
            true
        }
    }


    /*          INTERNALS WRAPPERS          */

    fn __wait_new_data(&mut self) -> GenalgoData {
        if *self.vardata.new_data.try_lock().unwrap(){
            let data = self.databuff.pop_front().unwrap();
            *self.vardata.new_data.try_lock().unwrap() = self.databuff.len() > 0;
            data
        }else{
            if let Err(_) = self.vardata.new_data_signal.wait(self.vardata.new_data.lock().unwrap()){
                panic!("Cannot get mutex from Condvar unwrap");
            };
            self.__wait_new_data()
        }
    }



    /*          POPULATION MANIPULATION     */
    fn __sort_cells(&mut self){
        if self.config.maximize_score{
            self.population.sort_by(|a, b| b.unwrap().get_data().score.partial_cmp(&a.unwrap().get_data().score).unwrap());
        }else{
            self.population.sort_by(|a, b| a.unwrap().get_data().score.partial_cmp(&b.unwrap().get_data().score).unwrap());
        }
        self.bestcell = (*self.population[0].unwrap().get_data()).clone();
    }

    fn generate_cells(&mut self, genomes: Vec<Genome>){
        self.population = vec![];
        let algo = self.algo.unwrap();
        for g in genomes.iter(){
            self.population.push(algo.create_cell_from_genome(g));
        }
    }
}


#[pymethods]
impl Genalgo {
    /*              API             */
    pub fn get_avg_process_time(&self) -> f64{
        self.avgtime.result
    }

    pub fn get_cells_number(&self) -> u32{
        self.settings.nb_cells
    }

    pub fn load_json_config(&mut self, jsdata: JsonData){
        if self.init{
            panic!("Configuration of genalgo only possible before initialisation");
        }
        self.setup_configuration(serde_json::from_str(jsdata.as_str()).expect("Cannot deserialize json to configuration"));
    }

    pub fn save_json_config(&mut self) -> JsonData{
        serde_json::to_string(&self.config).expect("Cannot parse configuration to json")
    }

    pub fn init_algo(&mut self){
        if self.init{
            self.reset_algo();
        }
        self.ga_method.unwrap().load_config(&self.config, &self.settings);
        let genomes = self.ga_method.unwrap().init_method(&self.bestcell, &self.algo);
        self.generate_cells(genomes);
        self.algo.unwrap_mut().initialize_cells(&mut self.population);
        self.init = true;
    }

    pub fn run_on_data(&mut self) -> bool {
        if self.databuff.len() == 0{
            panic!("No data in databuffer");
        }
        for i in 0..self.databuff.len(){
            let data = self.databuff.pop_front().unwrap();
            let now = Instant::now();
            self.algo.unwrap_mut().perform_action_on_data(&mut self.population, &data);
            self.avgtime.add_el(now.elapsed().as_secs_f64(), 1.0);
            if self.algo.unwrap().check_generation_over(&self){
                return true;
            }
        }
        false
    }

    pub fn finish_generation(&mut self) -> u64 {
        self.__sort_cells();
        let celldatarefs : Vec<&CellData> = {
            let mut res = vec![];
            for cell in self.population.iter(){
                res.push(cell.unwrap().get_data())
            }
            res
        };
        let genomes = self.ga_method.unwrap().process_results(self.config.maximize_score, celldatarefs, &self.vardata, &self.algo);
        self.generate_cells(genomes);
        self.epoch += 1;
        self.epoch
    }

    pub fn load_bestgen_from_json(&mut self, jsdata: JsonData){
        self.bestcell.genome = self.algo.unwrap().genome_from_json(jsdata);
    }

    pub fn save_bestcell_to_json(&self) -> JsonData {
        export_cell(&self.bestcell)
    }

    pub fn receive_data(&mut self, jsdata: &str) -> u32{
        let data = serde_json::from_str(jsdata).expect("Cannot decode Json data properly");
        if let serde_json::Value::Array(darray) = data{
            let mut received = 0;
            for d in darray.iter(){
                if self.__receive_datum(d){
                    received += 1
                }
            }
            received
        } else {
            self.__receive_datum(&data);
            1
        }
    }

}

// TODO Write tests
#[test]
fn test_init_generation_zero(){

}

#[test]
fn test_init_generation_bestgen_little(){

}

#[test]
fn test_init_generation_bestgen(){

}

#[test]
fn test_algo_genome_import_export(){
}

#[test]
fn test_genome_json_bindings(){
    let key_list = vec!["a", "b", "acsve", "azefazklej", "alkzeflkazejl"];
    let genome = vec![0.342, 0.532, 0.1232, 0.657543456, 0.12341513453];
    let exported = __genome_to_json(genome.clone(), &key_list);
    println!("{}", exported);
    let imported = __genome_from_json(exported, &key_list);
    for nb in 0..imported.len(){
        assert_eq!(genome[nb], imported[nb]);
    }
}
#[test]
fn test_nb_cells_created(){
    let mut a = Genalgo::create_algo("algo_test");
    let mut cfg = GenalgoConfiguration::default();
    cfg.max_nb_cells = Some(1000);
    cfg.max_mem_used = 1024*1024;
    println!("Test cells size: {}", a.algo.unwrap().get_cell_size());
    a.setup_configuration(cfg.clone());
    assert_eq!(a.settings.nb_cells, 1000);
    cfg.max_mem_used = 10*a.algo.unwrap().get_cell_size();
    a.setup_configuration(cfg.clone());
    assert_eq!(a.settings.nb_cells, 10);
    cfg.max_mem_used = 1024*1024;
    cfg.max_nb_cells = Option::None;
    a.setup_configuration(cfg.clone());
    println!("Nb test cells in 1Mb: {}", a.settings.nb_cells);
}

#[test]
fn test_optimisation_ratio_parts_sizes(){

}
