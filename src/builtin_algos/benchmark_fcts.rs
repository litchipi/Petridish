use std::cell::RefCell;
use std::mem;

use rand::prelude::*;

use crate::lab::*;
use crate::genalgo::Genalgo;
use crate::dataset::GenalgoData;
use crate::utils::{JsonData, format_error};
use crate::cell::{Genome, Cell, CellData};
use crate::algo::Algo;

use serde_json::{from_str, Value, to_string, json};

type FctScope = (i64, i64);

#[derive(Copy, Clone)]
pub enum BenchmarkFct{
    Nofct,
    Spherical(SphericalFct),
    XinSheYang1(XinSheYang1Fct),
    XinSheYang2(XinSheYang2Fct),
}

impl BenchmarkFct{
    fn calc(&mut self, data: &Genome) -> f64 {
        match self {
            BenchmarkFct::Spherical(f) => f.calc(data),
            BenchmarkFct::XinSheYang1(f) => f.calc(data),
            BenchmarkFct::XinSheYang2(f) => f.calc(data),
            _ => panic!("Function not set or recognized"),
        }
    }

    fn get_expected_optimum(&self, ndim: u8, scope: FctScope) -> Vec<f64>{
        match self {
            BenchmarkFct::Spherical(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            BenchmarkFct::XinSheYang1(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            BenchmarkFct::XinSheYang2(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            _ => panic!("Function not set or recognized"),
        }
    }

    fn get_minimum(&self) -> f64{
        match self {
            BenchmarkFct::Spherical(f) => 0.0,
            BenchmarkFct::XinSheYang1(f) => 0.0,
            BenchmarkFct::XinSheYang2(f) => 0.0,
            _ => panic!("Function not set or recognized"),
        }
    }

    fn set_scope(&mut self, scope: FctScope){
        match self {
            BenchmarkFct::Spherical(f) => f.set_scope(scope),
            BenchmarkFct::XinSheYang1(f) => f.set_scope(scope),
            BenchmarkFct::XinSheYang2(f) => f.set_scope(scope),
            _ => panic!("Function not set or recognized"),
        }
    }
}

pub fn get_fct_by_name(name: &str) -> Result<BenchmarkFct, &'static str>{
    match name{
        "spherical" => Ok(BenchmarkFct::Spherical(SphericalFct::new())),
        "xinsheyang1" => Ok(BenchmarkFct::XinSheYang1(XinSheYang1Fct::new())),
        "xinsheyang2" => Ok(BenchmarkFct::XinSheYang2(XinSheYang2Fct::new())),
        _ => Err("Benchmark function not found")
    }
}

#[derive(Clone)]
pub struct BenchmarkAlgo{
    math_fct: BenchmarkFct,
    fct_dimension: u8
}

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

    fn get_genome_length(&self) -> usize{
        self.fct_dimension as usize
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
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

    fn check_generation_over(&self, genalgo: &Lab<BenchmarkCell>) -> bool{
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
    fn genome_version_adapt(genome: &Genome, version: u64) -> Genome{
        genome.clone()
    }

    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
        let mut f = self.math_fct.borrow_mut();
        self.celldata.score = (f.get_minimum() - f.calc(&self.celldata.genome)).abs();
    }
    
    fn reset(&mut self, genome: &Genome){
        self.celldata.genome = genome.clone();
    }
}

trait MathFct{
    fn calc(&self, inputs: &Genome) -> f64;
    fn set_scope(&mut self, scope: FctScope);
}






/*              BENCHMARKING FUNCTIONS              */
//TODO Add more benchmarking functions
fn coordinates_to_gene(scope: FctScope, coordinate: f64) -> f64{
    coordinate/((scope.1 - scope.0) as f64)
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



// XinSheYang function n°1
#[derive(Copy, Clone)]
pub struct XinSheYang1Fct {scope: FctScope}
impl XinSheYang1Fct{
    fn new() -> XinSheYang1Fct{
        XinSheYang1Fct { scope : (-5, 5) }
    }
}
impl MathFct for XinSheYang1Fct{
    fn set_scope(&mut self, scope: FctScope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        let mut rng = rand::thread_rng();
        let mut res: f64 = 0.0;
        for (n, x) in inputs.iter().enumerate(){
            res += self.__gen_random(&mut rng)*coordinates_from_gene(self.scope, *x).abs().powf(n as f64)
        }
        res
    }
}

impl XinSheYang1Fct{
    fn __gen_random(&self, rng: &mut ThreadRng) -> f64{
        rng.gen()
    }
}


// XinSheYang function n°2
#[derive(Copy, Clone)]
pub struct XinSheYang2Fct {scope: FctScope}
impl XinSheYang2Fct{
    fn new() -> XinSheYang2Fct{
        XinSheYang2Fct { scope : (-5, 5) }
    }
}
impl MathFct for XinSheYang2Fct{
    fn set_scope(&mut self, scope: FctScope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, *x).abs()).sum::<f64>() * (0.0 - inputs.into_iter().map(|x| (coordinates_from_gene(self.scope, *x).powf(2.0)).sin()).sum::<f64>()).exp()
    }
}











#[test]
fn test_coordinate_transformation(){
    assert_eq!(coordinates_from_gene((-10, 10), 1.0), 10.0);
    assert_eq!(coordinates_from_gene((-10, 10), 0.0), -10.0);
    assert_eq!(coordinates_from_gene((-10, 10), 0.5), 0.0);
}

#[test]
fn test_xinsheyang2_benchmarking_fct(){
    let mut fct = match get_fct_by_name("xinsheyang2").unwrap() {
        BenchmarkFct::XinSheYang2(s) => s,
        _ => panic!("Expected XinSheYang2 function, got another one")
    };

    assert_eq!(fct.scope.0, -5);
    assert_eq!(fct.scope.1, 5);
    fct.set_scope((-10, 10));
    assert_eq!(fct.scope.0, -10);
    assert_eq!(fct.scope.1, 10);

    assert_eq!(fct.calc(&vec![0.5, 0.5]), 0.0);
    assert_eq!(fct.calc(&vec![1.0, 1.0]), fct.calc(&vec![0.0, 1.0]));
}


#[test]
fn test_xinsheyang1_benchmarking_fct(){
    let mut fct = match get_fct_by_name("xinsheyang1").unwrap() {
        BenchmarkFct::XinSheYang1(s) => s,
        _ => panic!("Expected XinSheYang1 function, got another one")
    };

    assert_eq!(fct.scope.0, -5);
    assert_eq!(fct.scope.1, 5);
    fct.set_scope((-10, 10));
    assert_eq!(fct.scope.0, -10);
    assert_eq!(fct.scope.1, 10);

    for i in 0..100{
        assert_eq!(fct.calc(&vec![0.5, 0.5]), 0.0);
        assert_ne!(fct.calc(&vec![0.0, 0.0]), fct.calc(&vec![0.0, 0.0]));
    }
}

#[test]
fn test_spherical_benchmarking_fct(){
    let mut fct = match get_fct_by_name("spherical").unwrap() {
        BenchmarkFct::Spherical(s) => s,
        _ => panic!("Expected Sperical function, got another one")
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
