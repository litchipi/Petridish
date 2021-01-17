use std::cell::RefCell;
use std::mem;

use crate::algorithms::AllCellsTypes;
use crate::genalgo::*;
use crate::utils::{JsonData, format_error};
use serde_json::*;

type FctScope = (i64, i64);

#[derive(Copy, Clone)]
pub enum BenchmarkFct{
    Spherical(SphericalFct)
}

impl BenchmarkFct{
    fn calc(&mut self, data: &Genome) -> f64 {
        match self {
            BenchmarkFct::Spherical(f) => f.calc(data),
        }
    }

    fn get_expected_optimum(&self, ndim: u8, scope: FctScope) -> Vec<f64>{
        match self {
            BenchmarkFct::Spherical(f) => {
                let mut res = vec![];
                for i in 0..ndim{
                    res.push(coordinates_to_gene(scope, 0.0));
                };
                res
            },
        }
    }

    fn get_minimum(&self) -> f64{
        match self {
            BenchmarkFct::Spherical(f) => 0.0
        }
    }

    fn set_scope(&mut self, scope: FctScope){
        match self {
            BenchmarkFct::Spherical(f) => f.set_scope(scope)
        }
    }
}

pub fn get_fct_by_name(name: &str) -> BenchmarkFct{
    match name{
        "spherical" => BenchmarkFct::Spherical(SphericalFct::new()),
        _ => panic!("Benchmark function not found")
    }
}

pub struct BenchmarkAlgo{
    math_fct: BenchmarkFct,
    fct_dimension: u8
}

pub struct BenchmarkCell{
    celldata: CellData,
    math_fct: RefCell<BenchmarkFct>
}

impl BenchmarkCell{
    fn set_math_fct(&mut self, fct: BenchmarkFct){
        self.math_fct = RefCell::new(fct);
    }
}

impl BenchmarkAlgo{
    //TODO init with null function (to be configured after)
    pub fn new(fct: BenchmarkFct, fct_dimension: u8) -> BenchmarkAlgo{
        BenchmarkAlgo {
            fct_dimension: fct_dimension,
            math_fct: fct
        }
    }

    fn __get_expected_optimum(&self, params: &serde_json::Value) -> JsonData{
        if (params.get("scope_min") == Option::None) || (params.get("scope_max") == Option::None){
            format_error("Please specify scope_min and scope_max fields", "BSDExO1", json!({}))
        }else{
            serde_json::to_string(&json!({
                "result":self.math_fct.get_expected_optimum(self.fct_dimension, (
                        params["scope_min"].as_i64().expect("Cannot convert scope_min as i64"),
                        params["scope_max"].as_i64().expect("Cannot convert scope_max as i64")
                        ))
            })).expect("Cannot convert results to JSON string")
        }
    }

}

impl Algo for BenchmarkAlgo{
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

    //TODO change math function and dimension
    fn recv_special_data(&mut self, data: &serde_json::Value){
        if data.get("scope") != Option::None{
            self.math_fct.set_scope((data["scope"][0].as_i64().expect("Unable to load scope value 0"), data["scope"][1].as_i64().expect("Unable to load scope value 1")));
        }
    }

    fn get_genome_length(&self) -> usize{
        self.fct_dimension as usize
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
        Genome::new()
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        serde_json::to_string(&genome).unwrap()
    }

    fn data_from_json(&self, jsdata: JsonData, vec: Vec<f64>){

    }

    fn create_cell_from_genome(&self, genome: &Genome) -> AllCellsTypes{
        AllCellsTypes::BenchmarkAlgoCell(BenchmarkCell {
            celldata: CellData { genome: genome.clone(), score: 0.0},
            math_fct: RefCell::new(self.math_fct)
        })
    }

    fn check_generation_over(&self, genalgo: &Genalgo) -> bool{
        true
    }

    fn get_cell_size(&self) -> usize {
        mem::size_of::<BenchmarkCell>()
    }

    fn initialize_cells(&mut self, pop: &mut Vec<AllCellsTypes>){
        for cell in pop.iter_mut(){
            if let AllCellsTypes::BenchmarkAlgoCell(c) = cell {
                c.set_math_fct(self.math_fct);
            }
        }
    }

    fn perform_action_on_data(&mut self, pop: &mut Vec<AllCellsTypes>, data: &GenalgoData){
        for cell in pop.iter_mut(){
            if let AllCellsTypes::BenchmarkAlgoCell(c) = cell {
                c.action(&data)
            }
        }
    }
}

impl Cell for BenchmarkCell{
    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
        let mut f = self.math_fct.borrow_mut();
        self.celldata.score = (f.get_minimum() - f.calc(&self.celldata.genome)).abs();
    }
}

trait MathFct{
    fn calc(&self, inputs: &Genome) -> f64;
    fn set_scope(&mut self, scope: FctScope);
}






/*              BENCHMARKING FUNCTIONS              */
//TODO Add more benchmarking functions
fn coordinates_to_gene(scope: FctScope, gene: f64) -> f64{
    0.5 //TODO Coordinates to gene calculation
}

fn coordinates_from_gene(scope: FctScope, gene: f64) -> f64{
    assert!(scope.0 < scope.1);
    (scope.0 as f64) + (((scope.1-scope.0) as f64)*gene)
}

//SPHERICAL
#[derive(Copy, Clone)]
pub struct SphericalFct {scope: FctScope}
impl SphericalFct{
    fn new() -> SphericalFct{
        SphericalFct { scope : (-5, 5) }
    }
}
impl MathFct for SphericalFct{
    fn set_scope(&mut self, scope: FctScope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, *x).powf(2.0)).collect::<Vec<f64>>().iter().sum()
    }
}

#[test]
fn test_coordinate_transformation(){
    assert_eq!(coordinates_from_gene((-10, 10), 1.0), 10.0);
    assert_eq!(coordinates_from_gene((-10, 10), 0.0), -10.0);
    assert_eq!(coordinates_from_gene((-10, 10), 0.5), 0.0);
}

#[test]
fn test_spherical_benchmarking_fct(){
    let mut fct = match get_fct_by_name("spherical") {
        BenchmarkFct::Spherical(s) => s,
    };
    assert_eq!(fct.scope.0, -5);
    assert_eq!(fct.scope.1, 5);
    fct.set_scope((-10, 10));
    assert_eq!(fct.scope.0, -10);
    assert_eq!(fct.scope.1, 10);
    assert_eq!(fct.calc(&vec![1.0, 1.0]), 200.0);
    assert_eq!(fct.calc(&vec![1.0, 0.0]), 200.0);
    assert_eq!(fct.calc(&vec![0.0, 1.0]), 200.0);
    assert_eq!(fct.calc(&vec![0.0, 0.0]), 200.0);
    assert_eq!(fct.calc(&vec![0.5, 0.5]), 0.0);
    assert_eq!(fct.calc(&vec![1.0, 0.5]), 100.0);
}
