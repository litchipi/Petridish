use rand::prelude::*;
use rand_distr::Normal;

use crate::errors::Errcode;
use crate::utils::{MeanComputeVec, StddevComputeVec};
use crate::lab;
use crate::genalgomethods::*;
use crate::cell::{Cell, Genome, CellData};
use crate::algo::Algo;


use serde::{Serialize, Deserialize};
use std::cmp;
use std::marker::PhantomData;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct DarwinMethodConfiguration{
    gene_reroll_proba: f64,
    optimization_ratio_epoch_shift: u32
}

pub fn darwin_default_config() -> DarwinMethodConfiguration{
    DarwinMethodConfiguration {
        gene_reroll_proba: 0.5,
        optimization_ratio_epoch_shift: 3
    }
}

fn normal_random_vec(moy_vec: &Genome, stdev: &Genome, rng: &mut ThreadRng) -> Genome{
    let mut res = Genome::new();
    assert_eq!(moy_vec.len(), stdev.len());
    for n in 0..moy_vec.len(){
        res.push({
            assert_ne!(stdev[n], f64::NAN);
            let mut nb = Normal::new(moy_vec[n], stdev[n]).unwrap().sample(rng);
            while nb >= 1.0{
                nb -= stdev[n];
            }
            while nb <= 0.0{
                nb += stdev[n];
            }
            nb
        });
        assert!(res[n] >= 0.0);
        assert!(res[n] <= 1.0);
    }
    res
}

enum BreedingMethod{
    ScoreBasedAverage,
    ScoreBasedChoose
}

pub struct DarwinMethod<T: Cell>{
    epoch_last_new_best: u32,
    config: DarwinMethodConfiguration,
    _phantom: PhantomData<T>,
}

impl<T: Cell> GenalgoMethod<T> for DarwinMethod<T>{
    fn new() -> Self where Self : Sized{
        DarwinMethod {
            epoch_last_new_best : 0,
            config: darwin_default_config(),
            _phantom: PhantomData,
        }
    }

    fn reset(&mut self){
        self.epoch_last_new_best = 0;
    }

    fn load_config(&mut self, cfg: GenalgoMethodsConfigurations){
        self.config = match cfg {
            GenalgoMethodsConfigurations::DarwinConfig(c) => c,
        }.clone();
    }

    fn init_method(&mut self, bestgen: &Genome, nb_cells: u32, nb_elites: u32, algo: &Box<dyn Algo<CellType = T>>, res: &mut Vec<Genome>) -> Result<(), Errcode>{
        if bestgen.len() == 0 {
            return Ok(self.__init_generate_random_population(algo, nb_cells, res))
        } else if bestgen.len() < algo.get_genome_length() {
            return Err( Errcode::CodeError("best genome length < expected algo length"));
        } else {
            return Ok(self.__init_generate_population_from_bestgen(bestgen.clone(), nb_cells, nb_elites, algo, res))
        }
    }

    fn process_results(&mut self, elites: &Vec<&CellData>, cells: &Vec<CellData>, algo: &Box<dyn Algo<CellType = T>>, genomes: &mut Vec<Genome>) -> Result<(), Errcode>{
        let mut rng = rand::thread_rng();

        if cells[0].score != elites.get(0).unwrap().score{
            self.epoch_last_new_best = 0;
        }else{
            self.epoch_last_new_best += 1;
        }

        let optimization_ratio: f64 = {
            let rgen : f64 = rng.gen();
            let res = (1.0 + self.epoch_last_new_best as f64) / ((1 + self.config.optimization_ratio_epoch_shift + self.epoch_last_new_best) as f64);
            assert!(res.is_finite());
            rgen*res
        };
        
        let mut mean_elite = MeanComputeVec::new(elites[0].genome.len());
        for elite in elites.iter(){
            assert!(elite.score < 1.0);
            assert!(elite.score > 0.0);
            mean_elite.add_el(&elite.genome, elite.score)
        }

        let mut std_elite  = StddevComputeVec::new(mean_elite.result.clone());
        for elite in elites.iter(){
            std_elite.add_el(&elite.genome);
        }

        let parts_size = self.__compute_population_parts_sizes(elites.len(), optimization_ratio, (cells.len() -1) as u32);
        assert_eq!(parts_size.iter().sum::<u32>(), (cells.len()-1) as u32);
        self.__generate_elite_childs(elites, parts_size[0], genomes, optimization_ratio, &mut rng);
        self.__generate_elite_mutations(&elites, parts_size[1], genomes, optimization_ratio, &mut rng);
        self.__generate_random_elite_childs(cells, elites.len() as u32, parts_size[2], genomes, optimization_ratio, &mut rng);
        self.__generate_random_pop_childs(cells, elites.len() as u32, parts_size[3], genomes, optimization_ratio, &mut rng);
        self.__generate_norm_random_cells(parts_size[4], &mean_elite.result, &std_elite.result, genomes, &mut rng);
        self.__generate_random_cells(algo, parts_size[5], genomes, &mut rng);
        Ok(())
    }

    fn validate_config(&self) -> Result<(), Errcode>{
        println!("Validation of Darwin config");
        Err(Errcode::NotImplemented("Darwin validate config"))
    }
}

impl<T: Cell> DarwinMethod<T>{
    fn __compute_population_parts_sizes(&self, nb_elites: usize, opt_ratio: f64, pop: u32) -> Vec<u32>{
        let opti_part = ((pop as f64)*(0.25 + (opt_ratio*0.5))) as u32;

        // OPTIMISATION PURPOSE
        let elite_childs = (nb_elites-1) as u32;
        let elite_mutated = (f64::from(opti_part-elite_childs)*0.5) as u32;
        let random_elite_child = opti_part-elite_childs-elite_mutated;

        // SWEEPING PURPOSE
        let sweep_part = pop-opti_part;
        let random_childs = ((sweep_part as f64)*0.25) as u32;
        let random_cells_norm = (((sweep_part-random_childs) as f64)*0.7) as u32;
        let random_cells = pop - opti_part - random_childs - random_cells_norm;
        vec![elite_childs, elite_mutated, random_elite_child, random_childs, random_cells_norm, random_cells]

    }

    fn __init_generate_population_from_bestgen(&mut self, bestgen: Genome, nb_cells: u32, nb_elites: u32, algo: &Box<dyn Algo<CellType = T>>, genomes: &mut Vec<Genome>){
        genomes.push(bestgen.clone());
        
        let mut rng = rand::thread_rng();
        for i in 1..nb_elites {
            let mut gen = bestgen.clone();
            self.mutate_genome(&mut gen, 0.75, &mut rng);
            genomes.push(gen);
        }

        for i in 0..(nb_cells - (genomes.len() as u32)){
            genomes.push(self.random_genome(algo));
        }
    }

    fn __init_generate_random_population(&mut self, algo: &Box<dyn Algo<CellType = T>>, nb_cells: u32, genomes: &mut Vec<Genome>){
        for i in 0..nb_cells{
            genomes.push(self.random_genome(algo));
        }
    }




    /*          GENOME MANIPULATION         */

    fn mutate_genome(&self, genome: &mut Genome, rate: f64, rng: &mut ThreadRng) {
        for g in genome.iter_mut() {
            let nb : f64 = rng.gen();
            if nb < rate*self.config.gene_reroll_proba {
                *g = rng.gen();
            } else if nb < rate*(1.0-self.config.gene_reroll_proba) {
                *g = (*g*(1.0+((nb-0.5)*(1.0+rate))))%1.0;
            }
        }
    }

    fn mutate_genome_direct(&self, genome: &mut Genome, rate: f64, rng: &mut ThreadRng) {
        for g in genome.iter_mut() {
            let nb : f64 = rng.gen();
            if nb < self.config.gene_reroll_proba {
                *g = rng.gen();
            } else {
                *g = (*g*(1.0+((nb-0.5)*(1.0+rate))))%1.0;
            }
        }
    }

    fn random_genome(&self, algo: &Box<dyn Algo<CellType = T>>) -> Genome {
        let mut rng = rand::thread_rng();
        self.__random_genome(&mut rng, algo.get_genome_length())
    }

    fn __random_genome(&self, rng: &mut ThreadRng, len: usize) -> Genome {
        let mut res : Genome = vec![];
        for i in 0..len {
            res.push(rng.gen());
        }
        res
    }

    fn __choose_parents(&self, p1scope: (u32, u32), p2scope: (u32, u32), rng: &mut ThreadRng) -> (usize, usize){
        (rng.gen_range(p1scope.0..p1scope.1) as usize, rng.gen_range(p2scope.0..p2scope.1) as usize)
    }

    fn __give_birth(&self, p1: &CellData, p2: &CellData, method: &BreedingMethod, rng: &mut ThreadRng) -> Genome{
        assert_eq!(p1.genome.len(), p2.genome.len());
        let mut genome = Genome::new();
        let sumscores : f64 = p1.score + p2.score;
        let part = (p1.score / sumscores, p2.score / sumscores);
        for g in 0..p1.genome.len(){
            let num : f64 = rng.gen();
            if num < part.0 {
                genome.push(p1.genome[g]);
            }else{
                genome.push(p2.genome[g]);
            }
        }
        genome
    }

    /*          CELLS MANIPULATION          */

    fn __generate_elite_childs(&self, elites: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        for i in 0..(elites.len() - 1){
            let mut child = self.__give_birth(elites.get(i).unwrap(), elites.get(i+1).unwrap(), &BreedingMethod::ScoreBasedChoose, rng);
            self.mutate_genome(&mut child, (1.0-opt_ratio).powf(2.0), rng);
            genvec.push(child);
        }
    }

    fn __generate_elite_mutations(&self, elites: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        for i in 0..size{
            let random_cell_nb = rng.gen_range(0..elites.len());
            let mut genome = elites[random_cell_nb as usize].genome.clone();
            self.mutate_genome_direct(&mut genome, 1.0-opt_ratio, rng);
            genvec.push(genome)
        }
    }

    fn __generate_random_elite_childs(&self, cells: &Vec<CellData>, nb_elites: u32, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        self.__generate_childs(cells,
            (0, nb_elites),
            (nb_elites, cells.len() as u32),
            size, genvec, (1.0-opt_ratio).powf(2.0), rng, BreedingMethod::ScoreBasedAverage);
    }

    fn __generate_random_pop_childs(&self, cells: &Vec<CellData>, nb_elites: u32, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        self.__generate_childs(cells,
            (nb_elites, cells.len() as u32),
            (nb_elites, cells.len() as u32),
            size, genvec, 1.0-opt_ratio, rng, BreedingMethod::ScoreBasedChoose);
    }

    fn __generate_norm_random_cells(&self, size: u32, mean_elites: &Genome, stddev_elites: &Genome, genvec: &mut Vec<Genome>, rng: &mut ThreadRng){
        for i in 0..size{
            genvec.push(normal_random_vec(mean_elites, &stddev_elites, rng));
        }
    }

    fn __generate_random_cells(&self, algo: &Box<dyn Algo<CellType = T>>, size: u32, genvec: &mut Vec<Genome>, rng: &mut ThreadRng){
        let genomelen = algo.get_genome_length();
        for i in 0..size{
            genvec.push(self.__random_genome(rng, genomelen));
        }
    }

    fn __generate_childs(&self, cells: &Vec<CellData>, p1scope: (u32, u32), p2scope: (u32, u32), nb: u32, genvec: &mut Vec<Genome>, mutrat: f64, rng: &mut ThreadRng, method: BreedingMethod){
        for i in 0..nb{
            let (nb1, nb2) = self.__choose_parents(p1scope, p2scope, rng);
            let mut child = self.__give_birth(cells.get(nb1).unwrap(), cells.get(nb2).unwrap(), &method, rng);
            self.mutate_genome(&mut child, mutrat, rng);
            genvec.push(child);
        }
    }
}
/*
#[test]
fn test_random_genome_generation(){
    use crate::algorithms;
    let a = DarwinMethod::new();
    let algo = algorithms::get_algo("algo_test");
    let genome_a = a.random_genome(&algo);
    let genome_b = a.random_genome(&algo);
    assert_eq!(genome_a.len(), algo.get_genome_length());
    assert_eq!(genome_b.len(), genome_a.len());
    println!("{:?}", genome_a);
    println!("{:?}", genome_b);
    for i in 0..genome_a.len(){
        assert_ne!(genome_a[i], genome_b[i]);
    }
}

#[test]
fn test_mutation_genome(){
    use crate::algorithms;
    let a = DarwinMethod::new();
    let algo = algorithms::get_algo("algo_test");
    let genome_a = a.random_genome(&algo);

    let mut success = false;
    for ntry in 0..1000{
        let genome_b = {
            let mut tmp = genome_a.clone();
            let mut rng = rand::thread_rng();
            a.mutate_genome(&mut tmp, 0.10, &mut rng);
            tmp
        };

        assert_eq!(genome_a.len(), algo.get_genome_length());
        println!("{:?}", genome_a);
        println!("{:?}", genome_b);
        for i in 0..genome_a.len(){
            success |= genome_a[i] != genome_b[i];
        }
        if success{
            break;
        }
    }
    assert!(success);
}

#[test]
fn test_mutation_genome_direct(){
    use crate::algorithms;
    let a = DarwinMethod::new();
    let algo = algorithms::get_algo("algo_test");
    let genome_a = a.random_genome(&algo);
    
    println!("{:?}", genome_a);
    for ntry in 0..10{
        let genome_b = {
            let mut tmp = genome_a.clone();
            let mut rng = rand::thread_rng();
            a.mutate_genome_direct(&mut tmp, 0.0002, &mut rng);
            tmp
        };

        assert_eq!(genome_a.len(), algo.get_genome_length());

        println!("{:?}", genome_b);
        for i in 0..genome_a.len(){
            assert_ne!(genome_a[i], genome_b[i]);
        }
    }
}

#[test]
fn test_normal_random_distrib(){
    for i in 0..1000{
        let res = normal_random_vec(&vec![0.5, 0.5], &vec![0.5, 0.5], &mut rand::thread_rng());
        println!("{:?}", res);
    }
    assert!(true);
}
*/
