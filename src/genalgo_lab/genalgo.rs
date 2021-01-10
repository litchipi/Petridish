use crate::algorithms;
use crate::genalgo_lab::{JsonData, ThreadID};
use log::{info, trace, warn};

extern crate serde;
extern crate pyo3;
use serde::{Serialize, Deserialize};

pub type Genome = Vec<f64>;
pub type Score = u64;

pub trait Algo{
    fn import_genome_from_json(jsdata: JsonData) -> Genome;
    fn export_genome_to_json(genome: Genome) -> JsonData;
    fn data_from_json(jsdata: JsonData, vec: Vec<f64>);
    fn create_cell_from_genome(&self, genome: &Genome) -> algorithms::AllCellsTypes;
    fn get_genome_length(&self) -> usize;
}

pub trait Cell{
    fn get_score(&self) -> Score;
    fn action(&self, data: Vec<f64>);
}

#[derive(Serialize)]
pub struct GenalgoConfiguration{
    nb_cells: u32
}

impl GenalgoConfiguration{
    fn default() -> GenalgoConfiguration{
        GenalgoConfiguration {nb_cells: 1000}
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

    /*          VARIABLES           */
    population: Vec<algorithms::AllCellsTypes>
}

impl Genalgo {
    /*              API             */
    pub fn create_algo(name: &str) -> Genalgo{
        Genalgo {
            status: GenalgoStatus { started: false, epoch: 0},
            bestgen: vec![],
            thread_id: 0,
            algo: algorithms::get_algo(name),
            config: GenalgoConfiguration::default(),
            population: vec![]
        }
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
            self.generate_random_population();
        } else if self.bestgen.len() < self.algo.unwrap().get_genome_length() {
            trace!("Best genome length < expected genome length, skipping");
            self.generate_random_population();
        } else {
            self.generate_population_from_bestgen(self.bestgen.clone());
        }
    }



    /*          GENERAL FUNCTIONS           */

    fn get_elite_nb(&self) -> usize{
        (f64::from(self.config.nb_cells)*0.1) as usize
    }

    /*          POPULATION MANIPULATION     */
    fn generate_population_from_bestgen(&mut self, bestgen: Genome){
        let mut genomes: Vec<Genome> = vec![];

        genomes.push(bestgen.clone());
        
        for i in 1..self.get_elite_nb(){
            genomes.push(self.mutate_genome(&bestgen, 0.75));
        }

        for i in 0..((self.config.nb_cells as usize) - genomes.len()){
            genomes.push(self.random_genome());
        }

        self.generate_cells(genomes);

    }

    fn generate_random_population(&mut self){
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
        Genome::new()   //TODO Random genome generation
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


