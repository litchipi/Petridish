use petridish::cell::Genome;
use rand::prelude::*;

type FctScope = (i64, i64);

trait MathFct{
    fn calc(&self, inputs: &Genome) -> f64;
    fn set_scope(&mut self, scope: FctScope);
}


#[derive(Copy, Clone)]
pub enum BenchmarkFct{
    Nofct,
    Spherical(SphericalFct),
    XinSheYang1(XinSheYang1Fct),
    XinSheYang2(XinSheYang2Fct),
}

impl BenchmarkFct{
    pub fn calc(&mut self, data: &Genome) -> f64 {
        match self {
            BenchmarkFct::Spherical(f) => f.calc(data),
            BenchmarkFct::XinSheYang1(f) => f.calc(data),
            BenchmarkFct::XinSheYang2(f) => f.calc(data),
            _ => panic!("Function not set or recognized"),
        }
    }

    pub fn get_expected_optimum(&self, ndim: u8, scope: FctScope) -> Vec<f64>{
        match self {
            BenchmarkFct::Spherical(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            BenchmarkFct::XinSheYang1(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            BenchmarkFct::XinSheYang2(f) => vec![coordinates_to_gene(scope, 0.0); ndim.into()],
            _ => panic!("Function not set or recognized"),
        }
    }

    pub fn get_minimum(&self) -> f64{
        match self {
            BenchmarkFct::Spherical(f) => 0.0,
            BenchmarkFct::XinSheYang1(f) => 0.0,
            BenchmarkFct::XinSheYang2(f) => 0.0,
            _ => panic!("Function not set or recognized"),
        }
    }

    pub fn set_scope(&mut self, scope: FctScope){
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
