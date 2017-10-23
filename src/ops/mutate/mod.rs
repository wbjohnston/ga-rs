//! Mutation operators

use Genome;

use rand::Rng;

mod flip_bit;
pub use self::flip_bit::FlipBit;

mod gaussian;
// pub use self::gaussian::Gaussian;

mod shuffle_indexes;
pub use self::shuffle_indexes::ShuffleIndexes;

mod uniform_int;
// pub use self::uniform_int::UniformInt;

/// Operator for mutating a genome
pub trait MutateOperator<G, C>
where
    G: Genome<C>,
    C: Clone + Sized,
{
    // TODO(will), make operator work in place
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, indv: &G, rng: &mut R) -> G;
}
