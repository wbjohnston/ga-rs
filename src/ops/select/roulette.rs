
use Genome;
use super::SelectOperator;
use rand::Rng;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Roulette;

impl<G, C, O> SelectOperator<G, C, O> for Roulette
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
        _: &mut R,
    ) -> Vec<G>
    {
        unimplemented!();
    }
}
