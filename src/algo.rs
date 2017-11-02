//! Evolutionary algorithm

#[cfg(feature = "par")]
use rayon::prelude::*;

use std::marker::PhantomData;

use rand::Rng;

use ops::select::SelectOperator;
use ops::crossover::CrossoverOperator;
use ops::mutate::MutateOperator;

use traits::Sequence;

/// An evolutationary algorithm
pub trait EvolutionaryAlgorithm<
    G: Sequence,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G>,
    MOp: MutateOperator<G>,
    EOp: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
> {
    /// Initialize the algorithm by generating a population using a generator fn
    fn initialize<F>(&mut self, n: usize, init_fn: F)
    where
        F: Fn() -> G;

    /// Advance to next generation
    fn next(&mut self) -> Vec<G>;

    /// Current generation
    fn population(&self) -> Vec<G>;

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool;
}

/// The simplest possible evolutationary algorithm
///
/// It's important to use a Selection operator that can select duplicate
/// individuals. Otherwise the selection operator will be useless because it
/// always selects as many individuals as are currently in the population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simple<
    G: Sequence,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G>,
    MOp: MutateOperator<G>,
    EOp: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
> {
    select_op: SOp,
    crossover_op: COp,
    mutate_op: MOp,
    evaluate_op: EOp,

    mut_pb: u32,
    cx_pb: u32,
    population: Vec<G>,
    generation: usize,
    max_generation: usize,

    rng: R,

    _marker: PhantomData<(O)>,
}

impl<
    G: Sequence,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G>,
    MOp: MutateOperator<G>,
    EOp: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
> Simple<G, SOp, COp, MOp, EOp, R, O> {
    /// Create a new Simple EvolutionaryAlgorithm
    pub fn new(
        select_op: SOp,
        crossover_op: COp,
        mutate_op: MOp,
        evaluate_op: EOp,
        mut_pb: f32,
        cx_pb: f32,
        max_generation: usize,
        rng: R,
    ) -> Self
    {
        assert!(cx_pb + mut_pb <= 1.0 && cx_pb + mut_pb >= 0.0);
        let cx_pb = (1.0 / cx_pb) as u32;
        let mut_pb = (1.0 / mut_pb) as u32;

        Self {
            select_op,
            crossover_op,
            mutate_op,
            evaluate_op,

            mut_pb,
            cx_pb,
            population: vec![],
            generation: 0,
            max_generation,
            rng,
            _marker: PhantomData,
        }
    }
}

impl<
    G: Sequence,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G>,
    MOp: MutateOperator<G>,
    EOp: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
> EvolutionaryAlgorithm<G, SOp, COp, MOp, EOp, R, O>
    for Simple<G, SOp, COp, MOp, EOp, R, O> {
    fn initialize<F>(&mut self, n: usize, init_fn: F)
    where
        F: Fn() -> G,
    {
        self.population = (0..n).map(|_| init_fn()).collect()
    }

    fn next(&mut self) -> Vec<G>
    {
        let pop = self.population();

        // evaluate
        let pop_with_fit: Vec<(O, G)> = pop.into_iter()
            .map(|x| {
                let fitness = (self.evaluate_op)(&x);
                (fitness, x)
            })
            .collect();

        // select phase
        let mut offspring = self.select_op.select(
            &pop_with_fit,
            pop_with_fit.len(),
            &mut self.rng,
        );

        // mutate phase
        for indv in &mut offspring
        {
            if self.rng.gen_weighted_bool(self.mut_pb)
            {
                *indv = self.mutate_op.mutate(indv, &mut self.rng);
            }
        }

        // crossover phase
        for i in 0..(offspring.len() / 2)
        {
            if self.rng.gen_weighted_bool(self.cx_pb)
            {
                let (idx1, idx2) = (i * 2, (i * 2) + 1);

                let (c1, c2) = self.crossover_op.crossover(
                    &offspring[idx1],
                    &offspring[idx2],
                    &mut self.rng,
                );

                offspring[idx1] = c1;
                offspring[idx2] = c2;
            }
        }

        self.population = offspring;
        self.generation += 1;

        // return new generation
        self.population()
    }

    fn population(&self) -> Vec<G>
    {
        self.population.clone()
    }

    fn is_done(&self) -> bool
    {
        self.generation >= self.max_generation
    }
}
