
use Genome;
use super::SelectOperator;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Roulette;

impl<G, C, O> SelectOperator<G, C, O> for Roulette
where
    G: Genome<C>,
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    #[inline]
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
