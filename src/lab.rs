use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethodsAvailable, GenalgoMethod, GenalgoMethodsConfigurations, load_default_config};
use crate::genalgo::Genalgo;
use crate::dataset::{GenalgoData, DatasetHandler};
use crate::utils::JsonData;
use crate::cell::{Genome, random_genome, Cell};
use crate::algo::{Algo, AlgoConfiguration, AlgoID, AlgoResult};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct LabConfig{
    pub npop:           usize,
    pub elite_ratio:    f64,
    pub maximize_score: bool,
    genalgo_method: GenalgoMethodsAvailable,
    pub (crate) genalgo_method_config: GenalgoMethodsConfigurations,
}

impl LabConfig{
    pub fn default() -> LabConfig{
        LabConfig {
            npop: 1000,
            elite_ratio: 0.1,
            maximize_score: false,
            genalgo_method: GenalgoMethodsAvailable::Darwin,
            genalgo_method_config: load_default_config(GenalgoMethodsAvailable::Darwin)
        }
    }

    pub fn new(npop: usize, elite_ratio: f64, maximize_score: bool) -> LabConfig{
        let method = GenalgoMethodsAvailable::default();
        LabConfig{npop: npop, elite_ratio: elite_ratio, maximize_score: maximize_score,
        genalgo_method: method,
        genalgo_method_config: load_default_config(method),
        }
    }

    pub fn from_json(js: JsonData) -> Result<LabConfig, Errcode>{
        //TODO  Implement LabConfig from JsonData
        Ok(LabConfig::default())
    }

    pub fn to_json(&self) -> Result<JsonData, serde_json::Error>{
        serde_json::to_string(self)
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
    pub fn new(config: LabConfig) -> Lab<T>{
        Lab {
            method:     config.genalgo_method.get_method(),

            algos:      vec![],
            configs:    vec![],
            bestgens:   vec![],
            cells:      vec![],

            out_algo:   Option::None,
            config:     config,
            init_done:  false,
        }
    }

    pub fn apply_map(&mut self, map: Vec<AlgoConfiguration>) -> Result<(), Errcode> {
        if self.algos.len() != map.len(){
            return Err(Errcode::SizeError("map", self.algos.len(), map.len()))
        }
        if !self.__validate_map(&map){
            return Err(Errcode::ValidationError("map"));
        }
        self.configs = map.clone();
        Ok(())
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
        let old_config = self.configs[id].clone();
        self.configs[id] = config;
        if !self.__validate_map(&self.configs){
            self.configs[id] = old_config;
            return Err(Errcode::ValidationError("map"));
        }
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
        println!("Loop gen");
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
        println!("Init lab");
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
        println!("Validate configuration");
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

    fn __validate_map(&self, map: &Vec<AlgoConfiguration>) -> bool{
        true    //TODO  Implement Algo map validation
    }
}
