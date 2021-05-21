use std::fmt;

#[derive(Debug)]
pub enum Errcode{
    NotImplemented(&'static str),
    InsuffisantPopulation(usize, usize),
    NotSet(&'static str),
    IdDoesntExist(usize),
    CodeError(&'static str),
    DatasetDoesntExist(String),
    ValidationError(&'static str),
}

impl fmt::Display for Errcode{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Errcode::NotSet(el) => write!(f, "Element \"{}\" not set", el),
            Errcode::ValidationError(el) => write!(f, "Error while validating element \"{}\"", el),
            _ =>  write!(f, "{:?}", self),
        }
    }
}
