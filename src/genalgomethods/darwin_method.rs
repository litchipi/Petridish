use rand::prelude::*;
use rand_distr::Normal;

use crate::utils::{MeanComputeVec, StddevComputeVec};
use crate::genalgo::*;
use crate::genalgomethods;

use serde::{Serialize, Deserialize};
use std::cmp;
use std::marker::PhantomData;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub (crate) struct DarwinMethodConfiguration{
    percent_elite: f64,
    variation_elite_pct: f64,
    gene_reroll_proba: f64,
    optimization_ratio_epoch_shift: u32
}

pub (crate) fn darwin_default_config() -> DarwinMethodConfiguration{
    DarwinMethodConfiguration {
        percent_elite: 0.01,
        variation_elite_pct: 0.25,
        gene_reroll_proba: 0.5,
        optimization_ratio_epoch_shift: 3
    }
}

pub (crate) fn new_darwin_method_settings() -> DarwinMethodSettings{
    DarwinMethodSettings { nb_elites: 0, nb_cells: 0}
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

#[derive(Copy, Clone)]
pub (crate) struct DarwinMethodSettings{
    nb_elites: u32,
    nb_cells: u32,
}

impl DarwinMethodSettings{
    fn from(&mut self, cfg: &GenalgoConfiguration, set: &GenalgoSettings){
        let method_cfg = match cfg.genalgo_method_config {
            genalgomethods::GenalgoMethodsConfigurations::DarwinConfig(c) => c,
        };
        self.nb_elites = ((set.nb_cells as f64) * method_cfg.percent_elite) as u32;
        self.nb_cells = set.nb_cells;
    }
}

pub (crate) struct DarwinMethod<T: Cell>{
    epoch_last_new_best: u32,
    best_cell: CellData,
    config: DarwinMethodConfiguration,
    settings: DarwinMethodSettings,
    _phantom: PhantomData<T>,
}

impl<T: Cell> genalgomethods::GenalgoMethod<T> for DarwinMethod<T>{
    fn new() -> Self where Self : Sized{
        DarwinMethod {
            best_cell: CellData { genome: Genome::new(), score: 0.0 },
            epoch_last_new_best : 0,
            config: darwin_default_config(),
            settings: new_darwin_method_settings(),
            _phantom: PhantomData,
        }
    }

    fn reset(&mut self){
        self.best_cell = CellData { genome: Genome::new(), score: 0.0 };
        self.epoch_last_new_best = 0;
    }

    fn load_config(&mut self, cfg: &GenalgoConfiguration, set: &GenalgoSettings){
        self.config = match cfg.genalgo_method_config {
            genalgomethods::GenalgoMethodsConfigurations::DarwinConfig(c) => c,
        }.clone();

        self.settings = new_darwin_method_settings();
        self.settings.from(cfg, set);
    }

    fn init_method(&mut self, bestcell: &CellData, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>{
        let bestgen = bestcell.genome.clone();
        if bestgen.len() == 0 {
            self.__init_generate_random_population(algo)
        } else if bestgen.len() < algo.get_genome_length() {
            //trace!("Best genome length < expected genome length, skipping");
            self.__init_generate_random_population(algo)
        } else {
            self.__init_generate_population_from_bestgen(bestgen.clone(), algo)
        }
    }

    //TODO IMPORTANT Propagate cells last results through generations (test with xinsheyang1 function)
    fn process_results(&mut self, maximize: bool, cells: Vec<&CellData>, var: &GenalgoVardata, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>{
        let mut rng = rand::thread_rng();

        if cells[0].score != self.best_cell.score{
            self.best_cell = cells[0].clone();
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
        let mut genomes: Vec<Genome> = vec![];
        let mut mean_elite = MeanComputeVec::new(cells[0].genome.len());


        genomes.push(cells[0].genome.clone());
        for i in 0..self.settings.nb_elites{
            mean_elite.add_el(
                &cells[i as usize].genome,
                if maximize { cells[i as usize].score } else { cells.last().unwrap().score - cells[i as usize].score }
                );
        }

        let mut std_elite  = StddevComputeVec::new(mean_elite.result.clone());
        for i in 0..self.settings.nb_elites{
            std_elite.add_el(&cells[i as usize].genome);
        }

        let parts_size = self.__compute_population_parts_sizes(optimization_ratio);
        assert_eq!(parts_size.iter().sum::<u32>(), self.settings.nb_cells-1);
        //println!("{:?} {}", parts_size, self.settings.nb_cells);
        self.__generate_elite_childs(&cells, parts_size[0], &mut genomes, optimization_ratio, &mut rng);
        self.__generate_elite_mutations(&cells, parts_size[1], &mut genomes, optimization_ratio, &mut rng);
        self.__generate_random_elite_childs(&cells, parts_size[2], &mut genomes, optimization_ratio, &mut rng);
        self.__generate_random_pop_childs(&cells, parts_size[3], &mut genomes, optimization_ratio, &mut rng);
        self.__generate_norm_random_cells(parts_size[4], &mean_elite.result, &std_elite.result, &mut genomes, &mut rng);
        self.__generate_random_cells(algo, parts_size[5], &mut genomes, &mut rng);

        genomes
    }
}

impl<T: Cell> DarwinMethod<T>{
    fn __compute_population_parts_sizes(&self, opt_ratio: f64) -> Vec<u32>{
        let pop = self.settings.nb_cells - 1;
        
        let opti_part = ((pop as f64)*(0.25 + (opt_ratio*0.5))) as u32;

        // OPTIMISATION PURPOSE
        let elite_childs = self.settings.nb_elites-1;
        let elite_mutated = (f64::from(opti_part-elite_childs)*0.5) as u32;
        let random_elite_child = opti_part-elite_childs-elite_mutated;

        // SWEEPING PURPOSE
        let sweep_part = pop-opti_part;
        let random_childs = ((sweep_part as f64)*0.25) as u32;
        let random_cells_norm = (((sweep_part-random_childs) as f64)*0.7) as u32;
        let random_cells = pop - opti_part - random_childs - random_cells_norm;
        vec![elite_childs, elite_mutated, random_elite_child, random_childs, random_cells_norm, random_cells]

    }

    fn __init_generate_population_from_bestgen(&mut self, bestgen: Genome, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>{
        let mut genomes: Vec<Genome> = vec![];

        genomes.push(bestgen.clone());
        
        let mut rng = rand::thread_rng();
        for i in 1..self.settings.nb_elites {
            let mut gen = bestgen.clone();
            self.mutate_genome(&mut gen, 0.75, &mut rng);
            genomes.push(gen);
        }

        for i in 0..(self.settings.nb_cells - (genomes.len() as u32)){
            genomes.push(self.random_genome(algo));
        }
        genomes
    }

    fn __init_generate_random_population(&mut self, algo: &Box<dyn Algo<CellType = T>>) -> Vec<Genome>{
        let mut genomes: Vec<Genome> = vec![];
        for i in 0..self.settings.nb_cells{
            genomes.push(self.random_genome(algo));
        }
        genomes
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

    fn __generate_elite_childs(&self, cells: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        for i in 0..((self.settings.nb_elites as usize) -1){
            let mut child = self.__give_birth(cells[i], cells[i+1], &BreedingMethod::ScoreBasedChoose, rng);
            self.mutate_genome(&mut child, (1.0-opt_ratio).powf(2.0), rng);
        }
    }

    fn __generate_elite_mutations(&self, cells: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        for i in 0..size{
            let random_cell_nb = rng.gen_range(0..self.settings.nb_elites);
            let mut genome = cells[random_cell_nb as usize].genome.clone();
            self.mutate_genome_direct(&mut genome, 1.0-opt_ratio, rng);
            genvec.push(genome)
        }
    }

    fn __generate_random_elite_childs(&self, cells: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        self.__generate_childs(&cells,
            (0, self.settings.nb_elites),
            (self.settings.nb_elites, cells.len() as u32),
            size, genvec, (1.0-opt_ratio).powf(2.0), rng, BreedingMethod::ScoreBasedAverage);
    }

    fn __generate_random_pop_childs(&self, cells: &Vec<&CellData>, size: u32, genvec: &mut Vec<Genome>, opt_ratio : f64, rng: &mut ThreadRng){
        self.__generate_childs(&cells,
            (self.settings.nb_elites, cells.len() as u32),
            (self.settings.nb_elites, cells.len() as u32),
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

    fn __generate_childs(&self, cells: &Vec<&CellData>, p1scope: (u32, u32), p2scope: (u32, u32), nb: u32, genvec: &mut Vec<Genome>, mutrat: f64, rng: &mut ThreadRng, method: BreedingMethod){
        for i in 0..nb{
            let (nb1, nb2) = self.__choose_parents(p1scope, p2scope, rng);
            let mut child = self.__give_birth(cells[nb1], cells[nb2], &method, rng);
            self.mutate_genome(&mut child, mutrat, rng);
            genvec.push(child);
        }
    }
}

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
