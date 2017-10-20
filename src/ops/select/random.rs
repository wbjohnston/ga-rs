//! Random selection operator

use Genome;
use super::SelectOperator;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Random;

impl<G, C, O> SelectOperator<G, C, O> for Random
where
    G: Genome<C>,
    C: Clone + Sized,
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
        let mut c = pop_with_fit.clone();
        rng.shuffle(&mut c);
        c.into_iter().map(|x| x.1).take(k).collect()
    }
}
