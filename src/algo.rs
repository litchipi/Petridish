use crate::cell::{Cell, CellData, Genome};
use crate::dataset::GenalgoData;
use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethodsAvailable, GenalgoMethodsConfigurations};
use crate::lab::Lab;
use crate::utils::JsonData;

use serde::{Deserialize, Serialize};

pub trait Algo {
    type CellType: Cell;
    fn new() -> Self
    where
        Self: Sized;

    fn genome_from_json(&self, jsdata: JsonData) -> Genome;
    fn genome_to_json(&self, genome: Genome) -> JsonData;

    fn initialize_cells(&mut self, pop: &mut Vec<Self::CellType>);
    fn create_cell_from_genome(&self, genome: &Genome) -> Self::CellType;

    fn recv_special_data(&mut self, data: &serde_json::Value) -> Result<(), JsonData>;
    fn send_special_data(&self, params: &serde_json::Value) -> Result<JsonData, JsonData>;

    fn process_data(&mut self, pop: &mut Vec<Self::CellType>, data: &GenalgoData);
    fn check_generation_over(&self, genalgo: &Lab<Self::CellType>) -> bool;
    fn reset(&mut self);
}

pub type AlgoID = usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AlgoPopulation{
    WeightofTot(f64),
    FixedSize(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AlgoConfiguration {
    pub id: String,
    pub method: String,
    pub method_options: GenalgoMethodsConfigurations,
    pub give: Vec<AlgoID>,              // Algos to give best cell
    pub impr_genes: Option<Vec<usize>>, // Index of genes to improve
    pub population: AlgoPopulation,
    //TODO  IMPORTANT   Add Vec of objectives (score_index, maximize / minimize)
}

impl AlgoConfiguration {
    pub fn get_pop_and_elite(&self, pop: usize, elite_ratio: f64) -> (usize, usize) {
        let algopop = match self.population{
                AlgoPopulation::WeightofTot(w) => ((pop as f64) * w) as usize,
                AlgoPopulation::FixedSize(s) => s,
        };
        (algopop, ((algopop as f64) * elite_ratio) as usize)
    }

    pub fn default() -> AlgoConfiguration {
        AlgoConfiguration {
            id: "Default".to_string(),
            method: GenalgoMethodsAvailable::Darwin.to_string(),
            method_options: GenalgoMethodsConfigurations::default(GenalgoMethodsAvailable::Darwin),
            give: vec![],
            impr_genes: None,
            population: AlgoPopulation::WeightofTot(1.0),
        }
    }

    pub fn method_default(method: String) -> Option<AlgoConfiguration> {
        Some(AlgoConfiguration {
            id: method.clone() + "_default",
            method_options: GenalgoMethodsConfigurations::default(
                match GenalgoMethodsAvailable::get_by_name(&method) {
                    Some(m) => m,
                    None => return None,
                }),
            give: vec![],
            method: method,
            impr_genes: None,
            population: AlgoPopulation::WeightofTot(1.0),
        })
    }

    pub fn from_json(jsdata: JsonData) -> Result<AlgoConfiguration, serde_json::Error> {
        serde_json::from_str(&jsdata)
    }

    pub fn to_json(&self) -> Result<JsonData, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

pub struct AlgoResult {
    pub cells_data: Vec<CellData>,
    pub exterior_elites: Vec<CellData>,
    nelite: usize,
}

impl AlgoResult {
    pub fn new(nelite: usize) -> AlgoResult {
        AlgoResult {
            cells_data: vec![],
            exterior_elites: vec![],
            nelite: nelite,
        }
    }

    pub fn load_cells<T: Cell>(&mut self, cells: &Vec<T>) {
        self.cells_data.extend(
            cells
                .iter()
                .map(|c| c.get_data().clone())
                .collect::<Vec<CellData>>(),
        );
    }

    pub fn sort_cells(&mut self, maximize: bool) -> Result<(), Errcode> {
        if maximize {
            self.cells_data
                .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        } else {
            self.cells_data
                .sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        }
        Ok(())
    }

    pub fn get_elites(&self) -> Vec<&CellData> {
        let mut res = vec![];
        for i in 0..self.nelite {
            res.push(self.cells_data.get(i).unwrap());
        }
        res.extend(self.exterior_elites.iter());
        res
    }

    pub fn clone_top_cells(&self) -> Vec<CellData> {
        let mut res = vec![];
        for i in 0..self.nelite {
            res.push(self.cells_data.get(i).unwrap().clone());
        }
        res
    }
}
