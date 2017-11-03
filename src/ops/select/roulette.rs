
use ops::traits::SelectOperator;

use genomes::Sequence;
use rand::Rng;

/// TODO(will): description
#[derive(Default, Debug, Clone, Copy)]
pub struct Roulette;

impl<'a, G, O> SelectOperator<'a, G, O> for Roulette
where
    G: Sequence<'a>,
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
