use std::fmt;

#[derive(Debug)]
pub enum Errcode{
    NotImplemented(&'static str),
    InsuffisantPopulation(usize, usize),
    NotSet(&'static str),
    IdDoesntExist(usize),
    CodeError(&'static str),
}

/*
impl Fmt::Display for Errcode{

}*/
