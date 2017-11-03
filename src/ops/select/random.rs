//! Random selection operator

use genomes::Sequence;

use ops::traits::SelectOperator;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};

/// Selection operator that randomly select k individuals from a population
///
/// This operator can select a genome multiple times
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Random;

impl<G, O> SelectOperator<G, O> for Random
where
    G: Sequence,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>
    {
        let range = Range::new(0, pop_with_fit.len());

        // randomly select k individuals
        let mut chosen = vec![];
        for _ in 0..k
        {
            let idx = range.ind_sample(rng);
            chosen.push(pop_with_fit[idx].clone().1);
        }

        chosen
    }
}
