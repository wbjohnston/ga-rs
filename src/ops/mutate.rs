//! Operators for mutating genomes

use rand::Rng;

use genome::Genome;

/// Operator for mutating a genome
pub trait MutateOperator<Gr>
where
    Gr: Genome,
{
    /// Mutate a genome
    fn mutate<R: Rng>(g: &Gr, rng: &mut R) -> Gr;
}
