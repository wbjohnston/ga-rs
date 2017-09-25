//! A trait that lets us operate on structures, treating them like a "genome"
//! so we can "mutate" and "cross" them with other structures

use std::iter::Iterator;

use rand::Rng;

/// A single unit of mutation
pub trait Codon
{
    /// Mutate the codon randomly
    fn mutate<R>(&self, rng: &mut R) -> Self
        where R: Rng;

    /// Mutate the codon in place randomly
    fn mutate_mut<R>(&mut self, rng: &mut R)
        where R: Rng;
}

/// Interface for randomly mutating, crossing over, and creating a genome used
/// in genetic algorithms
pub trait Genome
{
    /// Immutabel codon iterator
    fn codon_iter<C>(&self) -> Iterator<Item=C>
        where C: Codon;

    /// Mutable Codon iterator
    fn codon_iter_mut<C>(&mut self) -> Iterator<Item=&mut C>
        where C: Codon;

    /// Create an entirely random `Genome`
    fn genesis<R>(rng: &mut R) -> Self
        where R: Rng;

    /// Randomly mutate a single "gene" in the genome
    fn mutate<R>(&self, rate: f32, rng: &mut R) -> Self
        where R: Rng;

    // TODO: cross can be implemented entirely using the codon iterator methods,
    // providing a default implementation
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
