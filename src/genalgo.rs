use std::sync::{Condvar, Mutex};
use std::time::Instant;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::mem;

use rand::prelude::*;

use crate::errors::Errcode;
use crate::dataset::{DatasetHandler, EmptyDataset};
use crate::lab::*;
use crate::builtin_algos;
use crate::genalgomethods;
use crate::utils::{MeanCompute, JsonData};
use log::{info, trace, warn};

extern crate serde;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GenalgoConfiguration{
    //TODO Implement a way to choose between a limitation in number of cells, mem used or both
    //(first limitation reached)
    max_nb_cells: Option<u32>,
    max_mem_used: usize,            // in bytes
    genalgo_method: genalgomethods::GenalgoMethodsAvailable,
    pub (crate) genalgo_method_config: genalgomethods::GenalgoMethodsConfigurations,
}

impl GenalgoConfiguration{
    fn default() -> GenalgoConfiguration{
        GenalgoConfiguration {
            max_nb_cells: Some(1000),
            max_mem_used: (200*1024),
            genalgo_method: genalgomethods::GenalgoMethodsAvailable::Darwin,
            genalgo_method_config: genalgomethods::load_default_config(genalgomethods::GenalgoMethodsAvailable::Darwin)
        }
    }
}


type LabExport = Vec<(LabConfig, Vec<AlgoConfiguration>, Vec<AlgoResult>, Vec<Genome>)>;

/*  Used to manage labs, get datasets, import / export configurations, binds to Python API,
 *  etc...*/
pub struct Genalgo<T: Cell>{
    lab:        Lab<T>,
    datasets:   Vec<Box<dyn DatasetHandler>>,
    datasets_id: Vec<String>,
}

impl<T: 'static + Cell> Genalgo<T>{
    pub fn new(labconfig: LabConfig, config: GenalgoConfiguration) -> Genalgo<T>{
        Genalgo{
            lab: Lab::new(labconfig, config.genalgo_method),
            datasets: vec![],
            datasets_id: vec![],
        }
    }

    pub fn export_lab(&self) -> Result<JsonData, Errcode>{
        let exports : LabExport = vec![];
        Err(Errcode::NotImplemented("export_lab"))
    }

    pub fn import_lab(&self, data: JsonData) -> Result<(), Errcode>{
        Err(Errcode::NotImplemented("import_lab"))
    }

    pub fn test_function(&self) {
        println!("Genalgo created for usage with cell of type \"{}\"", type_name::<T>());
    }


    pub fn register_dataset(&mut self, id: String, dataset: Box<dyn DatasetHandler>){
        self.datasets.push(dataset);
        self.datasets_id.push(id);
    }

    pub fn remove_dataset(&mut self, id: String) -> Result<(), Errcode>{
        let ds_ind = self.datasets_id.iter().enumerate().filter(|(_, i)| *i == &id).map(|(n, _)| n).collect::<Vec<usize>>();
        if let Some(ind) = ds_ind.get(0){
            self.datasets_id.remove(*ind);
            self.datasets.remove(*ind);
            Ok(())
        }else{
            Err(Errcode::DatasetDoesntExist(id))
        }
    }

    pub fn start(&mut self, ngeneration:usize) -> Result<(), Errcode>{
        self.lab.start(ngeneration, &mut self.datasets)
    }
}

use crate::builtin_algos::algo_test;
pub type GenalgoTest = Genalgo<algo_test::TestCell>;

#[test]
pub fn test_genalgo(){
    let config = GenalgoConfiguration::default();
    let labconfig = LabConfig { npop: 1000, elite_ratio: 0.1, maximize_score: true};
    let mut genalgo = GenalgoTest::new(labconfig, config);
    genalgo.register_dataset(String::from("empty"), Box::new(EmptyDataset::new(3)));
    genalgo.start(5);
}
