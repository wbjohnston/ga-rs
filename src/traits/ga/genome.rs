//! A collection of chromosomes

use bit_vec::BitVec;

/// Interface for randomly mutating, crossing over, and creating a genome used
/// in genetic algorithms
pub trait Genome: From<BitVec> + Into<BitVec>
{
    /// Get the fitness of the organism
    fn fitness<O>(&self) -> O
    where
        O: Ord;
}
