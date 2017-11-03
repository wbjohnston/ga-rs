//! Best selection operator

use rand::Rng;
use ops::traits::SelectOperator;

use genomes::Sequence;

/// Selection operator that selects the k best individual from a population
#[derive(Debug, Copy, Clone)]
pub struct Best;

impl<'a, G, O> SelectOperator<'a, G, O> for Best
where
    G: Sequence<'a>,
    O: Clone + Ord,
{
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, G)>,
        k: usize,
        _: &mut R,
    ) -> Vec<G>
    {
        let mut c = pop_with_fit.clone();
        c.sort_by(|a, b| a.0.cmp(&b.0));
        c.into_iter().map(|x| x.1).take(k).collect()
    }
}
