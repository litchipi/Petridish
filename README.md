# RustPetridish
Genetic Algorithm library meant to be used inside a Python code, written in Rust.
Project made to learn Rust langage in depth and have fun, didn't expect it to work.


Following objectives are to implement Black Box Optimisation Benchmark (BbOB) functions to test each optimisation method and improve them, also to provide API to be able to tweak the algorithm behaviours from Python easily.


If you want to play with it, build it using cargo, copy file generated at target/\<release or debug\>/libgenalgo.so to where the test script is, and import it with Python as you would import any Python file.


Made to have fun, experiment and learn ! :-)


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
