//! Selection operators

use Genome;
use rand::Rng;

mod best;
pub use self::best::Best;

mod worst;
pub use self::worst::Worst;

mod random;
pub use self::random::Random;

mod roulette;
// pub use self::roulette::Roulette;

mod tournament;
pub use self::tournament::Tournament;

/// Operator for selecting genomes out of a population
///
/// # Arguments
/// TODO
pub trait SelectOperator<G, C, O>
where
    G: Genome<C>,
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>;
}
