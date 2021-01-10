use crate::genalgo_lab::genalgo::{Algo, Cell};
pub mod algo_test;


pub enum AlgoAvailable{
    None,
    TestAlgo(algo_test::TestAlgo),
}

pub enum AllCellsTypes{
    None,
    TestAlgoCell(algo_test::TestCell),
}





impl AllCellsTypes{
    pub fn unwrap(&self) -> &impl Cell{
        match self{
            AllCellsTypes::TestAlgoCell(c) => c,
            AllCellsTypes::None => panic!("Attempt to unwrap None cell")
        }
    }
}

impl AlgoAvailable{
    pub fn unwrap(&self) -> &impl Algo{
        match self{
            AlgoAvailable::TestAlgo(t) => t,
            AlgoAvailable::None => panic!("Attempt to unwrap None algo")
        }
    }
}





pub fn get_algo(name: &str) -> AlgoAvailable {
    match name {
        "algo_test" => AlgoAvailable::TestAlgo(algo_test::TestAlgo {}),
        _ => panic!("No such algo name implemented")
    }
}
