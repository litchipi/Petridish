use petridish::cell::Genome;
use rand::prelude::*;
use rand_pcg::Pcg64;

use enum_dispatch::enum_dispatch;

pub type Scope = (i64, i64);

/*              BENCHMARKING FUNCTIONS              */
fn coordinates_to_gene(scope: Scope, coordinate: f64) -> f64{
    assert!(scope.0 < scope.1);
    assert!(coordinate >= (scope.0 as f64));
    assert!(coordinate <= (scope.1 as f64));
    (coordinate - (scope.0 as f64))/((scope.1 - scope.0) as f64)
}

fn coordinates_from_gene(scope: Scope, gene: f64) -> f64{
    assert!(scope.0 < scope.1);
    (scope.0 as f64) + (((scope.1-scope.0) as f64)*gene)
}

//TODO  Automate build from stringified struct name
pub fn build_fct(name: &str, scope: Scope) -> Result<BenchmarkFct, &'static str>{
    match name{
        "spherical" => Ok(Spherical { scope }.into()), //Ok(BenchmarkFct::from(Spherical { scope })),
        "xinsheyang1" => Ok(XinSheYang1 { scope }.into()), //Ok(BenchmarkFct::from(XinSheYang1 { scope })),
        "xinsheyang2" => Ok(XinSheYang2 { scope }.into()), //Ok(BenchmarkFct::from(XinSheYang2 { scope })),
        "schwefel220" => Ok(Schwefel220 { scope }.into()),
        "styblinski_tank" => Ok(StyblinskiTank { scope }.into()),
        "quartic" => Ok(Quartic { scope }.into()),
        _ => Err("BenchmarkFct function not found")
    }
}


#[enum_dispatch(BenchmarkFct)]
pub trait MathFct{
    fn calc(&self, inputs: &Genome) -> f64;
    fn set_scope(&mut self, scope: Scope);
    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>;
    fn get_minimum(&self, ndim: u8) -> f64;
}

//TODO Add more benchmarking functions
#[enum_dispatch]
#[derive(Copy, Clone, Debug)]
pub enum BenchmarkFct{
    Spherical,
    XinSheYang1,
    XinSheYang2,
    Schwefel220,
    StyblinskiTank,
    Quartic,
}

impl BenchmarkFct{
    pub fn default() -> BenchmarkFct{
        build_fct("spherical", (-5, 5)).unwrap()
    }
}








//SPHERICAL
#[derive(Copy, Clone, Debug)]
pub struct Spherical {scope: Scope}

impl MathFct for Spherical{
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, *x).powf(2.0)).collect::<Vec<f64>>().iter().sum()
    }
    

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }
    fn get_minimum(&self, ndim: u8) -> f64{
        0.0
    }
}



// XinSheYang function n°1
#[derive(Copy, Clone, Debug)]
pub struct XinSheYang1 {scope: Scope}
impl MathFct for XinSheYang1{
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        let mut res: f64 = 0.0;
        let mut rng = Pcg64::from_entropy(); //seed_from_u64((x*100000000.0) as u64);
        for (n, x) in inputs.iter().enumerate(){
            res += rng.gen::<f64>()*coordinates_from_gene(self.scope, *x).abs().powf((n+1) as f64);
        }
        res
    }

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        0.0
    }
}



// XinSheYang function n°2
#[derive(Copy, Clone, Debug)]
pub struct XinSheYang2 {scope: Scope}
impl MathFct for XinSheYang2{
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, *x).abs()).sum::<f64>() * (0.0 - inputs.into_iter().map(|x| (coordinates_from_gene(self.scope, *x).powf(2.0)).sin()).sum::<f64>()).exp()
    }

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        0.0
    }
}



// Schwefel220
#[derive(Copy, Clone, Debug)]
pub struct Schwefel220 {scope: Scope}
impl MathFct for Schwefel220{
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        inputs.into_iter().map(|x| coordinates_from_gene(self.scope, *x).abs()).sum()
    }

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        0.0
    }
}



// StyblinskiTank
#[derive(Copy, Clone, Debug)]
pub struct StyblinskiTank {scope: Scope}
impl MathFct for StyblinskiTank{
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        let mut scores = 0.0;
        for g in inputs.iter(){
            let x = coordinates_from_gene(self.scope, *g);
            scores += x.powf(4.0) - (16.0*x.powf(2.0)) + (5.0*x);
        }
        scores / 2.0
    }

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, -2.903534); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        -39.16599*(ndim as f64)
    }
}

// Quartic
#[derive(Copy, Clone, Debug)]
pub struct Quartic {scope: Scope}
impl MathFct for Quartic {
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        let mut res: f64 = 0.0;
        let mut rng = Pcg64::seed_from_u64(inputs.into_iter().map(|x| (x*10000000.0) as u64).sum());
        for (n, x) in inputs.iter().enumerate(){
            res += ((n+1) as f64)*coordinates_from_gene(self.scope, *x).powf(4.0);
        }
        let rand_nb = rng.gen::<f64>();
        res + rand_nb
    }
    
    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        let mut rng = Pcg64::seed_from_u64(0);
        rng.gen::<f64>()
    }
}

/*  TEMPLATE

// Name
#[derive(Copy, Clone, Debug)]
pub struct Name {scope: Scope}
impl MathFct for Name {
    fn set_scope(&mut self, scope: Scope){
        self.scope = scope;
    }

    fn calc(&self, inputs: &Genome) -> f64{
        0.0
    }

    fn get_expected_optimum(&self, ndim: u8, scope: Scope) -> Vec<f64>{
        vec![coordinates_to_gene(scope, 0.0); ndim.into()]
    }

    fn get_minimum(&self, ndim: u8) -> f64{
        0.0
    }
}

*/







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

    for _ in 0..100{
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
