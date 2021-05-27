//TODO  IMPORTANT Find a way to import macros from dependencies of generate_py_ifaces in the scope
//TODO  IMPORTANT Import missing dependencies for generate_py_ifaces

#[macro_use]
use petridish::py_iface::*;
use petridish::generate_py_ifaces;
use petridish::pyo3;

struct TestCell;
struct TestAlgoA;
struct TestAlgoB;

generate_py_ifaces!(
    [test] TestCell => (A => TestAlgoA, B => TestAlgoB),
);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
