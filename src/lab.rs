use crate::algo::{Algo, AlgoConfiguration, AlgoID, AlgoResult};
use crate::cell::{random_genome, Cell, CellData, Genome};
use crate::dataset::DatasetHandler;
use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethod, GenalgoMethodsAvailable};
use crate::utils::{JsonData, MeanCompute};

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::SystemTime;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct LabConfig {
    pub npop: usize,
    pub elite_ratio: f64,
    pub maximize_score: bool,
}

impl LabConfig {
    pub fn default() -> LabConfig {
        LabConfig {
            npop: 1000,
            elite_ratio: 0.1,
            maximize_score: false,
        }
    }

    pub fn new(npop: usize, elite_ratio: f64, maximize_score: bool) -> LabConfig {
        LabConfig {
            npop: npop,
            elite_ratio: elite_ratio,
            maximize_score: maximize_score,
        }
    }

    pub fn from_json(_js: JsonData) -> Result<LabConfig, Errcode> {
        //TODO  Implement LabConfig from JsonData
        Ok(LabConfig::default())
    }

    pub fn to_json(&self) -> Result<JsonData, serde_json::Error> {
        serde_json::to_string(self)
    }
}

pub struct Lab<T: Cell> {
    genalgo_methods: Vec<Box<dyn GenalgoMethod<T>>>,

    pub algos: Vec<Box<dyn Algo<CellType = T>>>,
    configs: Vec<AlgoConfiguration>,
    bestgens: Vec<Genome>,
    cells: Vec<Vec<T>>,

    out_algo: Option<AlgoID>, //Algo from which getting the result
    config: LabConfig,
    init_done: bool,
    algo_configs_set: bool,

    mean_calc: MeanCompute,
}

impl<T: 'static + Cell> Lab<T> {
    pub fn new(config: LabConfig) -> Lab<T> {
        Lab {
            genalgo_methods: vec![],

            algos: vec![],
            configs: vec![],
            bestgens: vec![],
            cells: vec![],

            out_algo: Option::None,
            config: config,
            init_done: false,
            algo_configs_set: false,
            mean_calc: MeanCompute::new(),
        }
    }

    pub fn apply_map(&mut self, map: Vec<AlgoConfiguration>) -> Result<(), Errcode> {
        if self.algos.len() != map.len() {
            return Err(Errcode::SizeError("map", self.algos.len(), map.len()));
        }
        self.__validate_map(&map)?;
        self.configs = map.clone();
        self.algo_configs_set = true;
        Ok(())
    }

    pub fn register_new_algo(
        &mut self,
        algo: Box<dyn Algo<CellType = T>>,
    ) -> Result<AlgoID, Errcode> {
        self.configs.push(AlgoConfiguration::default());
        self.bestgens.push(random_genome(T::get_genome_length()));
        self.cells.push(vec![]);
        self.algos.push(algo);
        Ok(self.algos.len() - 1)
    }

    pub fn configure_algo(
        &mut self,
        id: AlgoID,
        config: AlgoConfiguration,
    ) -> Result<(), Errcode> {
        self.__check_id_exist(id)?;
        let old_config = self.configs[id].clone();
        self.configs[id] = config;
        if let Err(e) = self.__validate_map(&self.configs) {
            self.configs[id] = old_config;
            return Err(e);
        }
        self.algo_configs_set = true;
        Ok(())
    }

    pub fn import_best_genome(
        &mut self,
        genome: Genome,
        algo_id: Option<AlgoID>,
    ) -> Result<(), Errcode> {
        if let Some(id) = algo_id {
            self.__check_id_exist(id)?;
            self.bestgens[id] = genome;
        } else {
            for id in 0..self.bestgens.len() {
                self.bestgens[id] = genome.clone();
            }
        }
        Ok(())
    }

    pub fn export_best_genome(&mut self) -> Result<Genome, Errcode> {
        if let Some(id) = self.out_algo {
            Ok(self.bestgens[id].clone())
        } else {
            Err(Errcode::NotSet("output algorithm"))
        }
    }

    pub fn recv_special_data(
        &mut self,
        id: AlgoID,
        data: &serde_json::Value,
    ) -> Result<(), Errcode> {
        self.__check_id_exist(id)?;
        match self.algos[id].recv_special_data(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(Errcode::SpecialDataError(e)),
        }
    }

    pub fn send_special_data(
        &self,
        id: AlgoID,
        params: &serde_json::Value,
    ) -> Result<JsonData, Errcode> {
        self.__check_id_exist(id)?;
        match self.algos[id].send_special_data(params) {
            Ok(d) => Ok(d),
            Err(e) => Err(Errcode::SpecialDataError(e)),
        }
    }

    pub fn start(
        &mut self,
        ngeneration: usize,
        datasets: &mut Vec<Box<dyn DatasetHandler>>,
    ) -> Result<CellData, Errcode> {
        self.__validate_configuration()?;
        self.__init_lab()?;
        let mut top_cell: Option<CellData> = Option::None;
        for _ in 0..ngeneration {
            let t = SystemTime::now();
            top_cell = Some(self.__loop_gen(datasets)?);
            self.mean_calc
                .add_el(t.elapsed().unwrap().as_secs_f64(), 1.0);
            //println!("avg generation time: {}", self.mean_calc.result);
        }
        Ok(top_cell.unwrap())
    }

    /*              INTERNALS               */
    fn get_method_from_algo(
        &mut self,
        algoid: AlgoID,
    ) -> Result<&mut Box<dyn GenalgoMethod<T>>, Errcode> {
        match self.configs.get(algoid) {
            Some(cfg) => {
                if let Ok(method_enum_n) = GenalgoMethodsAvailable::from_str(&cfg.method) {
                    match self.genalgo_methods.get_mut(method_enum_n as usize) {
                        Some(m) => {
                            m.load_config(&cfg.method_options);
                            Ok(m)
                        }
                        None => {
                            return Err(Errcode::CodeError(
                                "get_method_from_algo genalgo_methods get_mut",
                            ))
                        }
                    }
                } else {
                    return Err(Errcode::CodeError("get_method_from_algo from_str"));
                }
            }
            None => return Err(Errcode::IdDoesntExist(algoid)),
        }
    }

    fn __init_genalgo_methods(&mut self) -> Result<(), Errcode> {
        for method in GenalgoMethodsAvailable::iter() {
            self.genalgo_methods.push(method.build());
        }
        Ok(())
    }

    fn __loop_gen(
        &mut self,
        datasets: &mut Vec<Box<dyn DatasetHandler>>,
    ) -> Result<CellData, Errcode> {
        for dataset in datasets.iter_mut() {
            self.__run_on_dataset(dataset)?;
        }

        let mut results: Vec<AlgoResult> = vec![];
        for id in 0..self.algos.len() {
            results.push(AlgoResult::new(
                self.configs
                    .get(id)
                    .unwrap()
                    .get_pop_and_elite(self.config.npop, self.config.elite_ratio)
                    .1,
            ));
        }
        let mut top_cell: Option<CellData> = Option::None;
        for id in 0..self.algos.len() {
            results[id].load_cells(self.cells.get(id).unwrap());
            results[id].sort_cells(self.config.maximize_score)?;
            if id == self.out_algo.unwrap() {
                top_cell = Some(results[id].cells_data[0].clone());
            }
            self.__propagate_results(id, &mut results)?;
            self.__prepare_next_gen(id, &results[id])?;
            self.algos[id].reset();
        }
        Ok(top_cell.unwrap())
    }

    fn __run_on_dataset(&mut self, dataset: &mut Box<dyn DatasetHandler>) -> Result<(), Errcode> {
        dataset.prepare();
        loop {
            let new_data_got = dataset.get_next_data();
            if let Some(new_data) = new_data_got {
                for (n, algo) in self.algos.iter_mut().enumerate() {
                    algo.process_data(&mut self.cells[n], &new_data);
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn __propagate_results(
        &mut self,
        id: AlgoID,
        results: &mut Vec<AlgoResult>,
    ) -> Result<(), Errcode> {
        for togive in self.configs.get(id).unwrap().give.iter() {
            self.__check_id_exist(*togive)?;
            let top_cells = results.get(id).unwrap().clone_top_cells();
            results
                .get_mut(*togive)
                .unwrap()
                .exterior_elites
                .extend(top_cells);
        }
        Ok(())
    }

    fn __prepare_next_gen(&mut self, id: AlgoID, res: &AlgoResult) -> Result<(), Errcode> {
        let mut genomes = vec![];
        self.get_method_from_algo(id)?.process_results(
            &res.get_elites(),
            &res.cells_data,
            &mut genomes,
        )?;

        if genomes.len() != self.cells.get(id).unwrap().len() {
            return Err(Errcode::CodeError(
                "prepare next gen: genome list len != nb cells",
            ));
        }
        if let Some(cells) = self.cells.get_mut(id) {
            for i in 0..genomes.len() {
                cells.get_mut(i).unwrap().reset(genomes.get(i).unwrap());
            }
        } else {
            return Err(Errcode::IdDoesntExist(id));
        }
        Ok(())
    }

    fn __init_lab(&mut self) -> Result<(), Errcode> {
        if !self.algo_configs_set {
            return Err(Errcode::NotSet("algos configuration"));
        }
        if self.init_done {
            return Ok(());
        }
        self.__init_genalgo_methods()?;
        for id in 0..self.algos.len() {
            let mut genomes = vec![];

            let (pop, elite) = self
                .configs
                .get(id)
                .unwrap()
                .get_pop_and_elite(self.config.npop, self.config.elite_ratio);

            let bestgens = self.bestgens.get(id).unwrap().clone();
            self.get_method_from_algo(id)?.init_population(
                &bestgens,
                pop as u32,
                elite as u32,
                &mut genomes,
            )?;

            for gene in genomes.iter() {
                self.cells
                    .get_mut(id)
                    .unwrap()
                    .push(self.algos.get(id).unwrap().create_cell_from_genome(gene));
            }
            self.algos
                .get_mut(id)
                .unwrap()
                .initialize_cells(self.cells.get_mut(id).unwrap());
        }
        self.init_done = true;
        Ok(())
    }

    fn __check_id_exist(&self, id: AlgoID) -> Result<(), Errcode> {
        if id < self.algos.len() {
            Ok(())
        } else {
            Err(Errcode::IdDoesntExist(id))
        }
    }

    fn __validate_configuration(&mut self) -> Result<(), Errcode> {
        if self.config.npop < 100 {
            return Err(Errcode::InsuffisantPopulation(self.config.npop, 100));
        }
        //TODO  Check if the sum of all algos = total population
        if self.algos.len() == 0 {
            return Err(Errcode::NotSet("lab algorithms"));
        }
        if self.algos.len() == 1 {
            self.out_algo = Some(0);
        } else if self.out_algo == Option::None {
            return Err(Errcode::NotSet("output algorithm"));
        }
        if self.algos.len() != self.configs.len() {
            return Err(Errcode::CodeError("algos len != configs len"));
        }
        if self.cells.len() != self.algos.len() {
            return Err(Errcode::CodeError("populations len != algos len"));
        }
        if self.configs.len() != self.bestgens.len() {
            return Err(Errcode::CodeError("configs len != bestgens len"));
        }

        for method in self.genalgo_methods.iter() {
            method.validate_config()?;
        }
        Ok(())
    }

    fn __validate_map(&self, map: &Vec<AlgoConfiguration>) -> Result<(), Errcode> {
        for cfg in map.iter() {
            if let Err(_) = GenalgoMethodsAvailable::from_str(&cfg.method) {
                return Err(Errcode::ValidationError("genalgo method"));
            }
        }
        Ok(())
    }

    fn __debug_fct(&self) {
        println!(
            "Algos-related len: {} algos, {} configs, {} cells, {} bestgens",
            self.algos.len(),
            self.configs.len(),
            self.cells.len(),
            self.bestgens.len()
        );
        print!("Cells in algos: ");
        for (n, c) in self.cells.iter().enumerate() {
            print!("{}: {}, ", n, c.len());
        }
        println!("");
    }
}
