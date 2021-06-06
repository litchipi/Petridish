use crate::algo::{AlgoConfiguration, AlgoPopulation};
use crate::errors::Errcode;
use crate::genalgomethods::GenalgoMethodsConfigurations;
use crate::utils::JsonData;
use crate::*;

use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
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

#[pyclass]
pub struct LabMapAssistant {
    random_opti: AlgoConfiguration,
    iso_algos: Vec<AlgoConfiguration>,
    final_tail: Vec<AlgoConfiguration>,

    priorities: [f64; 3], // Random, Tree, Final
    mapformat: LabMapFormatType,
}

#[pymethods]
impl LabMapAssistant {
    #[staticmethod]
    pub fn new(format_str: String) -> LabMapAssistant {
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
        method: String, method_cfg: JsonData){
        self.iso_algos.push(AlgoConfiguration { 
            id, method,
            method_options: py_err_if_fail!(
                GenalgoMethodsConfigurations::from_str(method_cfg),
                "GenalgoMethodsConfigurations load failed"),
            give: vec![],
            impr_genes: Some(genes_opt),
            population: AlgoPopulation::WeightofTot(priority*self.priorities[1])
        });
    }

    pub fn generate_map(&self, mix_method: String) -> JsonData{
        // AlgosID:     <Random opti> <Final tail> <Map>
        py_err_if_fail!(self.mapformat.generate_map(&self.iso_algos, &self.random_opti,
            &self.final_tail, &self.priorities, mix_method), "Map generation failed")
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
