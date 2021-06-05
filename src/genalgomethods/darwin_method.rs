use rand::prelude::*;
use rand_distr::Normal;

use crate::cell::{Cell, CellData, Genome};
use crate::errors::Errcode;
use crate::genalgomethods::*;
use crate::utils::{MeanComputeVec, StddevComputeVec};

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct DarwinMethodConfiguration {
    gene_reroll_proba: f64,
    exploration_scope_epoch_max: u32,
}

impl DarwinMethodConfiguration {
    pub fn default() -> DarwinMethodConfiguration {
        DarwinMethodConfiguration {
            gene_reroll_proba: 0.5,
            exploration_scope_epoch_max: 3,
        }
    }
}

fn normal_random_vec(moy_vec: &Genome, stdev: &Genome, rng: &mut ThreadRng) -> Genome {
    assert_eq!(moy_vec.len(), stdev.len());
    let mut res = Genome::new();

    for n in 0..moy_vec.len() {
        res.push({
            assert!(!stdev[n].is_nan());
            let mut nb = 0.0;
            while (nb >= 1.0) || (nb <= 0.0) {
                nb = Normal::new(moy_vec[n], stdev[n]).unwrap().sample(rng);
            }
            nb
        });
    }

    res
}

fn __transform_score(bestscore: f64, cellscore: f64) -> f64 {
    cellscore.log(bestscore)
}

enum BreedingMethod {
    ScoreBasedAverage,
    ScoreBasedChoose,
}

//TODO  Rewrite it with generic enums implementation of mutation and breeding
//TODO  Move out generic code into general functions in genalgomethods.rs
//TODO  Clean code from esoteric tries, comment,
//          improve general algorithm structure and lisibility

pub struct DarwinMethod<T: Cell> {
    epoch_last_new_best: u32,
    config: DarwinMethodConfiguration,
    bestcell_avg: MeanComputeVec,
    last_best_cell: Genome,
    _phantom: PhantomData<T>,
}

impl<T: Cell> GenalgoMethod<T> for DarwinMethod<T> {
    fn new() -> Self
    where
        Self: Sized,
    {
        DarwinMethod {
            last_best_cell: vec![],
            bestcell_avg: MeanComputeVec::new(T::get_genome_length()),
            epoch_last_new_best: 0,
            config: DarwinMethodConfiguration::default(),
            _phantom: PhantomData,
        }
    }

    fn json_import(&mut self, _jsdata: JsonData) -> Self
    where
        Self: Sized,
    {
        todo!();
    }

    fn reset(&mut self) {
        self.epoch_last_new_best = 0;
    }

    fn load_config(&mut self, cfg: &GenalgoMethodsConfigurations) {
        self.config = match cfg {
            GenalgoMethodsConfigurations::DarwinConfig(c) => c,
            _ => unreachable!(),
        }
        .clone();
    }

    fn init_population(
        &mut self,
        bestgen: &Genome,
        nb_cells: u32,
        nb_elites: u32,
        res: &mut Vec<Genome>,
    ) -> Result<(), Errcode> {
        if bestgen.len() == 0 {
            return Ok(self.__init_generate_random_population(nb_cells, res));
        } else if bestgen.len() < T::get_genome_length() {
            return Err(Errcode::CodeError(
                "best genome length < expected algo length",
            ));
        } else {
            return Ok(self.__init_generate_population_from_bestgen(
                bestgen.clone(),
                nb_cells,
                nb_elites,
                res,
            ));
        }
    }

    fn process_results(
        &mut self,
        elites: &Vec<&CellData>,
        cells: &Vec<CellData>,
        genomes: &mut Vec<Genome>,
    ) -> Result<(), Errcode> {
        let mut rng = rand::thread_rng();

        if (!self.last_best_cell.is_empty()) & (cells[0].genome != self.last_best_cell) {
            self.epoch_last_new_best = 0;
            self.bestcell_avg.add_el(&cells[0].genome, 1.0);
            self.last_best_cell = cells[0].genome.clone();
        } else {
            self.epoch_last_new_best += 1;
        }

        if self.last_best_cell.is_empty() {
            self.last_best_cell = cells[0].genome.clone();
        }

        //TODO  Add to DarwinMethodConfiguration
        let min_explo = 0.15;
        let max_explo = 0.85;

        let exploration_ratio: f64 = {
            let max_ratio = (self.config.exploration_scope_epoch_max as f64)
                / ((self.config.exploration_scope_epoch_max + self.epoch_last_new_best) as f64);
            min_explo + (max_ratio * rng.gen::<f64>() * (max_explo - min_explo))
        };

        let mut mean_elite = MeanComputeVec::new(elites[0].genome.len());
        for elite in elites.iter() {
            mean_elite.add_el(
                &elite.genome,
                __transform_score(cells[0].score, elite.score),
            )
        }

        let mut std_elite = StddevComputeVec::new(mean_elite.result.clone());
        for elite in elites.iter() {
            std_elite.add_el(&elite.genome);
        }

        let parts_size = self.__compute_population_parts_sizes(
            elites.len() as u32,
            exploration_ratio,
            (cells.len() - 2) as u32,
        );
        assert_eq!(parts_size.iter().sum::<u32>(), (cells.len() - 2) as u32);
        genomes.push(elites.get(0).unwrap().genome.clone());
        genomes.push(self.bestcell_avg.result.clone());
        self.__generate_elite_childs(elites, genomes, exploration_ratio, &mut rng);
        self.__generate_elite_mutations(
            &elites,
            parts_size[1],
            genomes,
            exploration_ratio,
            &mut rng,
        );
        self.__generate_random_elite_childs(
            cells,
            elites.len() as u32,
            parts_size[2],
            genomes,
            exploration_ratio,
            &mut rng,
        );
        self.__generate_random_pop_childs(
            cells,
            elites.len() as u32,
            parts_size[3],
            genomes,
            exploration_ratio,
            &mut rng,
        );
        self.__generate_norm_random_cells(
            parts_size[4],
            &mean_elite.result,
            &std_elite.result,
            genomes,
            &mut rng,
        );
        self.__generate_random_cells(parts_size[5], genomes, &mut rng);
        Ok(())
    }

    fn validate_config(&self) -> Result<(), Errcode> {
        if self.config.exploration_scope_epoch_max == 0 {
            return Err(Errcode::ValidationError(
                "Darwin method: exploration_scope_epoch_max == 0",
            ));
        }

        if (self.config.gene_reroll_proba > 1.0) || (self.config.gene_reroll_proba < 0.0) {
            return Err(Errcode::ValidationError(
                "Darwin method: gene_reroll_proba not in range (0, 1)",
            ));
        }

        Ok(())
    }
}

pub fn get_part_of_pop(pop: u32, part: f64) -> u32 {
    ((pop as f64) * part) as u32
}

impl<T: Cell> DarwinMethod<T> {
    fn __compute_population_parts_sizes(
        &self,
        nb_elites: u32,
        exploration_ratio: f64,
        pop: u32,
    ) -> Vec<u32> {
        let pop_rest = pop - (nb_elites - 1);

        // OPTIMISATION PURPOSE
        let opti_pop = get_part_of_pop(pop_rest, 1.0 - exploration_ratio);
        let elite_mutated = get_part_of_pop(opti_pop, 0.6);
        let random_elite_child = get_part_of_pop(opti_pop, 0.4);

        // EXPLORATION PURPOSE
        let explo_pop = pop_rest - opti_pop;
        let random_childs = get_part_of_pop(explo_pop, 0.4);
        let random_cells_norm = get_part_of_pop(explo_pop, 0.4);
        let random_cells =
            pop_rest - elite_mutated - random_elite_child - random_childs - random_cells_norm;

        assert_eq!(
            (nb_elites - 1)
                + elite_mutated
                + random_elite_child
                + random_childs
                + random_cells_norm
                + random_cells,
            pop
        );

        vec![
            (nb_elites - 1),
            elite_mutated,
            random_elite_child,
            random_childs,
            random_cells_norm,
            random_cells,
        ]
    }

    fn __init_generate_population_from_bestgen(
        &mut self,
        bestgen: Genome,
        nb_cells: u32,
        nb_elites: u32,
        genomes: &mut Vec<Genome>,
    ) {
        genomes.push(bestgen.clone());

        let mut rng = rand::thread_rng();
        for _ in 1..nb_elites {
            let mut gen = bestgen.clone();
            self.mutate_genome(&mut gen, 0.75, &mut rng);
            genomes.push(gen);
        }

        for _ in 0..(nb_cells - (genomes.len() as u32)) {
            genomes.push(self.random_genome());
        }
    }

    fn __init_generate_random_population(&mut self, nb_cells: u32, genomes: &mut Vec<Genome>) {
        for _ in 0..nb_cells {
            genomes.push(self.random_genome());
        }
    }

    /*          GENOME MANIPULATION         */

    fn mutate_genome(&self, genome: &mut Genome, rate: f64, rng: &mut ThreadRng) {
        for g in genome.iter_mut() {
            let nb: f64 = rng.gen();
            if nb < rate * self.config.gene_reroll_proba {
                *g = rng.gen();
            } else if nb < rate * (1.0 - self.config.gene_reroll_proba) {
                *g = (*g * (1.0 + ((nb - 0.5) * (1.0 + rate)))) % 1.0;
            }
        }
    }

    fn mutate_genome_direct(&self, genome: &mut Genome, rate: f64, rng: &mut ThreadRng) {
        for g in genome.iter_mut() {
            let nb: f64 = rng.gen();
            if nb < self.config.gene_reroll_proba {
                *g = rng.gen();
            } else {
                *g = (*g * (1.0 + ((nb - 0.5) * (1.0 + rate)))) % 1.0;
            }
        }
    }

    fn random_genome(&self) -> Genome {
        let mut rng = rand::thread_rng();
        self.__random_genome(&mut rng, T::get_genome_length())
    }

    fn __random_genome(&self, rng: &mut ThreadRng, len: usize) -> Genome {
        let mut res: Genome = vec![];
        for _ in 0..len {
            res.push(rng.gen());
        }
        res
    }

    fn __choose_parents(
        &self,
        p1scope: (u32, u32),
        p2scope: (u32, u32),
        rng: &mut ThreadRng,
    ) -> (usize, usize) {
        (
            rng.gen_range(p1scope.0..p1scope.1) as usize,
            rng.gen_range(p2scope.0..p2scope.1) as usize,
        )
    }

    fn __give_birth(
        &self,
        p1: &CellData,
        p2: &CellData,
        _method: &BreedingMethod,
        rng: &mut ThreadRng,
    ) -> Genome {
        assert_eq!(p1.genome.len(), p2.genome.len());
        //TODO  Use BreedingMethod enum, taking care of getting good weight if maximization or
        //minimization
        let mut genome = Genome::new();
        let sumscores: f64 = p1.score + p2.score;
        let part = (p1.score / sumscores, p2.score / sumscores);
        for g in 0..p1.genome.len() {
            let num: f64 = rng.gen();
            if num < part.0 {
                genome.push(p1.genome[g]);
            } else {
                genome.push(p2.genome[g]);
            }
        }
        genome
    }

    /*          CELLS MANIPULATION          */

    fn __generate_elite_childs(
        &self,
        elites: &Vec<&CellData>,
        genvec: &mut Vec<Genome>,
        exploration_ratio: f64,
        rng: &mut ThreadRng,
    ) {
        for i in 0..(elites.len() - 1) {
            let mut child = self.__give_birth(
                elites.get(i).unwrap(),
                elites.get(i + 1).unwrap(),
                &BreedingMethod::ScoreBasedChoose,
                rng,
            );
            self.mutate_genome(&mut child, (1.0 - exploration_ratio).powf(2.0), rng);
            genvec.push(child);
        }
    }

    fn __generate_elite_mutations(
        &self,
        elites: &Vec<&CellData>,
        size: u32,
        genvec: &mut Vec<Genome>,
        exploration_ratio: f64,
        rng: &mut ThreadRng,
    ) {
        for _ in 0..size {
            let random_cell_nb = rng.gen_range(0..elites.len());
            let mut genome = elites[random_cell_nb as usize].genome.clone();
            self.mutate_genome_direct(&mut genome, 1.0 - exploration_ratio, rng);
            genvec.push(genome)
        }
    }

    fn __generate_random_elite_childs(
        &self,
        cells: &Vec<CellData>,
        nb_elites: u32,
        size: u32,
        genvec: &mut Vec<Genome>,
        exploration_ratio: f64,
        rng: &mut ThreadRng,
    ) {
        self.__generate_childs(
            cells,
            (0, nb_elites),
            (nb_elites, cells.len() as u32),
            size,
            genvec,
            (1.0 - exploration_ratio).powf(2.0),
            rng,
            BreedingMethod::ScoreBasedAverage,
        );
    }

    fn __generate_random_pop_childs(
        &self,
        cells: &Vec<CellData>,
        nb_elites: u32,
        size: u32,
        genvec: &mut Vec<Genome>,
        exploration_ratio: f64,
        rng: &mut ThreadRng,
    ) {
        self.__generate_childs(
            cells,
            (nb_elites, cells.len() as u32),
            (nb_elites, cells.len() as u32),
            size,
            genvec,
            1.0 - exploration_ratio,
            rng,
            BreedingMethod::ScoreBasedChoose,
        );
    }

    fn __generate_norm_random_cells(
        &self,
        size: u32,
        mean_elites: &Genome,
        stddev_elites: &Genome,
        genvec: &mut Vec<Genome>,
        rng: &mut ThreadRng,
    ) {
        for _ in 0..size {
            genvec.push(normal_random_vec(mean_elites, stddev_elites, rng));
        }
    }

    fn __generate_random_cells(&self, size: u32, genvec: &mut Vec<Genome>, rng: &mut ThreadRng) {
        let genomelen = T::get_genome_length();
        for _ in 0..size {
            genvec.push(self.__random_genome(rng, genomelen));
        }
    }

    fn __generate_childs(
        &self,
        cells: &Vec<CellData>,
        p1scope: (u32, u32),
        p2scope: (u32, u32),
        nb: u32,
        genvec: &mut Vec<Genome>,
        mutrat: f64,
        rng: &mut ThreadRng,
        method: BreedingMethod,
    ) {
        for _ in 0..nb {
            let (nb1, nb2) = self.__choose_parents(p1scope, p2scope, rng);
            let mut child = self.__give_birth(
                cells.get(nb1).unwrap(),
                cells.get(nb2).unwrap(),
                &method,
                rng,
            );
            self.mutate_genome(&mut child, mutrat, rng);
            genvec.push(child);
        }
    }
}
