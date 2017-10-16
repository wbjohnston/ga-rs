
use ga::primitives::{Genome, Population};

use rand::Rng;

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

/// Implement on structs representing a parameterized evaluate operator
pub trait Evaluate {
    fn evaluate<O: Ord>(&self, g: &Genome) -> O;
}
