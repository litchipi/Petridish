#![allow(unused_variables)]

use petridish::generate_py_ifaces;
use petridish::cell::*;
use petridish::lab::Lab;
use petridish::dataset::GenalgoData;

use serde_json;

const KEY_LIST: [&str; 2] = ["parameter1", "parameter2"];

#[derive(Clone)]
pub struct TestAlgoA{

}

impl Algo for TestAlgoA{
    type CellType = TestCell;

    fn new() -> Self where Self: Sized{
        TestAlgoA { }
    }

    fn reset(&mut self){

    }

    fn send_special_data(&self, params: &serde_json::Value) -> JsonData{
        String::from("")
    }

    fn recv_special_data(&mut self, data: &serde_json::Value){

    }

    fn get_genome_length(&self) -> usize{
        2
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
        __genome_from_json(jsdata, &KEY_LIST.to_vec())
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        __genome_to_json(genome, &KEY_LIST.to_vec())
    }

    fn create_cell_from_genome(&self, genome: &Genome) -> Self::CellType{
        TestCell {
            celldata: CellData { genome: genome.clone(), score: 0.0, version: 1},
        }
    }

    fn check_generation_over(&self, genalgo: &Lab<TestCell>) -> bool{
        false
    }

    fn initialize_cells(&mut self, pop: &mut Vec<Self::CellType>){

    }

    fn process_data(&mut self, pop: &mut Vec<Self::CellType>, data: &GenalgoData){
        for cell in pop.iter_mut(){
            cell.action(&data);
        }
    }
}

pub struct TestAlgoB{

}

impl Algo for TestAlgoB{
    type CellType = TestCell;

    fn new() -> Self where Self: Sized{
        TestAlgoB { }
    }

    fn reset(&mut self){

    }

    fn send_special_data(&self, params: &serde_json::Value) -> JsonData{
        String::from("")
    }

    fn recv_special_data(&mut self, data: &serde_json::Value){

    }

    fn get_genome_length(&self) -> usize{
        2
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
        __genome_from_json(jsdata, &KEY_LIST.to_vec())
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        __genome_to_json(genome, &KEY_LIST.to_vec())
    }

    fn create_cell_from_genome(&self, genome: &Genome) -> Self::CellType{
        TestCell {
            celldata: CellData { genome: genome.clone(), score: 0.0, version: 1},
        }
    }

    fn check_generation_over(&self, genalgo: &Lab<TestCell>) -> bool{
        false
    }

    fn initialize_cells(&mut self, pop: &mut Vec<Self::CellType>){

    }

    fn process_data(&mut self, pop: &mut Vec<Self::CellType>, data: &GenalgoData){
        for cell in pop.iter_mut(){
            cell.action(&data);
        }
    }
}

#[derive(Clone)]
pub struct TestCell{
    celldata: CellData,
}

impl Cell for TestCell{
    fn genome_version_adapt(genome: &Genome, version: u64) -> Genome{
        genome.clone()
    }

    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
    }

    fn reset(&mut self, genome: &Genome){
        self.celldata.genome = genome.clone();
    }
}

generate_py_ifaces!(petridish,
    [test] TestCell => (A => TestAlgoA, B => TestAlgoB),
);
