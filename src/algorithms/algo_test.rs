use crate::algorithms::AllCellsTypes;
use crate::genalgo_lab::genalgo::*;
use crate::genalgo_lab::JsonData;

pub struct TestAlgo{

}

pub struct TestCell{
    genome: Genome
}

impl Algo for TestAlgo{
    fn get_genome_length(&self) -> usize{
        2
    }

    fn import_genome_from_json(jsdata: JsonData) -> Genome{

        Genome::new()   // TODO Import Genome from Json
    }

    fn export_genome_to_json(genome: Genome) -> JsonData{
        String::new()
    }

    fn data_from_json(jsdata: JsonData, vec: Vec<f64>){

    }

    fn create_cell_from_genome(&self, genome: &Genome) -> AllCellsTypes{
        AllCellsTypes::TestAlgoCell(TestCell {genome: genome.clone()})
    }

}

impl Cell for TestCell{
    fn get_score(&self) -> Score{
        0
    }

    fn action(&self, data: Vec<f64>){
    }
}
