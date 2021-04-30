# Petridish
Genetic Algorithm library, written in Rust.
Project made to learn Rust langage in depth and have fun, didn't expect it to work.
Following objectives are to implement Black Box Optimisation Benchmark (BbOB) functions to test each optimisation method and improve them.

Intended to be used as a Rust library with limited API on Python as the pyo3 library restrict the capabilities.

### Implemented Optimisation methods
- **Darwin** (Mix of technics, including CMA on elites and natural selection. Name chosen by myself)


### Implemented BboB functions
- [Spheric function](http://benchmarkfcns.xyz/benchmarkfcns/spherefcn.html)
- [Xin-She Yang 1 function](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn1fcn.html)
**Note:** *As this function gets a part of randomness, I cheated and made a mean of 2 values at calculation time to allow the optimisation to converge faster*
- [Xin-She Yang 2 function](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn2fcn.html)

### Implemented Algorithms
- AlgoTest (Dummy)
- Benchmark (BBOB functions)
