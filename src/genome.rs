//! A trait that lets us operate on structures, treating them like a "genome"
//! so we can "mutate" and "cross" them with other structures

use rand::Rng;

/// Interface for randomly mutating, crossing over, and creating a genome used
/// in genetic algorithms
pub trait Genome
{
    /// Create an entirely random `Genome`
    fn genesis<R>(rng: &mut R) -> Self
        where R: Rng;

    /// Randomly mutate a single "gene" in the genome
    fn mutate<R>(&self, rate: f32, rng: &mut R) -> Self
        where R: Rng;

    /// Cross this orgnanism with another
    ///
    /// # Arguments
    /// * `other`: the "mate" of this organism that will cross genomes
    fn cross<R>(&self, other: &Self, rng: &mut R) -> Self
        where R: Rng;

    /// Get the fitness of the organism
    fn fitness<T>(&self) -> T
        where T: Ord;
}
