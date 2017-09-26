use rand::Rng;

/// A single unit of mutation
pub trait Chromosome {
    /// Create a Chromosome
    fn genesis(&self, rng: &mut Rng) -> Self;

    /// Mutate the Chromosome randomly
    fn mutate(&self, rng: &mut Rng) -> Self;

    /// Mutate the Chromosome in place randomly
    fn mutate_mut(&mut self, rng: &mut Rng);
}
