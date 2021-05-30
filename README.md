# Petridish
A library to create a Genetic Algorithm library, written in Rust.
Examples are located in the `example` directory.
Meant to be used inside a Rust project after defining own structs implementing mandatory traits, has some 
bindings to Python using pyo3 (see examples & experiment with them)

### Principle
A Genalgo struct gathers datasets and feed them into a Lab.

A Lab is a set of Algo, which manages and feed data to Cells.

After all data of all datasets fed the cells, the cells are passed into a GenalgoMethod to
prepare the next generation, and get the best cells.

The GenalgoMethod uses MutationProcess and BreedingMethod to make operations on Cell's genomes

Each algo's best cell is injected into the population of another algo, based on a LabMap.

The algo defined as "output algo" is the one used to get the final optimised genome.

### Philosophy
Everything must be moddable using traits.
Are traits:
- Cell
- Algo
- GenalgoMethod
- MutationProcess (Work in progress)
- BreedingMethod (Work in progress)
- LabMap (Work in progress)

### Implemented Optimisation methods
- **Darwin** (Mix of technics, including CMA on elites and natural selection. Custom made for experiments)


### Implemented BboB functions
On the code of `example/benchmarking/`, you can test the optimisation efficiency using Black-Box Optimisation Benchmark functions (BBOB).
The following functions are implemented: 
- [Spheric function](http://benchmarkfcns.xyz/benchmarkfcns/spherefcn.html)
- [Xin-She Yang 1 function](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn1fcn.html)
- [Xin-She Yang 2 function](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn2fcn.html)
- [Schwefel 2.20 function](http://benchmarkfcns.xyz/benchmarkfcns/schwefel220fcn.html)
- [Styblinski Tank function](http://benchmarkfcns.xyz/benchmarkfcns/styblinskitankfcn.html)
- [Quartic function](http://benchmarkfcns.xyz/benchmarkfcns/quarticfcn.html)
*A special thanks to [mazhar-ansari-ardeh](https://github.com/mazhar-ansari-ardeh) for his very high quality website about these functions*
