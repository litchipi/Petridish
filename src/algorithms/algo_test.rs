use std::mem;
use crate::algorithms::AllCellsTypes;
use crate::genalgo::*;
use crate::utils::JsonData;
const KEY_LIST: [&str; 2] = ["parameter1", "parameter2"];

pub struct TestAlgo{

}

pub struct TestCell{
    celldata: CellData,
}

impl Algo for TestAlgo{

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


    fn initialize_cells(&mut self, pop: &mut Vec<AllCellsTypes>){

    }

    fn perform_action_on_data(&mut self, pop: &mut Vec<AllCellsTypes>, data: &GenalgoData){
        for cell in pop.iter_mut(){
            if let AllCellsTypes::TestAlgoCell(c) = cell {
                c.action(&data)
            }
            cell.unwrap_mut().action(&data);
        }
    }
}

impl Cell for TestCell{
    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
    }
}
