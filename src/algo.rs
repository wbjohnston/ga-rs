//! Evolutionary algorithm

use std::marker::PhantomData;

use rand::Rng;

use ops::evaluate::EvaluateOperator;
use ops::select::SelectOperator;
use ops::crossover::CrossoverOperator;
use ops::mutate::MutateOperator;

use genome::Genome;

pub trait EvolutionaryAlgorithm<
    G: Genome<C>,
    C: Clone + Sized,
    SOp: SelectOperator<G, C, O>,
    COp: CrossoverOperator<G, C>,
    MOp: MutateOperator<G, C>,
    EOp: EvaluateOperator<G, C, O>,
    R: Rng,
    O: Ord + Clone,
> {
    /// Initialize the algorithm by generating a population
    fn initialize(&mut self, n: usize, init_fn: fn() -> G);

    /// Advance to next generation
    fn next(&mut self) -> Vec<G>;

    /// Current generation
    fn population(&self) -> Vec<G>;

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simple<
    G: Genome<C>,
    C: Clone + Sized,
    SOp: SelectOperator<G, C, O>,
    COp: CrossoverOperator<G, C>,
    MOp: MutateOperator<G, C>,
    EOp: EvaluateOperator<G, C, O>,
    R: Rng,
    O: Ord + Clone,
> {
    select_op: SOp,
    crossover_op: COp,
    mutate_op: MOp,
    evaluate_op: EOp,

    mut_pb: f32,
    cx_pb: f32,
    population: Vec<G>,
    generation: usize,
    max_generation: usize,

    rng: R,

    _marker: PhantomData<(O, C)>
}

impl<
    G: Genome<C>,
    C: Clone + Sized,
    SOp: SelectOperator<G, C, O>,
    COp: CrossoverOperator<G, C>,
    MOp: MutateOperator<G, C>,
    EOp: EvaluateOperator<G, C, O>,
    R: Rng,
    O: Ord + Clone,
> Simple<G, C, SOp, COp, MOp, EOp, R, O> {
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
            _marker: PhantomData
        }
    }
}

impl<
    G: Genome<C>,
    C: Clone + Sized,
    SOp: SelectOperator<G, C, O>,
    COp: CrossoverOperator<G, C>,
    MOp: MutateOperator<G, C>,
    EOp: EvaluateOperator<G, C, O>,
    R: Rng,
    O: Ord + Clone,
> EvolutionaryAlgorithm<G, C, SOp, COp, MOp, EOp, R, O>
    for Simple<G, C, SOp, COp, MOp, EOp, R, O> {
    /// Initialize the algorithm by generating a population
    fn initialize(&mut self, n: usize, init_fn: (fn() -> G))
    {
        self.population = (0..n).map(|_| init_fn()).collect()
    }

    /// Advance to next generation
    fn next(&mut self) -> Vec<G>
    {
        let pop = self.population();

        // evaluate
        let pop_with_fit: Vec<(O, G)> = pop.into_iter()
            .map(|x| {
                let fitness = self.evaluate_op.evaluate(&x);
                (fitness, x)
            })
            .collect();

        // select phase
        let mut offspring =
            self.select_op.select(&pop_with_fit, pop_with_fit.len(), &mut self.rng);

        // mutate phase
        for i in 0..offspring.len()
        {
            if self.rng.gen_weighted_bool((self.mut_pb * 100.0) as u32)
            {
                let mutated =
                    self.mutate_op.mutate(&offspring[i], &mut self.rng);
                offspring[i] = mutated;
            }
        }

        // crossover phase
        for i in 0..(offspring.len() / 2)
        {
            if self.rng.gen_weighted_bool((self.cx_pb * 100.0) as u32)
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

    /// Current generation
    fn population(&self) -> Vec<G>
    {
        self.population.clone()
    }

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool
    {
        self.generation >= self.max_generation
    }
}
