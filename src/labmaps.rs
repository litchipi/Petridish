use crate::algo::{AlgoConfiguration, AlgoPopulation};
use crate::errors::Errcode;
use crate::genalgomethods::{GenalgoMethodsAvailable, GenalgoMethodsConfigurations};
use crate::utils::{format_error, JsonData};
use crate::raise_python_error;

use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde_json::json;
mod wheel;
use wheel::WheelFormat;

struct LabMapCore {
    isolated: Vec<AlgoConfiguration>,
    mixes: Vec<AlgoConfiguration>,
    bundles: Vec<AlgoConfiguration>,
}
impl LabMapCore {
    pub fn count_algos(&self) -> usize {
        self.isolated.len() + self.mixes.len() + self.bundles.len()
    }
}

#[enum_dispatch(LabMapFormatType)]
trait LabMapFormat {
    fn generate_mixes(&self, core: &mut LabMapCore, method: GenalgoMethodsAvailable);
    fn add_objective(
        &self,
        score_opti: Vec<usize>,
        priority: f64,
        core: &mut LabMapCore,
        methods: &[GenalgoMethodsAvailable; 4],
    );
    fn setup_gives(&self, core: &mut LabMapCore);
    fn setup_objectives(&self, core: &mut LabMapCore);
}

#[enum_dispatch]
pub enum LabMapFormatType {
    WheelFormat,
}

impl LabMapFormatType {
    fn from(format_str: String) -> LabMapFormatType {
        //TODO  Use enum to string mehod
        match format_str.as_str() {
            "WheelFormat" => WheelFormat {}.into(),
            _ => unreachable!(),
        }
    }
}

#[pyclass]
pub struct LabMapAssistant {
    methods: [GenalgoMethodsAvailable; 4],
    random_opti: AlgoConfiguration,
    final_opti: AlgoConfiguration,
    final_tail: Vec<AlgoConfiguration>,
    objectives: Vec<(usize, f64)>,
    priorities: (f64, f64, f64, f64, f64), // Random, Isolated, Mix, Final, Tail
    core: LabMapCore,
    mapformat: LabMapFormatType,
}

#[pymethods]
impl LabMapAssistant {
    pub fn add_parts(&mut self, parts: Vec<(Vec<usize>, f64)>) -> usize {
        for (genes_opti, priority) in parts.iter() {
            self.create_isolated(genes_opti.clone(), *priority);
        }
        self.mapformat_actions();
        self.count_algos()
    }

    pub fn append_to_final(&mut self, js_algo_conf: JsonData) -> usize {
        match AlgoConfiguration::from_json(js_algo_conf){
            Ok(algocfg) => {
                self.final_tail.push(algocfg);
                self.mapformat_actions();
                self.count_algos()
            }, 
            Err(e) => raise_python_error!(format!("{}", e)),
        }
    }

    // Modify priorities
    pub fn set_random_source_priority(&mut self, priority: f64) {
        self.priorities.0 = priority;
    }

    pub fn set_all_isolated_opti_priority(&mut self, priority: f64) {
        self.priorities.1 = priority;
    }

    pub fn set_all_mix_opti_priority(&mut self, priority: f64) {
        self.priorities.2 = priority;
    }

    pub fn set_final_opti_priority(&mut self, priority: f64) {
        self.priorities.3 = priority;
    }

    pub fn set_all_tail_opti_priority(&mut self, priority: f64) {
        self.priorities.4 = priority;
    }

    pub fn generate(&mut self) -> JsonData {
        self.mapformat.setup_gives(&mut self.core);
        self.setup_population();
        match self.validate() {
            Ok(_) => String::from(""),      //TODO  Generate JSON Map from LabMapAssistant
            Err(e) => format_error(
                "LabMap validation failed",
                "LMV1",
                json!({ "error": format!("{}", e) }),
            ),
        }
    }
}

impl LabMapAssistant {
    pub fn new(
        format_str: String,
        iso_method: GenalgoMethodsAvailable,
        mix_method: GenalgoMethodsAvailable,
        bundle_method: GenalgoMethodsAvailable,
        final_method: GenalgoMethodsAvailable,
    ) -> LabMapAssistant {
        LabMapAssistant {
            methods: [iso_method, mix_method, bundle_method, final_method],
            random_opti: get_random_opti_algoconf(),
            final_opti: get_final_opti_algoconf(final_method),
            final_tail: vec![],
            objectives: vec![(1, 1.0)],
            core: LabMapCore {
                isolated: vec![],
                mixes: vec![],
                bundles: vec![],
            },
            priorities: (1.0, 1.0, 1.0, 1.0, 1.0),
            mapformat: LabMapFormatType::from(format_str),
        }
    }

    /*          PRIVATE METHODS         */
    fn validate(&self) -> Result<(), Errcode> {
        todo!();    //TODO  Validate LabMapAssistant configurations
        Ok(())
    }

    fn setup_population(&mut self) {
        //  Pop weight = General priority * element priority
        //TODO  Setup LabMapAssistant population
        todo!();
    }

    fn create_isolated(&mut self, genes_opti: Vec<usize>, priority: f64){
        //TODO  Create Isolated algo
        todo!();
    }

    fn count_algos(&self) -> usize {
        self.core.count_algos() + 2 + self.final_tail.len()
    }

    fn mapformat_actions(&mut self) {
        if self.core.isolated.len() > 2 {
            self.mapformat
                .generate_mixes(&mut self.core, self.methods[1]);
        }

        if self.objectives.len() > 1 {
            self.mapformat.setup_objectives(&mut self.core);
        }
    }
}

pub fn get_random_opti_algoconf() -> AlgoConfiguration {
    AlgoConfiguration {
        method: "RandomOpti".to_string(),
        method_options: GenalgoMethodsConfigurations::NoConfig,
        give: vec![],
        impr_genes: Option::None,
        population: AlgoPopulation::WeightofTot(1.0),
    }
}

pub fn get_final_opti_algoconf(method: GenalgoMethodsAvailable) -> AlgoConfiguration {
    AlgoConfiguration {
        method: method.to_string(),
        method_options: GenalgoMethodsConfigurations::default(method),
        give: vec![],
        impr_genes: Option::None,
        population: AlgoPopulation::WeightofTot(1.0),
    }
}
