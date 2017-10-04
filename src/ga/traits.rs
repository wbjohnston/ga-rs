//! Traits used in gentic algorithms

use rand::Rng;

use ga::primitives::Genome;
use ga::primitives::Population;

/// A struct that can be evaluated for fitness, and can be converted to and from
/// a genome for evolution
pub trait Individual: From<Genome> + Into<Genome>
{
    /// Evaulate the fitness of the individual
    fn fitness<O: Ord>(&self) -> O;
}

/// Implement on structs representing a parameterized cross operator
pub trait Cross
{
    /// Cross two `Genomes` to produce a new `Genome`
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome;
}

/// Implement on structs representing a parameterized select operator
pub trait Select
{
    /// Select `Genomes` from a population
    fn select(population: &Population, rng: &mut Rng) -> Population;
}

/// Implement on structs representing a parameterized mutate operator
pub trait Mutate
{
    /// Mutate target `Genome`
    fn mutate(target: &mut Genome, rng: &mut Rng);
}
