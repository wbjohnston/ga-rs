//! Operators for selecting genomes

use rand::Rng;

use genome::Genome;

/// Operator for selecting individuals out of a population
pub trait SelectOperator<Gr>
where
    Gr: Genome,
{
    /// Select k individuals from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<Gr>,
        k: usize,
        rng: &mut R,
    ) -> Vec<Gr>;
}
