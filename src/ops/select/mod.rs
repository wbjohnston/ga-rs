//! Selection operators

use rand::Rng;

use traits::Sequence;

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
pub trait SelectOperator<G, O>
where
    G: Sequence,
    O: Clone + Ord,
{
    // TODO(will): make this operate in place
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>;
}
