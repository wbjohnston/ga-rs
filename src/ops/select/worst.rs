//! Worst selection operator

use traits::Sequence;

use super::SelectOperator;
use rand::Rng;

/// Selection operator that selects `k` individuals with the lowest fitness
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Worst;

impl<G, O> SelectOperator<G, O> for Worst
where
    G: Sequence,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, G)>,
        k: usize,
        _: &mut R,
    ) -> Vec<G>
    {
        let mut c = pop_with_fit.clone();
        c.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        c.into_iter().map(|x| x.1).take(k).collect()
    }
}
