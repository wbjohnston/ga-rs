//! A trait that lets us operate on structures, treating them like a "genome"
//! so we can "mutate" and "cross" them with other structures

use rand::Rng;

/// Interface for randomly mutating, crossing over, and creating a genome used
/// in genetic algorithms
pub trait Individual {
    /// Create an entirely random `Genome`
    ///
    /// # Arguments
    /// * `rng`: Random number generator to pull randomness from
    fn genesis(rng: &mut Rng) -> Self;

    /// Randomly mutate the `Genome`
    ///
    /// # Argument
    /// * `rate`: probability (0.0, 1.0) of each codon mutating
    /// * `rng`: Random number generator to pull randomness from
    ///
    /// # Return
    /// New mutated genome
    fn mutate(&self, rate: f32, rng: &mut Rng) -> Self;

    /// Cross this genome with another
    ///
    /// # Arguments
    /// * `other`: the "mate" of this organism that will cross genomes
    /// * `rng`: Random number generator to pull randomness from
    ///
    /// # Return
    /// TODO
    fn cross(&self, other: &Self, rng: &mut Rng) -> Self;

    /// Get the fitness of the organism
    fn fitness<O>(&self) -> O
    where
        O: Ord;
}
