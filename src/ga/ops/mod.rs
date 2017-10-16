//! Operators for genetic algorithms

use ga::traits::{CrossOver, Select, Mutate};
use ga::primitives::{Genome, Population};

use rand::Rng;

pub mod selection;

pub mod crossover;

pub mod mutation;

pub struct ToolBox<S, C, M>
where
    S: Select,
    M: Mutate,
    C: CrossOver,
{
    select: S,
    mutate: M,
    crossover: C,
    // evaluate: fn(Genome) -> f32
}


impl<S, C, M> ToolBox<S, C, M>
where
    S: Select,
    C: CrossOver,
    M: Mutate,
{
    /// Create a new ToolBox
    pub fn new(select: S, crossover: C, mutate: M) -> Self
    {
        Self {
            select,
            mutate,
            crossover,
        }
    }

    /// Select k genomes from a population using the packed selection operator
    pub fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        self.select.select(population, k, rng)
    }

    /// Mutate a genome using the packed mutate operator
    pub fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        self.mutate.mutate(target, rng)
    }

    /// Cross over two genomes using the packed crossover operator
    pub fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        self.crossover.crossover(g1, g2, rng)
    }
}
