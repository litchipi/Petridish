use crate::errors::Errcode;
use crate::cell::{Cell, Genome, CellData};

use serde::{Serialize, Deserialize};

mod darwin_method;

//TODO  Modify so method are usable by any algo at any moment (do not store algo-specific data)
//          Only provide a set of functions usefull for genalgo
pub trait GenalgoMethod<T: Cell>{
    fn new() -> Self where Self: Sized;
    //TODO  Import from JsonData
    fn load_config(&mut self, cfg: GenalgoMethodsConfigurations);
    fn init_method(&mut self, bestgen: &Genome, nb_cells: u32, nb_elites: u32, res: &mut Vec<Genome>) -> Result<(), Errcode>;
    fn process_results(&mut self, elites: &Vec<&CellData>, cells: &Vec<CellData>, genomes: &mut Vec<Genome>) -> Result<(), Errcode>;
    fn reset(&mut self);

    fn validate_config(&self) -> Result<(), Errcode>;
}


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GenalgoMethodsAvailable{
    Darwin
}

impl GenalgoMethodsAvailable{
    //TODO  Get method by name
    //TODO  Rename to "build"
    pub fn build<T: 'static + Cell>(&self) -> Box<dyn GenalgoMethod<T>>{
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
