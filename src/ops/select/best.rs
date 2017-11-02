//! Best selection operator

use rand::Rng;
use super::SelectOperator;

use traits::Sequence;

/// Selction operator that selects the k best individual from a population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Best;

impl<G, O> SelectOperator<G, O> for Best
where
    G: Sequence,
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
