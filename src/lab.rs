use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethodsAvailable, GenalgoMethod};
use crate::genalgo::Genalgo;
use crate::dataset::{GenalgoData, DatasetHandler};
use crate::utils::JsonData;

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

#[derive(Clone, Serialize)]
pub struct CellData{
    pub genome: Genome,
    pub score: Score,
    pub version: u64,
}

pub trait Cell{
    fn get_data(&self) -> &CellData;
    fn action(&mut self, data: &GenalgoData);
    fn reset(&mut self, genome: &Genome);
    fn genome_version_adapt(genome: &Genome, version: u64) -> Genome;
}





pub trait Algo{
    type CellType : Cell;
    fn new() -> Self where Self : Sized;
    
    fn genome_from_json(&self, jsdata: JsonData) -> Genome;
    fn genome_to_json(&self, genome: Genome) -> JsonData;
    fn get_genome_length(&self) -> usize;
    
    fn initialize_cells(&mut self, pop: &mut Vec<Self::CellType>);
    fn create_cell_from_genome(&self, genome: &Genome) -> Self::CellType;

    fn recv_special_data(&mut self, data: &serde_json::Value);
    fn send_special_data(&self, params: &serde_json::Value) -> JsonData;

    fn process_data(&mut self, pop: &mut Vec<Self::CellType>, data: &GenalgoData);
    fn check_generation_over(&self, genalgo: &Genalgo<Self::CellType>) -> bool;
    fn reset(&mut self);
}

pub type AlgoID = usize;

pub struct AlgoConfiguration{
    give:           Vec<AlgoID>,        // Algos to give best cell
    impr_genes:      Option<Vec<usize>>, // Index of genes to improve
    weight_in_pop:  f64                 // Part of the population (in weight)
}

impl AlgoConfiguration{
    pub fn get_pop_and_elite(&self, pop: usize, elite_ratio: f64) -> (usize, usize){
        (
            ((pop as f64)*self.weight_in_pop) as usize,
            (((pop as f64)*self.weight_in_pop)*elite_ratio) as usize,
        )
    }
}

pub struct AlgoResult{
    cells_data: Vec<CellData>,
    exterior_elites: Vec<CellData>,
    nelite: usize
}

impl AlgoResult{
    pub fn new(nelite: usize) -> AlgoResult{
        AlgoResult{
            cells_data: vec![],
            exterior_elites: vec![],
            nelite: nelite,
        }
    }

    pub fn load_cells<T: Cell>(&mut self, cells: &Vec<T>){
        self.cells_data.extend(cells.iter().map(|c| c.get_data().clone()).collect::<Vec<CellData>>());
    }

    pub fn sort_cells(&mut self, maximize: bool) -> Result<(), Errcode>{
        if maximize{
            self.cells_data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        }else{
            self.cells_data.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        }
        Ok(())
    }

    pub fn get_elites(&self) -> Vec<&CellData>{
        let mut res = vec![];
        for i in 0..self.nelite{
            res.push(self.cells_data.get(i).unwrap());
        }
        res.extend(self.exterior_elites.iter().map(|e| e).collect::<Vec<&CellData>>());
        res
    }

    pub fn clone_top_cells(&self) -> Vec<CellData>{
        let mut res = vec![];
        for i in 0..self.nelite{
            res.push(self.cells_data.get(i).unwrap().clone());
        }
        res
    }
}

#[derive(Serialize, Deserialize)]
pub struct LabConfig{
    npop:           usize,
    elite_ratio:    f64,
    maximize_score: bool,
}

impl LabConfig{
    pub fn from_json(js: JsonData) -> Result<LabConfig, Errcode>{
        //TODO  LabConfig from JsonData
        Ok(LabConfig { npop: 10, elite_ratio: 0.4, maximize_score: false})
    }
}


pub struct Lab<T: Cell>{
    method:     Box<dyn GenalgoMethod<T>>, //GenalgoMethodsAvailable,

    algos:      Vec<Box<dyn Algo<CellType = T>>>,
    configs:    Vec<AlgoConfiguration>,
    bestgens:   Vec<Genome>,
    cells:      Vec<Vec<T>>,

    out_algo:   Option<AlgoID>,     //Algo from which getting the result
    config:     LabConfig,
    init_done:  bool,
}

impl<T: 'static + Cell> Lab<T>{
    pub fn new(config: LabConfig, method: GenalgoMethodsAvailable) -> Lab<T>{
        Lab {
            method:     method.get_method(),

            algos:      vec![],
            configs:    vec![],
            bestgens:   vec![],
            cells:      vec![],

            out_algo:   Option::None,
            config:     config,
            init_done:  false,
        }
    }

    pub fn register_new_algo(&mut self, algo: Box<dyn Algo<CellType = T>>) -> AlgoID {
        self.configs.push(AlgoConfiguration{give:vec![], impr_genes: Option::None, weight_in_pop: 0.0});
        self.bestgens.push(random_genome(algo.get_genome_length()));
        self.cells.push(vec![]);
        self.algos.push(algo);
        self.algos.len() - 1
    }

    pub fn configure_algo(&mut self, id: AlgoID, config: AlgoConfiguration) -> Result<(), Errcode> {
        self.__check_id_exist(id)?;
        self.configs[id] = config;
        Ok(())
    }

    pub fn import_best_genome(&mut self, genome: Genome, algo_id: Option<AlgoID>) -> Result<(), Errcode> {
        if let Some(id) = algo_id{
            self.__check_id_exist(id)?;
            self.bestgens[id] = genome;
        }else{
            for id in 0..self.bestgens.len(){
                self.bestgens[id] = genome.clone();
            }
        }
        Ok(())
    }

    pub fn export_best_genome(&mut self) -> Result<Genome, Errcode> {
        if let Some(id) = self.out_algo{
            Ok(self.bestgens[id].clone())
        }else{
            Err(Errcode::NotSet("output algorithm"))
        }
    }

    pub fn recv_special_data(&mut self, id: AlgoID, data: &serde_json::Value) ->  Result<(), Errcode>{
        self.__check_id_exist(id)?;
        self.algos[id].recv_special_data(data);
        Ok(())
    }

    pub fn send_special_data(&self, id: AlgoID, params: &serde_json::Value) -> Result<JsonData, Errcode>{
        self.__check_id_exist(id)?;
        Ok(self.algos[id].send_special_data(params))
    }

    pub fn start(&mut self, ngeneration: usize, datasets: &mut Vec<Box<dyn DatasetHandler>>) -> Result<(), Errcode>{
        self.__validate_configuration()?;
        self.__init_lab()?;
        for _ in 0..ngeneration{
            self.__loop_gen(datasets)?;
        }
        Ok(())
    }





    /*              INTERNALS               */
    fn __loop_gen(&mut self, datasets: &mut Vec<Box<dyn DatasetHandler>>) -> Result<(), Errcode>{
        for dataset in datasets.iter_mut(){
            self.__run_on_dataset(dataset)?;
        }

        let mut results: Vec<AlgoResult> = vec![];
        for id in 0..self.algos.len(){
            results.push(AlgoResult::new(self.configs.get(id).unwrap().get_pop_and_elite(self.config.npop, self.config.elite_ratio).1));
        }
        for id in 0..self.algos.len(){
            results[id].load_cells(self.cells.get(id).unwrap());
            results[id].sort_cells(self.config.maximize_score)?;
            self.__propagate_results(id, &mut results)?;
            self.__prepare_next_gen(id, &results[id])?;
        }
        Ok(())
    }

    fn __run_on_dataset(&mut self, dataset: &mut Box<dyn DatasetHandler>) -> Result<(), Errcode>{
        dataset.prepare();
        loop{
            let new_data_got = dataset.get_next_data();
            if let Some(new_data) = new_data_got{
                for (n, algo) in self.algos.iter_mut().enumerate(){
                    algo.process_data(&mut self.cells[n], &new_data);
                }
            }else{
                break;
            }
        }
        Ok(())
    }

    fn __propagate_results(&mut self, id: AlgoID, results: &mut Vec<AlgoResult>) -> Result<(), Errcode>{
        for togive in self.configs.get(id).unwrap().give.iter(){
            self.__check_id_exist(*togive)?;
            let top_cells = results.get(id).unwrap().clone_top_cells();
            results.get_mut(*togive).unwrap().exterior_elites.extend(top_cells);
        }
        Ok(())
    }

    fn __prepare_next_gen(&mut self, id: AlgoID, res: &AlgoResult) -> Result<(), Errcode>{
        let mut genomes = vec![];
        self.method.process_results(
            &res.get_elites(),
            &res.cells_data,
            self.algos.get(id).unwrap(),
            &mut genomes
        )?;

        if genomes.len() != self.cells.get(id).unwrap().len(){
            return Err(Errcode::CodeError("prepare next gen: genome list len != nb cells"));
        }
        for i in 0..genomes.len(){
            self.cells.get_mut(id).unwrap().get_mut(i).unwrap().reset(genomes.get(i).unwrap());
        }
        Ok(())
    }

    fn __init_lab(&mut self) -> Result<(), Errcode>{
        if self.init_done{ return Ok(()); }
        for id in 0..self.algos.len(){
            let mut genomes = vec![];

            let (pop, elite) = self.configs.get(id).unwrap().get_pop_and_elite(self.config.npop, self.config.elite_ratio);
            self.method.init_method(
                self.bestgens.get(id).unwrap(),
                pop as u32, elite as u32,
                self.algos.get(id).unwrap(),
                &mut genomes)?;

            for gene in genomes.iter(){
                self.cells.get_mut(id).unwrap().push(self.algos.get(id).unwrap().create_cell_from_genome(gene));
            }
            self.algos.get_mut(id).unwrap().initialize_cells(self.cells.get_mut(id).unwrap());
        }
        Ok(())
    }

    fn __check_id_exist(&self, id: AlgoID) -> Result<(), Errcode> {
        if id < self.algos.len(){Ok(())}
        else { Err(Errcode::IdDoesntExist(id))}
    }

    fn __validate_configuration(&self) -> Result<(), Errcode>{
        if self.config.npop < 100                  { return Err(Errcode::InsuffisantPopulation(self.config.npop, 100)); }
        //TODO  Check if the sum of all algos = total population

        if self.algos.len() == 0            { return Err(Errcode::NotSet("register algorithms")); }
        if self.out_algo == Option::None    { return Err(Errcode::NotSet("output algorithm")); }

        if self.algos.len() != self.configs.len()   { return Err(Errcode::CodeError("algos len != configs len")) }
        if self.cells.len() != self.algos.len()     { return Err(Errcode::CodeError("populations len != algos len")) }
        if self.configs.len() != self.bestgens.len(){ return Err(Errcode::CodeError("configs len != bestgens len")) }
        self.method.validate_config()?;
        Ok(())
    }
}
