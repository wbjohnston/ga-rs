
use ops::traits::SelectOperator;

use genomes::Sequence;
use rand::Rng;

/// TODO(will): description
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Roulette;

impl<G, O> SelectOperator<G, O> for Roulette
where
    G: Sequence,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    #[allow(unused_variables)]
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>
    {
        unimplemented!();
    }
}
