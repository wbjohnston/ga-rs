//! Crossover operators

use ::Genome;

use rand::Rng;

mod one_point;
pub use self::one_point::OnePoint;

/// Operator for crossing two genomes to crate offspring
pub trait CrossoverOperator<G, C>
where
    G: Genome<C>,
    C: Clone + Sized,
{
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(&self, i1: &G, i2: &G, rng: &mut R) -> (G, G);
}

