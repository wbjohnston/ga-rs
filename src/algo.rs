//! Evolutionary algorithm

use rand::Rng;

use ops::evaluate::EvaluateOperator;
use ops::select::SelectOperator;
use ops::crossover::CrossoverOperator;
use ops::mutate::MutateOperator;

use individual::Individual;
use genome::Genome;

pub trait EvolutionaryAlgorithm<
    G: Genome,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G, O>,
    MOp: MutateOperator<G, O>,
    EOp: EvaluateOperator<G, O>,
    R: Rng,
    O: Ord + Clone,
> {
    /// Initialize the algorithm by generating a population
    fn initialize(&mut self, n: usize, init_fn: fn() -> Individual<G, O>);

    /// Advance to next generation
    fn next(&mut self) -> Vec<Individual<G, O>>;

    /// Current generation
    fn population(&self) -> Vec<Individual<G, O>>;

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simple<
    G: Genome,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G, O>,
    MOp: MutateOperator<G, O>,
    EOp: EvaluateOperator<G, O>,
    R: Rng,
    O: Ord + Clone,
> {
    select_op: SOp,
    crossover_op: COp,
    mutate_op: MOp,
    evaluate_op: EOp,

    mut_pb: f32,
    cx_pb: f32,
    population: Vec<Individual<G, O>>,
    generation: usize,
    max_generation: usize,

    rng: R,
}

impl<
    G: Genome,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G, O>,
    MOp: MutateOperator<G, O>,
    EOp: EvaluateOperator<G, O>,
    R: Rng,
    O: Ord + Clone,
> Simple<G, SOp, COp, MOp, EOp, R, O> {
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
        }
    }
}

impl<
    G: Genome,
    SOp: SelectOperator<G, O>,
    COp: CrossoverOperator<G, O>,
    MOp: MutateOperator<G, O>,
    EOp: EvaluateOperator<G, O>,
    R: Rng,
    O: Ord + Clone,
> EvolutionaryAlgorithm<G, SOp, COp, MOp, EOp, R, O>
    for Simple<G, SOp, COp, MOp, EOp, R, O> {
    /// Initialize the algorithm by generating a population
    fn initialize(&mut self, n: usize, init_fn: (fn() -> Individual<G, O>))
    {
        self.population = (0..n).map(|_| init_fn()).collect()
    }

    /// Advance to next generation
    fn next(&mut self) -> Vec<Individual<G, O>>
    {
        let mut pop = self.population();

        // validate fitness
        pop.iter_mut().filter(|x| !x.is_valid()).for_each(|x| {
            let fitness = self.evaluate_op.evaluate(x);
            x.validate(fitness);
        });


        // select phase
        let mut offspring =
            self.select_op.select(&pop, pop.len(), &mut self.rng);

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

        // validate fitness of new offspring
        offspring.iter_mut().filter(|x| !x.is_valid()).for_each(
            |x| {
                let fitness = self.evaluate_op.evaluate(x);
                x.validate(fitness);
            },
        );

        self.population = offspring;
        self.generation += 1;

        // return new generation
        self.population()
    }

    /// Current generation
    fn population(&self) -> Vec<Individual<G, O>>
    {
        self.population.clone()
    }

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool
    {
        self.generation >= self.max_generation
    }
}
