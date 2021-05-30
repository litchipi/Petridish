use std::fmt;
use serde_json::Error;

use crate::utils::JsonData;

#[derive(Debug)]
pub enum Errcode{
    NotImplemented(&'static str),
    InsuffisantPopulation(usize, usize),
    NotSet(&'static str),
    IdDoesntExist(usize),
    CodeError(&'static str),
    DatasetDoesntExist(String),
    SizeError(&'static str, usize, usize),        // Expected, Got
    ValidationError(&'static str),
    JsonSerializationError(Error),
    SpecialDataError(JsonData),
}

impl fmt::Display for Errcode{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Errcode::NotSet(el) => write!(f, "Element \"{}\" not set", el),
            Errcode::SizeError(id, exp, got) => write!(f, "Size error for element \"{}\", expected {} got {}", id, exp, got),
            Errcode::ValidationError(el) => write!(f, "Error while validating element \"{}\"", el),
            Errcode::SpecialDataError(d) => write!(f, "Error with special data handling: {}", d),
            _ =>  write!(f, "{:?}", self),
        }
    }
}
