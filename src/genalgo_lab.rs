pub mod genalgo;
extern crate serde;
extern crate pyo3;

use serde::{Serialize, Deserialize};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::collections::HashMap;

type AlgoID = u8;
pub type JsonData = String;
pub type ThreadID = u64;

#[pyclass]
pub struct GenalgoEngine {
    #[pyo3(get)]
    nb_algos: i32,
    algos: HashMap<AlgoID, genalgo::Genalgo>,
    algoid_counter: AlgoID
}

impl GenalgoEngine {
    pub fn new() -> GenalgoEngine {
        GenalgoEngine {nb_algos: 0, algos: HashMap::new(), algoid_counter: 0}
    }

    fn get_algo(&self, id: AlgoID) -> Option<&genalgo::Genalgo> {
        self.algos.get(&id)
    }

    fn generate_new_id(&mut self) -> AlgoID {
        if self.algoid_counter >= 255{
            panic!("Algorithms number reached maximum");
        }
        self.algoid_counter += 1;
        self.algoid_counter
    }

    fn create_algo_thread(&mut self, id: AlgoID){
        self.algos.get_mut(&id).unwrap().thread_id = 1;     //TODO Create a thread of each algo registered
    }
}

#[pymethods]
impl GenalgoEngine {
    fn register_algo(&mut self, name: &str) -> AlgoID {
        let id = self.generate_new_id();
        self.algos.insert(id, genalgo::Genalgo::create_algo(name));
        self.create_algo_thread(id);
        id
    }


    /*          COMMANDS            */
    fn start_algo(&mut self, id: AlgoID){
        // TODO Start algo thread
    }

    fn stop_algo(&mut self, id: AlgoID){
        // TODO Stop algo execution (pause thread)
    }

    fn get_algo_status(&mut self, id:AlgoID) -> JsonData{
        serde_json::to_string(&(self.get_algo(id).expect("Cannot find algo").status)).expect("Cannot deserialize Algo Status into JSON")
    }

    fn algo_bestgen(&mut self, id:AlgoID) -> genalgo::Genome {
        self.get_algo(id).expect("Cannot find algo").bestgen.clone()
    }
}





















/*              TESTS                   */

#[test]
#[should_panic]
fn test_genalgo_limits(){
    let mut engine = GenalgoEngine::new();

    for i in 0..256 {
        engine.register_algo("algo_test");
    }
}

#[test]
fn test_genalgo_add() {
    let mut engine = GenalgoEngine::new();

    let id = engine.register_algo("algo_test");
    assert_eq!(id, 1);
    assert_eq!(engine.algoid_counter, 1);

    let id = engine.register_algo("algo_test");
    assert_eq!(id, 2);
    assert_eq!(engine.algoid_counter, 2);

    for i in 0..253{
        let id = engine.register_algo("algo_test");
        assert_eq!(id, 3+i);
        assert_eq!(engine.algoid_counter, 3+i);
    }
}

#[test]
fn test_genalgo_status_json_serialize(){
    let mut engine = GenalgoEngine::new();

    let id = engine.register_algo("algo_test");
    let jsonstatus = engine.get_algo_status(id);
    println!("{}", jsonstatus);
    assert_eq!(jsonstatus, "{\"started\":false,\"epoch\":0}");
}

#[test]
fn test_genalgo_control(){

}


#[test]
fn pyclass_manipulation_basic() {
    /*
    use pyo3::prelude::*;
    use pyo3::types::PyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();
    let obj = PyCell::new(py, GenalgoEngine{ num: 3, debug: true }).unwrap();
    {
        let obj_ref = obj.borrow(); // Get PyRef
        assert_eq!(obj_ref.num, 3);
        // You cannot get PyRefMut unless all PyRefs are dropped
        assert!(obj.try_borrow_mut().is_err());
    }
    {
        let mut obj_mut = obj.borrow_mut(); // Get PyRefMut
        obj_mut.num = 5;
        // You cannot get any other refs until the PyRefMut is dropped
        assert!(obj.try_borrow().is_err());
        assert!(obj.try_borrow_mut().is_err());
    }

    // You can convert `&PyCell` to a Python object
    pyo3::py_run!(py, obj, "assert obj.num == 5");
    */
}
