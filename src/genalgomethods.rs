use crate::errors::Errcode;
use crate::lab::*;
use crate::cell::{Cell, Genome, CellData};
use crate::algo::Algo;

use serde::{Serialize, Deserialize};

mod darwin_method;


pub trait GenalgoMethod<T: Cell>{
    fn new() -> Self where Self: Sized;
    fn load_config(&mut self, cfg: GenalgoMethodsConfigurations);
    fn init_method(&mut self, bestgen: &Genome, nb_cells: u32, nb_elites: u32, algo: &Box<dyn Algo<CellType = T>>, res: &mut Vec<Genome>) -> Result<(), Errcode>;
    fn process_results(&mut self, elites: &Vec<&CellData>, cells: &Vec<CellData>, algo: &Box<dyn Algo<CellType = T>>, genomes: &mut Vec<Genome>) -> Result<(), Errcode>;
    fn reset(&mut self);

    fn validate_config(&self) -> Result<(), Errcode>;
}


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GenalgoMethodsAvailable{
    Darwin
}

impl GenalgoMethodsAvailable{
    pub fn get_method<T: 'static + Cell>(&self) -> Box<dyn GenalgoMethod<T>>{
        match self {
            GenalgoMethodsAvailable::Darwin => Box::new(darwin_method::DarwinMethod::new()),
        }
    }

    pub fn default() -> GenalgoMethodsAvailable{
        GenalgoMethodsAvailable::Darwin
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GenalgoMethodsConfigurations{
    DarwinConfig(darwin_method::DarwinMethodConfiguration)
}

pub fn load_default_config(method: GenalgoMethodsAvailable) -> GenalgoMethodsConfigurations{
    match method{
        GenalgoMethodsAvailable::Darwin => GenalgoMethodsConfigurations::DarwinConfig(darwin_method::darwin_default_config()),
    }
}
