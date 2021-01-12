use std::collections::HashMap;

use rand::prelude::*;

use crate::algorithms;
use crate::genalgo_lab::{JsonData, ThreadID};
use log::{info, trace, warn};

extern crate serde;
extern crate pyo3;
use serde::{Serialize, Deserialize};

pub type Genome = Vec<f64>;
pub type Score = u64;




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

pub trait Algo{
    // Utilitary
    fn genome_from_json(jsdata: JsonData) -> Genome;
    fn genome_to_json(genome: Genome) -> JsonData;
    fn data_from_json(jsdata: JsonData, vec: Vec<f64>);
    fn create_cell_from_genome(&self, genome: &Genome) -> algorithms::AllCellsTypes;
    fn get_genome_length(&self) -> usize;

}

pub trait Cell{
    fn get_score(&self) -> Score;
    fn action(&self, data: Vec<f64>);
}

#[derive(Serialize, Deserialize)]
pub struct GenalgoConfiguration{
    nb_cells: u32,
    percent_elite: f32,
    variation_elite_pct: f32
}

impl GenalgoConfiguration{
    fn default() -> GenalgoConfiguration{
        GenalgoConfiguration {
            nb_cells: 1000,
            percent_elite: 0.1,
            variation_elite_pct: 0.25
        }
    }
}

struct GenalgoSettings {
    nb_elites: u32
}

impl GenalgoSettings{
    fn from_config(cfg: &GenalgoConfiguration) -> GenalgoSettings{
        GenalgoSettings {
            nb_elites: (f64::from(cfg.nb_cells)*(cfg.percent_elite as f64)) as u32
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
    config: GenalgoConfiguration,
    settings: GenalgoSettings,

    /*          VARIABLES           */
    population: Vec<algorithms::AllCellsTypes>,
}


impl Genalgo {
    /*              API             */
    pub fn create_algo(name: &str) -> Genalgo{
        let config = GenalgoConfiguration::default();
        let settings = GenalgoSettings::from_config(&config);
        Genalgo {
            status: GenalgoStatus {started: false, epoch: 0},
            bestgen: vec![],
            thread_id: 0,
            algo: algorithms::get_algo(name),
            config: config,
            settings: settings, //GenalgoSettings::new(),
            population: vec![],
        }
    }

    pub fn load_json_config(&mut self, jsdata: JsonData){
        self.config = serde_json::from_str(jsdata.as_str()).expect("Cannot deserialize json to configuration");
        self.settings = GenalgoSettings::from_config(&self.config);
    }

    pub fn start_algo(&mut self){
        // TODO Initialise genalgo loop & start it
    }

    pub fn load_bestgen_from_json(&mut self, jsdata: JsonData){

    }



    /*          INTERNALS WRAPPERS          */
    fn loop_fct(&mut self){
        // TODO Genalgo main loop
    }

    fn init_generation(&mut self){
        if self.bestgen.len() == 0 {
            self.__init_generate_random_population();
        } else if self.bestgen.len() < self.algo.unwrap().get_genome_length() {
            trace!("Best genome length < expected genome length, skipping");
            self.__init_generate_random_population();
        } else {
            self.__init_generate_population_from_bestgen(self.bestgen.clone());
        }
    }



    /*          POPULATION MANIPULATION     */
    fn __init_generate_population_from_bestgen(&mut self, bestgen: Genome){
        let mut genomes: Vec<Genome> = vec![];

        genomes.push(bestgen.clone());
        
        for i in 1..self.settings.nb_elites {
            genomes.push(self.mutate_genome(&bestgen, 0.75));
        }

        for i in 0..((self.config.nb_cells as usize) - genomes.len()){
            genomes.push(self.random_genome());
        }

        self.generate_cells(genomes);

    }

    fn __init_generate_random_population(&mut self){
        let mut genomes: Vec<Genome> = vec![];
        self.population = vec![];
        for i in 0..self.config.nb_cells{
            genomes.push(self.random_genome());
        }

        self.generate_cells(genomes);
    }




    /*          GENOME MANIPULATION         */

    fn mutate_genome(&self, bestgen: &Genome, rate: f32) -> Genome {
        bestgen.clone()
    }

    fn random_genome(&self) -> Genome {
        let mut rng = rand::thread_rng();
        {
            let mut res : Genome = vec![];
            for i in 0..self.algo.unwrap().get_genome_length(){
                res.push(rng.gen());
            }
            res
        }
    }



    /*          CELLS MANIPULATION          */
    fn generate_cells(&mut self, genomes: Vec<Genome>){
        self.population = vec![];
        for g in genomes.iter(){
            self.population.push(self.algo.unwrap().create_cell_from_genome(g));
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
fn test_random_genome_generation(){
    let a = Genalgo::create_algo("algo_test");
    let genome_a = a.random_genome();
    let genome_b = a.random_genome();
    assert_eq!(genome_a.len(), a.algo.unwrap().get_genome_length());
    assert_eq!(genome_b.len(), genome_a.len());
    println!("{:?}", genome_a);
    println!("{:?}", genome_b);
    for i in 0..genome_a.len(){
        assert_ne!(genome_a[i], genome_b[i]);
    }
}
