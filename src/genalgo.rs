use std::any::type_name;

use crate::algo::{AlgoConfiguration, AlgoID, AlgoResult, Algo};
use crate::cell::{Cell, CellData, Genome};
use crate::dataset::DatasetHandler;
use crate::errors::Errcode;
use crate::lab::*;
use crate::utils::cells_from_memory;
use crate::utils::JsonData;

type LabExport = Vec<(
    LabConfig,
    Vec<AlgoConfiguration>,
    Vec<AlgoResult>,
    Vec<Genome>,
)>;

/*  Used to manage labs, get datasets, import / export configurations, binds to Python API,
 *  etc...*/
pub struct Genalgo<T: Cell> {
    pub lab: Lab<T>,
    datasets: Vec<Box<dyn DatasetHandler>>,
    datasets_id: Vec<String>,
}

impl<T: 'static + Cell> Genalgo<T> {
    pub fn max_cell_nb(max_mem_usage: usize) -> usize {
        cells_from_memory::<T>(max_mem_usage)
    }

    pub fn new(labconfig: LabConfig) -> Genalgo<T> {
        println!("New genalgo");
        Genalgo {
            lab: Lab::new(labconfig),
            datasets: vec![],
            datasets_id: vec![],
        }
    }

    pub fn export_lab(&self) -> Result<JsonData, Errcode> {
        let _exports: LabExport = vec![];
        Err(Errcode::NotImplemented("export_lab"))
    }

    pub fn import_lab(&self, _data: JsonData) -> Result<(), Errcode> {
        Err(Errcode::NotImplemented("import_lab"))
    }

    pub fn test_function(&self) {
        println!(
            "Genalgo created for usage with cell of type \"{}\"",
            type_name::<T>()
        );
    }

    pub fn register_dataset(&mut self, id: String, dataset: Box<dyn DatasetHandler>) {
        self.datasets.push(dataset);
        self.datasets_id.push(id);
        println!("{}", self.datasets.len());
    }

    pub fn remove_dataset(&mut self, id: String) -> Result<(), Errcode> {
        let ds_ind = self
            .datasets_id
            .iter()
            .enumerate()
            .filter(|(_, i)| *i == &id)
            .map(|(n, _)| n)
            .collect::<Vec<usize>>();
        if let Some(ind) = ds_ind.get(0) {
            self.datasets_id.remove(*ind);
            self.datasets.remove(*ind);
            Ok(())
        } else {
            Err(Errcode::DatasetDoesntExist(id))
        }
    }

    /*          Public API          */
    pub fn apply_json_map(&mut self, jsdata: JsonData) -> Result<(), Errcode> {
        match serde_json::from_str::<Vec<AlgoConfiguration>>(&jsdata) {
            Ok(map) => self.lab.apply_map(map)?,
            Err(_) => return Err(Errcode::ValidationError("jsonmap")),
        }
        Ok(())
    }

    pub fn apply_map_with_algo<A: 'static + Algo<CellType=T>>(&mut self, jsdata: JsonData) -> Result<(), Errcode> {
        match serde_json::from_str::<Vec<AlgoConfiguration>>(&jsdata) {
            Ok(map) => self.lab.apply_map_with_algo::<A>(map)?,
            Err(_) => return Err(Errcode::ValidationError("jsonmap")),
        }
        Ok(())
    }

    pub fn start(&mut self, ngeneration: usize) -> Result<CellData, Errcode> {
        self.lab.start(ngeneration, &mut self.datasets)
    }

    pub fn send_special_data(
        &mut self,
        id: AlgoID,
        jsdata: JsonData,
    ) -> Result<JsonData, Errcode> {
        let data: serde_json::Value = match serde_json::from_str(&jsdata) {
            Ok(d) => d,
            Err(e) => return Err(Errcode::JsonSerializationError(e)),
        };
        self.lab.send_special_data(id, &data)
    }

    pub fn recv_special_data(&mut self, id: AlgoID, jsdata: JsonData) -> Result<(), Errcode> {
        let data: serde_json::Value = match serde_json::from_str(&jsdata) {
            Ok(d) => d,
            Err(e) => return Err(Errcode::JsonSerializationError(e)),
        };
        self.lab.recv_special_data(id, &data)
    }

    pub fn set_output_algorithm(&mut self, ind: AlgoID){
        self.lab.out_algo = Some(ind);
    }

}
