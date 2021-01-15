use std::mem;

use crate::algorithms::AllCellsTypes;

use crate::genalgo_lab::genalgo::*;
use crate::genalgo_lab::JsonData;

type FctScope = (i32, i32);

pub enum BenchmarkFct{
    Spherical(SphericalFct)
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
}

impl BenchmarkAlgo{
    pub fn new(fct: BenchmarkFct, fct_dimension: u8) -> BenchmarkAlgo{
        BenchmarkAlgo {
            fct_dimension: fct_dimension,
            math_fct: fct
        }
    }
}

impl Algo for BenchmarkAlgo{

    fn get_genome_length(&self) -> usize{
        self.fct_dimension as usize
    }

    fn genome_from_json(&self, jsdata: JsonData) -> Genome{
        Genome::new()
    }

    fn genome_to_json(&self, genome: Genome) -> JsonData{
        String::from("")
    }

    fn data_from_json(&self, jsdata: JsonData, vec: Vec<f64>){

    }

    fn create_cell_from_genome(&self, genome: &Genome) -> AllCellsTypes{
        AllCellsTypes::BenchmarkAlgoCell(BenchmarkCell {
            celldata: CellData { genome: genome.clone(), score: 0.0},
        })
    }

    fn check_generation_over(&self, genalgo: &Genalgo) -> bool{
        false
    }

    fn get_cell_size(&self) -> usize {
        mem::size_of::<BenchmarkCell>()
    }
}

impl Cell for BenchmarkCell{
    fn get_data(&self) -> &CellData{
        &self.celldata
    }

    fn action(&mut self, data: &GenalgoData){
    }
}

trait MathFct{
    fn calc(&self, inputs: Vec<f64>) -> f64;
    fn set_scope(&mut self, scope: FctScope);
}






/*              BENCHMARKING FUNCTIONS              */

fn coordinates_from_gene(scope: FctScope, gene: f64) -> f64{
    assert!(scope.0 < scope.1);
    (scope.0 as f64) + (((scope.1-scope.0) as f64)*gene)
}

//SPHERICAL
pub struct SphericalFct { scope: FctScope}
impl SphericalFct{
    fn new() -> SphericalFct{
        SphericalFct { scope : (-5, 5) }
    }
}
impl MathFct for SphericalFct{
    fn set_scope(&mut self, scope: FctScope){
        self.scope = scope;
    }

    fn calc(&self, inputs: Vec<f64>) -> f64{
        println!("{:?} {:?} {:?}", inputs, self.scope, coordinates_from_gene(self.scope, inputs[0]));
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, x).powf(2.0)).collect::<Vec<f64>>().iter().sum()
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
    assert_eq!(fct.calc(vec![1.0, 1.0]), 200.0);
    assert_eq!(fct.calc(vec![1.0, 0.0]), 200.0);
    assert_eq!(fct.calc(vec![0.0, 1.0]), 200.0);
    assert_eq!(fct.calc(vec![0.0, 0.0]), 200.0);
    assert_eq!(fct.calc(vec![0.5, 0.5]), 0.0);
    assert_eq!(fct.calc(vec![1.0, 0.5]), 100.0);
}
