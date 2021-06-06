use crate::algo::{AlgoConfiguration, AlgoPopulation};
use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethodsAvailable, GenalgoMethodsConfigurations};
use crate::utils::{format_error, JsonData};
use crate::*;

use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde_json::json;
mod wheel;
use wheel::WheelFormat;

#[enum_dispatch(LabMapFormatType)]
trait LabMapFormat {
    fn generate_map(&self, iso_algos: &Vec<AlgoConfiguration>,
        random_opti: &AlgoConfiguration, final_tail: &Vec<AlgoConfiguration>,
        priorities: &[f64; 3], mix_method: String) -> Result<JsonData, Errcode>;

}

#[enum_dispatch]
pub enum LabMapFormatType {
    WheelFormat,
}

impl LabMapFormatType {
    fn from(format_str: String) -> LabMapFormatType {
        //TODO  Use enum to string mehod
        match format_str.as_str() {
            "WheelFormat" => WheelFormat::new().into(),
            _ => unreachable!(),
        }
    }
}

pub struct LabMapAssistant {
    random_opti: AlgoConfiguration,
    iso_algos: Vec<AlgoConfiguration>,
    final_tail: Vec<AlgoConfiguration>,

    priorities: [f64; 3], // Random, Tree, Final
    mapformat: LabMapFormatType,
}

impl LabMapAssistant {
    pub fn new(format_str: String, final_method: String) -> LabMapAssistant {
        LabMapAssistant {
            random_opti: get_random_opti_algoconf(),
            iso_algos: vec![],
            final_tail: vec![],
            mapformat: LabMapFormatType::from(format_str),
            //          rand  map final
            priorities: [1.0; 3],
        }
    }

    pub fn add_opti_part(&mut self, id: String, genes_opt: Vec<usize>, priority: f64,
        method: String, method_cfg: JsonData) -> Result<(), Errcode>{
        self.iso_algos.push(AlgoConfiguration { 
            id, method,
            method,
            method_options: GenalgoMethodsConfigurations::from_str(method_cfg)?,
            give: vec![],
            impr_genes: Some(genes_opt),
            population: AlgoPopulation::WeightofTot(priority+self.priorities[1])
        });
        Ok(())
    }

    pub fn generate_map(&self, mix_method: String) -> Result<JsonData, Errcode>{
        // AlgosID:     <Random opti> <Final tail> <Map>
        self.mapformat.generate_map(&self.iso_algos, &self.random_opti,
            &self.final_tail, &self.priorities, mix_method)
    }
}

pub fn get_random_opti_algoconf() -> AlgoConfiguration {
    AlgoConfiguration {
        id: "Random".to_string(),
        method: "RandomOpti".to_string(),
        method_options: GenalgoMethodsConfigurations::NoConfig,
        give: vec![],
        impr_genes: Option::None,
        population: AlgoPopulation::WeightofTot(1.0),
    }
}
