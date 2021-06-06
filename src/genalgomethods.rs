use crate::cell::{Cell, CellData, Genome};
use crate::errors::Errcode;
use crate::utils::JsonData;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

mod random_opti;
mod darwin_method;

//TODO  IMPORTANT       Create genome mutation method trait and child creation trait
//          Use enum_derive to implement the trait directly
//          Use Stringified enum to configure through Python script

pub trait GenalgoMethod<T: Cell> {
    fn new() -> Self
    where
        Self: Sized;
    fn json_import(&mut self, jsdata: JsonData) -> Self
    where
        Self: Sized;
    fn load_config(&mut self, cfg: &GenalgoMethodsConfigurations);
    fn init_population(
        &mut self,
        bestgen: &Genome,
        nb_cells: u32,
        nb_elites: u32,
        res: &mut Vec<Genome>,
    ) -> Result<(), Errcode>;
    fn process_results(
        &mut self,
        elites: &Vec<&CellData>,
        cells: &Vec<CellData>,
        genomes: &mut Vec<Genome>,
    ) -> Result<(), Errcode>;
    fn reset(&mut self);

    fn validate_config(&self) -> Result<(), Errcode>;
}

#[derive(Copy, Clone, Serialize, Debug, Deserialize, EnumIter, EnumString, strum_macros::ToString)]
pub enum GenalgoMethodsAvailable {
    RandomOpti,
    Darwin,
}

impl GenalgoMethodsAvailable {
    //TODO IMPORTANT Replace with method from string to get enum
    pub fn get_by_name(name: &String) -> Option<GenalgoMethodsAvailable> {
        match name.as_str() {
            "Darwin" => Some(GenalgoMethodsAvailable::Darwin),
            "RandomOpti" => Some(GenalgoMethodsAvailable::RandomOpti),
            _ => Option::None,
        }
    }

    pub fn build<T: 'static + Cell>(&self) -> Box<dyn GenalgoMethod<T>> {
        match self {
            GenalgoMethodsAvailable::Darwin => Box::new(darwin_method::DarwinMethod::new()),
            GenalgoMethodsAvailable::RandomOpti => Box::new(random_opti::RandomOpti::new()),
        }
    }

    pub fn default() -> GenalgoMethodsAvailable {
        GenalgoMethodsAvailable::Darwin
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum GenalgoMethodsConfigurations {
    NoConfig,
    DarwinConfig(darwin_method::DarwinMethodConfiguration),
}

impl GenalgoMethodsConfigurations {
    pub fn default(method: GenalgoMethodsAvailable) -> GenalgoMethodsConfigurations {
        match method {
            GenalgoMethodsAvailable::Darwin => GenalgoMethodsConfigurations::DarwinConfig(
                darwin_method::DarwinMethodConfiguration::default(),
            ),
            GenalgoMethodsAvailable::RandomOpti => GenalgoMethodsConfigurations::NoConfig,
        }
    }

    pub fn from_str(data: JsonData) -> Result<GenalgoMethodsConfigurations, serde_json::Error> {
        serde_json::from_str(&data)
    }
}
