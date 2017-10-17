//! Evolutionary algorithm

use std::marker::PhantomData;

use rand::Rng;

use ops::select::SelectOperator;
use ops::crossover::CrossoverOperator;
use ops::mutate::MutateOperator;
use ops::evaluate::EvaluteOperator;

use individual::Individual;
use genome::Genome;

pub trait EvolutionaryAlgorithm<Gt, It, SOp, COp, MOp, EOp, O>
where
    Gt: Genome,
    It: Individual,
    SOp: SelectOperator<Gt>,
    COp: CrossoverOperator<Gt>,
    MOp: MutateOperator<Gt>,
    EOp: EvaluteOperator<It, O>,
    O: Ord
{
/// Advance to next generation
    fn next(&mut self) -> Result<(), ()>;

/// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool;

/// Current generation
    fn population(&self) -> Vec<Gt>;
}

/// Simple genetic algorithm
#[derive(Debug, Serialize, Deserialize)]
pub struct Simple<Gt, It, SOp, COp, MOp, EOp, R, O>
where
    Gt: Genome + Clone,
    It: Individual,
    SOp: SelectOperator<Gt>,
    COp: CrossoverOperator<Gt>,
    MOp: MutateOperator<Gt>,
    EOp: EvaluteOperator<It, O>,
    R: Rng,
    O: Ord,
{
    generation: usize,
    max_generation: usize,
    population: Vec<Gt>,

    select_op: SOp,
    crossover_op: COp,
    mutate_op: MOp,
    evaluate_op: EOp,
    rng: R,

    _marker: PhantomData<(It, O)>,
}

impl <Gt, It, SOp, COp, MOp, EOp, R, O> Simple<Gt, It, SOp, COp, MOp, EOp, R, O>
where
    Gt: Genome + Clone,
    It: Individual,
    SOp: SelectOperator<Gt>,
    COp: CrossoverOperator<Gt>,
    MOp: MutateOperator<Gt>,
    EOp: EvaluteOperator<It, O>,
    R: Rng,
    O: Ord
{
    pub fn new(
        population: Vec<Gt>,
        select_op: SOp,
        crossover_op: COp,
        mutate_op: MOp,
        evaluate_op: EOp,
        max_generation: usize,
        rng: R
    ) -> Self
    {
        Self {
            generation: 0,
            max_generation: max_generation,
            population: population,

            select_op: select_op,
            crossover_op: crossover_op,
            mutate_op: mutate_op,
            evaluate_op: evaluate_op,

            rng: rng,
            _marker: PhantomData,
        }
    }
}

impl<
    Gt,
    It,
    SOp,
    COp,
    MOp,
    EOp,
    R,
    O,
> EvolutionaryAlgorithm<Gt, It, SOp, COp, MOp, EOp, O>
    for Simple<Gt, It, SOp, COp, MOp, EOp, R, O>
where
    Gt: Genome + Clone,
    It: Individual,
    SOp: SelectOperator<Gt>,
    COp: CrossoverOperator<Gt>,
    MOp: MutateOperator<Gt>,
    EOp: EvaluteOperator<It, O>,
    R: Rng,
    O: Ord,
{
    /// advance to next generation
    fn next(&mut self) -> Result<(), ()>
    {
        let new_pop = self.select_op.select(
            &self.population,
            self.population.len(),
            &mut self.rng,
        );

        // TODO: actually do stuff

        self.population = new_pop;
        self.generation += 1;

        Ok(())
    }

    /// Is the evolutationary algorithm done?
    fn is_done(&self) -> bool
    {
        self.generation > self.max_generation
    }

    /// current generation
    fn population(&self) -> Vec<Gt>
    {
        self.population.clone()
    }
}
