use std::sync::{Condvar, Mutex};
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

use rand::prelude::*;

use crate::algorithms;
use crate::genalgo_lab::{JsonData, ThreadID, genalgomethods};
use log::{info, trace, warn};

extern crate serde;
extern crate pyo3;
use serde::{Serialize, Deserialize};

pub type Genome = Vec<f64>;
pub type Score = f64;
pub type GenalgoData = Vec<f64>;

pub fn __genome_from_json(jsdata: JsonData, key_list: &Vec<&str>) -> Genome{
    let data: serde_json::Value = serde_json::from_str(jsdata.as_str()).expect("Error while parsing json");
    let mut res = Genome::new();
    for key in key_list.iter(){
        res.push(data[key].as_f64().expect("Error while decoding genome, cannot convert as floating number"));
    }
    res
}

pub fn __genome_to_json(genome: Genome, key_list: &Vec<&str>) -> JsonData{
        let mut result : HashMap<&str, f64> = HashMap::new();
        for (nb, key) in key_list.into_iter().enumerate(){
            result.insert(key, genome[nb]);
        }
        serde_json::to_string(&result).expect("Cannot serialize genome to Json")
}

pub (crate) trait GenalgoMethod{
    fn load_config(&mut self, cfg: &GenalgoConfiguration, set: &GenalgoSettings);
    fn init_method(&mut self, bestgen: &Genome, algo: &algorithms::AlgoAvailable) -> Vec<Genome>;
    fn process_results(&self, cells: Vec<&CellData>, var: &GenalgoVardata, algo: &algorithms::AlgoAvailable) -> Vec<Genome>;
    fn sort_before_process(&self) -> bool;
}

pub trait Algo{
    fn genome_from_json(&self, jsdata: JsonData) -> Genome;
    fn genome_to_json(&self, genome: Genome) -> JsonData;
    fn data_from_json(&self, jsdata: JsonData, vec: Vec<f64>);
    fn recv_special_data(&mut self, data: &serde_json::Value);
    fn create_cell_from_genome(&self, genome: &Genome) -> algorithms::AllCellsTypes;
    fn get_genome_length(&self) -> usize;
    fn check_generation_over(&self, genalgo: &Genalgo) -> bool;
    fn get_cell_size(&self) -> usize;
}

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
    max_nb_cells: u32,
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
            max_nb_cells: 1000,
            max_mem_used: (200*1024),
            databuffer_max_length: 10,
            genalgo_method: genalgomethods::GenalgoMethodsAvailable::Darwin,
            genalgo_method_config: genalgomethods::load_default_config(genalgomethods::GenalgoMethodsAvailable::Darwin)
        }
    }
}

pub (crate) struct GenalgoVardata {
    current_dataset_nb: u8,
    new_data: Mutex<bool>,
    new_data_signal: Condvar,
}

impl GenalgoVardata{
    fn from(cfg: &GenalgoConfiguration, set: &GenalgoSettings) -> GenalgoVardata{
        GenalgoVardata {
            current_dataset_nb: set.nb_datasets,
            new_data_signal: Condvar::new(),
            new_data: Mutex::new(false),
        }

    }
}

#[derive(Clone)]
pub (crate) struct GenalgoSettings {
    pub (crate) nb_datasets: u8,
    pub (crate) nb_cells: u32,
}

impl GenalgoSettings{
    fn from_config(cfg: &GenalgoConfiguration, cell_memsize: usize) -> GenalgoSettings{
        let nb_cells = cmp::min(cfg.max_nb_cells as u32, (cfg.max_mem_used/cell_memsize) as u32);
        GenalgoSettings {
            nb_datasets: 0,
            nb_cells: nb_cells,
        }
    }
}

#[derive(Serialize)]
pub struct GenalgoStatus{
    started: bool,
    epoch: u64,
}

pub struct Genalgo {
    /*          PUBLIC              */
    pub status: GenalgoStatus,
    pub bestgen: Genome,
    pub thread_id: ThreadID,

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
    /*              API             */
    pub fn create_algo(name: &str) -> Genalgo{
        let algo = algorithms::get_algo(name);
        let config = GenalgoConfiguration::default();
        let settings = GenalgoSettings::from_config(&config, algo.unwrap().get_cell_size());
        let vardata = GenalgoVardata::from(&config, &settings);
        let method = genalgomethods::get_method(config.genalgo_method);

        Genalgo {
            status: GenalgoStatus {started: false, epoch: 0},
            bestgen: vec![],
            thread_id: 0,
            algo: algo,
            ga_method: method,
            config: config,
            settings: settings,
            vardata: vardata,
            population: vec![],
            databuff: VecDeque::new()
        }
    }

    pub fn load_json_config(&mut self, jsdata: JsonData){
        self.setup_configuration(serde_json::from_str(jsdata.as_str()).expect("Cannot deserialize json to configuration"));
    }

    fn setup_configuration(&mut self, cfg: GenalgoConfiguration){
        self.config = cfg;
        self.settings = GenalgoSettings::from_config(&self.config, self.algo.unwrap().get_cell_size());
        if !self.status.started{
            self.vardata = GenalgoVardata::from(&self.config, &self.settings);
        }
    }

    pub fn start_algo(&mut self){
        self.ga_method.unwrap().load_config(&self.config, &self.settings);
        let genomes = self.ga_method.unwrap().init_method(&self.bestgen, &self.algo);
        self.generate_cells(genomes);
        self.status.started = true;
        loop{
            self.loop_fct();
        }
    }

    pub fn load_bestgen_from_json(&mut self, jsdata: JsonData){
        self.bestgen = self.algo.unwrap().genome_from_json(jsdata);
    }

    pub fn save_bestgen_to_json(&self) -> JsonData {
        self.algo.unwrap().genome_to_json(self.bestgen.clone())
    }

    pub fn receive_data(&mut self, data: serde_json::Value) -> bool {
        if self.databuff.len() >= (self.config.databuffer_max_length as usize){
            false
        }else{
            //let data: serde_json::Value = serde_json::from_str(jsdata.as_str()).expect("Error while parsing json");
            if data.get("algo_special_data") != Option::None {
                self.algo.unwrap_mut().recv_special_data(&data["algo_special_data"]);
            }

            if data.get("dataset_nb") != Option::None {
                self.vardata.current_dataset_nb = data["dataset_nb"].as_u64().expect("Cannot load value from dataset_nb parameter") as u8;
            }

            if data.get("dataset_total") != Option::None {
                self.settings.nb_datasets = data["dataset_total"].as_u64().expect("Cannot load value from dataset_total parameter") as u8;
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

    fn perform_action_on_data(&mut self, data: &GenalgoData) -> u8 {
        for cell in self.population.iter_mut(){
            cell.unwrap_mut()
                .action(&data);
        }
        self.settings.nb_datasets - self.vardata.current_dataset_nb
    }


    fn loop_fct(&mut self){
        loop {
            let data: GenalgoData = self.__wait_new_data();
            if self.perform_action_on_data(&data) == 0{
                break;
            }
        }

        let sort_cells = self.ga_method.unwrap().sort_before_process();
        if sort_cells{
            self.__sort_cells();
        }

        let celldatarefs : Vec<&CellData> = {
            let mut res = vec![];
            for cell in self.population.iter(){
                res.push(cell.unwrap().get_data())
            }
            res
        };

        let genomes = self.ga_method.unwrap().process_results(celldatarefs, &self.vardata, &self.algo);

        self.generate_cells(genomes);
        self.status.epoch += 1;
    }

    /*          POPULATION MANIPULATION     */
    fn __sort_cells(&mut self){
        if self.config.maximize_score{
            self.population.sort_by(|a, b| b.unwrap().get_data().score.partial_cmp(&a.unwrap().get_data().score).unwrap());
        }else{
            self.population.sort_by(|a, b| a.unwrap().get_data().score.partial_cmp(&b.unwrap().get_data().score).unwrap());
        }
    }

    fn generate_cells(&mut self, genomes: Vec<Genome>){
        self.population = vec![];
        let algo = self.algo.unwrap();
        for g in genomes.iter(){
            self.population.push(algo.create_cell_from_genome(g));
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
    cfg.max_nb_cells = 1000;
    cfg.max_mem_used = 1024*1024;
    println!("Test cells size: {}", a.algo.unwrap().get_cell_size());
    a.setup_configuration(cfg.clone());
    assert_eq!(a.settings.nb_cells, 1000);
    cfg.max_mem_used = 10*a.algo.unwrap().get_cell_size();
    a.setup_configuration(cfg.clone());
    assert_eq!(a.settings.nb_cells, 10);
    cfg.max_mem_used = 1024*1024;
    cfg.max_nb_cells = u32::MAX;
    a.setup_configuration(cfg.clone());
    println!("Nb test cells in 1Mb: {}", a.settings.nb_cells);
}

#[test]
fn test_optimisation_ratio_parts_sizes(){

}
