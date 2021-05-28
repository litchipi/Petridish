use std::cell::RefCell;

use petridish::utils::format_error;
use petridish::generate_py_ifaces;
use petridish::cell::*;
use petridish::lab::Lab;
use petridish::dataset::GenalgoData;

mod fcts;
use fcts::*;

use serde_json::json;

#[derive(Clone)]
pub struct BenchmarkCell{
    celldata: CellData,
    math_fct: RefCell<BenchmarkFct>
}

impl BenchmarkCell{
    fn set_math_fct(&mut self, fct: BenchmarkFct){
        self.math_fct = RefCell::new(fct);
    }
}

#[derive(Clone)]
pub struct BenchmarkAlgo{
    math_fct: BenchmarkFct,
    fct_dimension: u8
}

impl BenchmarkAlgo{
    fn __get_expected_optimum(&self, params: &serde_json::Value) -> JsonData{
        if params.get("scope") == Option::None{ //) || (params.get("scope_max") == Option::None){
            format_error("Please specify scope field", "BSDExO1", json!({}))
        }else{
            serde_json::to_string(&json!({
                "result":self.math_fct.get_expected_optimum(self.fct_dimension, (
                        params["scope"][0].as_i64().expect("Cannot convert scope min as i64"),
                        params["scope"][1].as_i64().expect("Cannot convert scope max as i64")
                        ))
            })).expect("Cannot convert results to JSON string")
        }
    }
}

impl Algo for BenchmarkAlgo{
    type CellType = BenchmarkCell;
    
    //TODO init with null function (to be configured after)
    fn new() -> Self where Self : Sized {
        BenchmarkAlgo {
            fct_dimension: 0, //fct_dimension,
            math_fct: BenchmarkFct::Nofct //fct
        }
    }

    fn reset(&mut self){

    }

    fn send_special_data(&self, params: &serde_json::Value) -> JsonData{
        if params.get("method") == Option::None{
            format_error("Please specify method name", "BSD1", json!({}));
        }
        match params["method"].as_str().expect("Cannot unwrap method as str") {
            "expected_optimum" => self.__get_expected_optimum(params),
            _ => format_error("method not recognized", "BSD2", json!(["expected_optimum"]))
        }
    }

    fn recv_special_data(&mut self, data: &serde_json::Value){
        if data.get("scope") != Option::None{
            self.math_fct.set_scope((data["scope"][0].as_i64().expect("Unable to load scope value 0"), data["scope"][1].as_i64().expect("Unable to load scope value 1")));
        }

        if data.get("mathfct") != Option::None{
            self.math_fct = get_fct_by_name(data["mathfct"].as_str().unwrap()).unwrap();
        }

        if data.get("nb_dimensions") != Option::None{
            self.fct_dimension = data["nb_dimensions"].as_u64().unwrap() as u8;
        }
    }

    fn genome_from_json(&self, _jsdata: JsonData) -> Genome{
        Genome::new()
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        serde_json::to_string(&genome).unwrap()
    }

    fn create_cell_from_genome(&self, genome: &Genome) -> Self::CellType{
        BenchmarkCell {
            celldata: CellData { genome: genome.clone(), score: 0.0, version:1},
            math_fct: RefCell::new(self.math_fct)
        }
    }

    fn check_generation_over(&self, _genalgo: &Lab<BenchmarkCell>) -> bool{
        true
    }

    fn initialize_cells(&mut self, pop: &mut Vec<Self::CellType>){
        if let BenchmarkFct::Nofct = self.math_fct {
            panic!("No math function was set up before initialisation");
        }
        for cell in pop.iter_mut(){
            cell.set_math_fct(self.math_fct);
        }
    }

    fn process_data(&mut self, pop: &mut Vec<Self::CellType>, data: &GenalgoData){
        for cell in pop.iter_mut(){
            cell.action(&data)
        }
    }
}

impl Cell for BenchmarkCell{
    fn get_genome_length() -> usize{
        0   //TODO  IMPORTANT get_genome_length
    }

    fn genome_version_adapt(genome: &Genome, _version: u64) -> Genome{
        genome.clone()
    }

    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, _data: &GenalgoData){
        let mut f = self.math_fct.borrow_mut();
        self.celldata.score = (f.get_minimum() - f.calc(&self.celldata.genome)).abs();
    }
    
    fn reset(&mut self, genome: &Genome){
        self.celldata.genome = genome.clone();
    }
}

generate_py_ifaces!(petridish,
    //TODO  One algo for one math function -> macro generated
    [benchmark] BenchmarkCell => (benchmark => BenchmarkAlgo),
);
