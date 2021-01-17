use crate::genalgo::GenalgoMethod;
use serde::{Serialize, Deserialize};

mod darwin_method;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum GenalgoMethodsAvailable{
    Darwin
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub (crate) enum GenalgoMethodsConfigurations{
    DarwinConfig(darwin_method::DarwinMethodConfiguration)
}

pub (crate) enum AllGenalgoMethod{
    Darwin(darwin_method::DarwinMethod)
}

impl AllGenalgoMethod{
    pub (crate) fn unwrap(&mut self) -> &mut impl GenalgoMethod{
        match self{
            AllGenalgoMethod::Darwin(m) => m,
        }
    }
}

pub (crate) fn get_method(method: GenalgoMethodsAvailable) -> AllGenalgoMethod{
    match method{
        GenalgoMethodsAvailable::Darwin => AllGenalgoMethod::Darwin(darwin_method::new_darwin_method()),
    }
}


pub (crate) fn load_default_config(method: GenalgoMethodsAvailable) -> GenalgoMethodsConfigurations{
    match method{
        GenalgoMethodsAvailable::Darwin => GenalgoMethodsConfigurations::DarwinConfig(darwin_method::darwin_default_config()),
    }
}
