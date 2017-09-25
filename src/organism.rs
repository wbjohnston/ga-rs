use std::cmp::PartialOrd;

pub trait Genome
{
    /// Create an entirely random `Organism`
    fn genesis() -> Self;

    /// Randomly mutate this organism
    fn mutate(&self) -> Self;

    /// Cross this orgnanism with another
    fn cross(&self, other: &Self) -> Self;

    /// Get the fitness of the organism
    fn fitness<T>(&self) -> T
        where T: PartialOrd;
}
