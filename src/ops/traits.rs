//! Traits for implementing genetic operators

use genomes::Sequence;

use rand::Rng;

/// Operator for selecting genomes out of a population
pub trait SelectOperator<'a, G, O>
where
    G: Sequence<'a>,
    O: Clone + Ord,
{
    // TODO(will): make this operate in place
    // TODO(will): have this return meaningful data about its operation
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>;
}

/// Operator for mutating a genome
pub trait MutateOperator<'a, G>
where
    G: Sequence<'a>,
{
    // TODO(will), make operator work in place
    // TODO(will): Make this return "number of mutations(usize)"
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, indv: &G, rng: &mut R) -> G;
}

/// Operator for crossing two genomes to crate offspring
pub trait CrossoverOperator<'a, G>
where
    G: Sequence<'a>,
{
    // TODO(will): make this operator work in place
    // TODO(will): Make this return meaningful information after doing above
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(&self, g1: &G, g2: &G, rng: &mut R) -> (G, G);
}
