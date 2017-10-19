//! Selection operators

use ::Genome;

use rand::Rng;

mod best;
pub use self::best::Best;

/// Operator for selecting genomes out of a population
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
