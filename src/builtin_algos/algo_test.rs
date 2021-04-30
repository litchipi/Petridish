use std::mem;
use crate::genalgo::*;
use crate::utils::JsonData;
const KEY_LIST: [&str; 2] = ["parameter1", "parameter2"];

#[derive(Clone)]
pub struct TestAlgo{

}

impl Algo for TestAlgo{
    type CellType = TestCell;

    fn new() -> Self where Self: Sized{
        TestAlgo { }
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
            celldata: CellData { genome: genome.clone(), score: 0.0},
        }
    }

    fn check_generation_over(&self, genalgo: &Genalgo<TestCell>) -> bool{
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
    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
    }
}
