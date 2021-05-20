use crate::genalgo::{Genalgo, GenalgoConfiguration};
use crate::lab::{LabConfig, Cell};
use crate::utils::JsonData;
use crate::errors::Errcode;

use paste::paste;

pub struct PyGenalgoInterface<T: Cell>{
    genalgo: Genalgo<T>,
}

impl<T: 'static + Cell> PyGenalgoInterface<T>{
    pub fn new(labcfg_json: JsonData, cfg_json:JsonData) -> Result<PyGenalgoInterface<T>, Errcode>{
        let labcfg = LabConfig::from_json(labcfg_json)?;
        let cfg = GenalgoConfiguration::from_json(cfg_json)?;
        Ok(PyGenalgoInterface {
            genalgo: Genalgo::new(labcfg, cfg)
        })
    }
}

macro_rules! create_genalgo_type {
    ($name:ident, $celltype:expr) => {
        paste!{
            pub type [<Genalgo $name>] = PyGenalgoInterface<$celltype>;
            pub fn [<create_algo_ $name>](labcfg: JsonData, cfg: JsonData) -> [<Genalgo $name>]{
                match [<Genalgo $name>]::new(labcfg, cfg){
                    Ok(g) => g,
                    Err(e) => panic!("Not implemented"),
                }
            }
        }
    };
}

use crate::builtin_algos::algo_test::TestCell;
create_genalgo_type!(test, TestCell);
