
use Genome;

use rand::Rng;

mod flip_bit;
pub use self::flip_bit::FlipBit;

/// Operator for mutating a genome
pub trait MutateOperator<G, C>
where
    G: Genome<C>,
    C: Clone + Sized,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, indv: &G, rng: &mut R) -> G;
}
