use crate::genalgo::*;
use serde::{Serialize, Deserialize};

mod darwin_method;


pub (crate) trait GenalgoMethod<T: Cell>{
    fn new() -> Self where Self: Sized;
    fn load_config(&mut self, cfg: &GenalgoConfiguration, set: &GenalgoSettings);
    fn init_method(&mut self, bestcell: &CellData, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>;
    fn process_results(&mut self, maximize: bool, cells: Vec<&CellData>, var: &GenalgoVardata, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>;
    fn reset(&mut self);
}


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GenalgoMethodsAvailable{
    Darwin
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub (crate) enum GenalgoMethodsConfigurations{
    DarwinConfig(darwin_method::DarwinMethodConfiguration)
}

pub (crate) fn get_method<T: 'static + Cell>(method: GenalgoMethodsAvailable) -> Box<dyn GenalgoMethod<T>>{
    match method{
        GenalgoMethodsAvailable::Darwin => Box::new(darwin_method::DarwinMethod::new()),
    }
}


pub (crate) fn load_default_config(method: GenalgoMethodsAvailable) -> GenalgoMethodsConfigurations{
    match method{
        GenalgoMethodsAvailable::Darwin => GenalgoMethodsConfigurations::DarwinConfig(darwin_method::darwin_default_config()),
    }
}
