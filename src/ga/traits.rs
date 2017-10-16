//! Traits used in gentic algorithms

use rand::Rng;

use ga::primitives::Genome;
use ga::primitives::Population;

/// Implement on structs representing a parameterized cross operator
pub trait CrossOver {
    /// Cross two `Genomes` to produce a new `Genome`, producing two children
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome);
}

/// Implement on structs representing a parameterized select operator
pub trait Select {
    /// Select k `Genomes` from a population
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population;
}

/// Implement on structs representing a parameterized mutate operator
pub trait Mutate {
    /// Mutate target `Genome`
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R);
}
