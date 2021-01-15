use crate::genalgo_lab::genalgo::{Algo, Cell};
pub mod algo_test;
pub mod benchmark_fcts;


pub enum AlgoAvailable{
    None,
    TestAlgo(algo_test::TestAlgo),
    BenchmarkAlgo(benchmark_fcts::BenchmarkAlgo),
}

pub enum AllCellsTypes{
    None,
    TestAlgoCell(algo_test::TestCell),
    BenchmarkAlgoCell(benchmark_fcts::BenchmarkCell),
}





impl AllCellsTypes{
    pub fn unwrap(&self) -> Box<&dyn Cell>{
        match self {
            AllCellsTypes::TestAlgoCell(c) => Box::new(c),
            AllCellsTypes::BenchmarkAlgoCell(b) => Box::new(b),
            AllCellsTypes::None => panic!("Attempt to unwrap None cell")
        }
    }

    pub fn unwrap_mut(&mut self) -> Box<&mut dyn Cell>{
        match self{
            AllCellsTypes::TestAlgoCell(c) => Box::new(c),
            AllCellsTypes::BenchmarkAlgoCell(b) => Box::new(b),
            AllCellsTypes::None => panic!("Attempt to unwrap None cell")
        }
    }
}

impl AlgoAvailable{
    pub fn unwrap(&self) -> Box<&dyn Algo> {
        match self {
            AlgoAvailable::TestAlgo(t) => Box::new(t),
            AlgoAvailable::BenchmarkAlgo(b) => Box::new(b),
            AlgoAvailable::None => panic!("Attempt to unwrap None algo")
        }
    }

    pub fn unwrap_mut(&mut self) -> Box<&mut dyn Algo>{
        match self{
            AlgoAvailable::TestAlgo(t) => Box::new(t),
            AlgoAvailable::BenchmarkAlgo(b) => Box::new(b),
            AlgoAvailable::None => panic!("Attempt to unwrap None algo")
        }
    }
}





pub fn get_algo(name: &str) -> AlgoAvailable {
    match name {
        "algo_test" => AlgoAvailable::TestAlgo(algo_test::TestAlgo {}),
        "benchmark" => AlgoAvailable::BenchmarkAlgo(benchmark_fcts::BenchmarkAlgo::new(benchmark_fcts::get_fct_by_name("spherical"), 8)),
        _ => panic!("No such algo name implemented")
    }
}
