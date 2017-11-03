//! Simplest genetic algorithm

use std::marker::PhantomData;

use algo::EvolutionaryAlgorithm;
use ops::traits::{SelectOperator, CrossoverOperator, MutateOperator};

use rand::Rng;

use genomes::Sequence;

// TODO(will): create a `State` struct to hold runtime state of algorithm, 
//      implement serialize and deserialize on this. should be shared between
//      differente algorithm implementations

/// The simplest possible evolutationary algorithm
///
/// It's important to use a Selection operator that can select duplicate
/// individuals. Otherwise the selection operator will be useless because it
/// always selects as many individuals as are currently in the population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simple<G, S, C, M, E, R, O> 
where
    G: Sequence,
    S: SelectOperator<G, O>,
    C: CrossoverOperator<G>,
    M: MutateOperator<G>,
    E: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
{
    select_op: S,
    crossover_op: C,
    mutate_op: M,
    evaluate_op: E,

    mut_pb: u32,
    cx_pb: u32,
    population: Vec<G>,
    generation: usize,
    max_generation: usize,

    rng: R,

    _marker: PhantomData<(O)>,
}

impl<G, S, C, M, E, R, O> Simple<G, S, C, M, E, R, O>
where
    G: Sequence,
    S: SelectOperator<G, O>,
    C: CrossoverOperator<G>,
    M: MutateOperator<G>,
    E: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
{
    /// Create a new Simple EvolutionaryAlgorithm
    pub fn new(
        select_op: S,
        crossover_op: C,
        mutate_op: M,
        evaluate_op: E,
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

impl<G, S, C, M, E, R, O> EvolutionaryAlgorithm<G, S, C, M, E, R, O> 
for Simple<G, S, C, M, E, R, O> 
where
    G: Sequence,
    S: SelectOperator<G, O>,
    C: CrossoverOperator<G>,
    M: MutateOperator<G>,
    E: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
{
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
