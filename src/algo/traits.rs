//! Traits for implementing evolutationary algorithms

use rand::Rng;

use genomes::Sequence;

use ops::traits::SelectOperator;
use ops::traits::CrossoverOperator;
use ops::traits::MutateOperator;

/// An evolutationary algorithm
pub trait EvolutionaryAlgorithm<'a, G, S, C, M, E, R, O>
where
    G: Sequence<'a>,
    S: SelectOperator<'a, G, O>,
    C: CrossoverOperator<'a, G>,
    M: MutateOperator<'a, G>,
    E: Fn(&G) -> O,
    R: Rng,
    O: Ord + Clone,
{
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
