use crate::utils::JsonData;
use crate::dataset::GenalgoData;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

pub type Genome = Vec<f64>;

pub fn random_genome(ngens: usize) -> Genome{
    let mut rng = rand::thread_rng();
    let mut res = vec![];
    for _ in 0..ngens {
        res.push(rng.gen());
    }
    res
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



pub type Score = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellData{
    pub genome: Genome,
    pub score: Score,
    pub version: u64,
}

pub trait Cell{
    fn get_genome_length() -> usize;
    fn get_data(&self) -> &CellData;
    fn action(&mut self, data: &GenalgoData);
    fn reset(&mut self, genome: &Genome);
    fn genome_version_adapt(genome: &Genome, version: u64) -> Genome;
}
