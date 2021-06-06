use crate::genalgomethods::{GenalgoMethod, GenalgoMethodsConfigurations};
use crate::cell::{Genome, Cell, CellData, random_genome};
use crate::errors::Errcode;
use crate::utils::JsonData;

use std::marker::PhantomData;

pub struct RandomOpti<T: Cell>{
    _phantom: PhantomData<T>,
}

impl<T: Cell> GenalgoMethod<T> for RandomOpti<T> {
    fn new() -> Self where Self: Sized{
        RandomOpti { _phantom: PhantomData }
    }

    fn json_import(&mut self, _jsdata: JsonData) -> Self where Self: Sized{
        todo!();
    }

    fn load_config(&mut self, _cfg: &GenalgoMethodsConfigurations){}


    fn init_population(
        &mut self,
        bestgen: &Genome,
        nb_cells: u32,
        _nb_elites: u32,
        res: &mut Vec<Genome>,
    ) -> Result<(), Errcode>{
        for _ in 0..nb_cells{
            res.push(random_genome(bestgen.len()));
        }
        Ok(())
    }

    fn process_results(
        &mut self,
        _elites: &Vec<&CellData>,
        cells: &Vec<CellData>,
        genomes: &mut Vec<Genome>,
    ) -> Result<(), Errcode>{
        let ngens = cells[0].genome.len();
        for _ in 0..cells.len(){
            genomes.push(random_genome(ngens));
        }
        Ok(())
    }

    fn reset(&mut self) {}

    fn validate_config(&self) -> Result<(), Errcode> { Ok(()) }
}
