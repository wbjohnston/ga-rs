//! Traits used in gentic algorithms

use rand::Rng;

use ga::primitives::Genome;
use ga::primitives::Population;

/// A struct that can be evaluated for fitness, and can be converted to and from
/// a genome for evolution
pub trait Individual {
    /// Evaulate the fitness of the individual
    fn fitness<O: Ord>(&self) -> O;
}

/// Implement on structs representing a parameterized cross operator
pub trait Cross {
    /// Cross two `Genomes` to produce a new `Genome`, producing two children
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome);
}

/// Implement on structs representing a parameterized select operator
pub trait Select {
    /// Select k `Genomes` from a population
    fn select<R: Rng>(&self, population: &Population, k: usize, rng: &mut R) -> Population;
}

/// Implement on structs representing a parameterized mutate operator
pub trait Mutate {
    /// Mutate target `Genome`
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R);
}
