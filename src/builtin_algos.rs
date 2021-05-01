use crate::genalgo::*;
use crate::lab::Algo;

pub mod algo_test;
pub mod benchmark_fcts;

pub enum BuiltinAlgo{
    TestAlgo(Genalgo<algo_test::TestCell>),
    BenchmarkAlgo(Genalgo<benchmark_fcts::BenchmarkCell>),
}

impl BuiltinAlgo{
    pub fn create(name: &str) -> Option<BuiltinAlgo>{
        match name {
//            "algo_test" => Option::Some(BuiltinAlgo::TestAlgo(Genalgo::create_algo(Box::new(algo_test::TestAlgo {})))),
//            "benchmark" => Option::Some(BuiltinAlgo::BenchmarkAlgo(Genalgo::new (Box::new(benchmark_fcts::BenchmarkAlgo::new())))),
            _ => Option::None
        }
    }
}

/*  TODO        Benchmark algorithms and lab
 *      Benchmarking functions with and without the split of dimensions
 *          Test with high dimensions to check if dimensions split works
 *          Output tests results to csv file
 *          Open file with Python and display matplotlib
 */
