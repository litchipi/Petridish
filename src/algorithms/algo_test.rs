use std::mem;
use crate::algorithms::AllCellsTypes;
use crate::genalgo_lab::genalgo::*;
use crate::genalgo_lab::JsonData;

const KEY_LIST: [&str; 2] = ["parameter1", "parameter2"];

pub struct TestAlgo{

}

pub struct TestCell{
    celldata: CellData,
}

impl Algo for TestAlgo{
    type AlgoType = TestAlgo;
    type CellType = TestCell;
    fn get_genome_length(&self) -> usize{
        2
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
        __genome_from_json(jsdata, &KEY_LIST.to_vec())
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        __genome_to_json(genome, &KEY_LIST.to_vec())
    }

    fn data_from_json(&self, jsdata: JsonData, vec: Vec<f64>){

    }

    fn create_cell_from_genome(&self, genome: &Genome) -> AllCellsTypes{
        AllCellsTypes::TestAlgoCell(TestCell {
            celldata: CellData { genome: genome.clone(), score: 0.0},
        })
    }

    fn check_generation_over(&self, genalgo: &Genalgo) -> bool{
        false
    }

    fn get_cell_size(&self) -> usize {
        mem::size_of::<TestCell>()
    }
}

impl Cell for TestCell{
    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
    }
}
