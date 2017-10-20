//! Tournament selection operator

use Genome;
use super::SelectOperator;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};


/// Selection operator that randomly samples a specified "tournament" of
/// individuals k times, and returns the best individual from each tournament
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Tournament {
    size: usize,
}

impl Tournament {
    /// Create a new Tournament operator with a given size
    pub fn with_size(size: usize) -> Self
    {
        Self { size }
    }
}

impl<G, C, O> SelectOperator<G, C, O> for Tournament
where
    G: Genome<C>,
    C: Clone + Sized,
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
        let mut chosen = vec![];
        let range = Range::new(0, pop_with_fit.len());

        for _ in 0..k
        {
            // run K rounds of the Tournament
            let mut contestents = vec![];

            for _ in 0..self.size
            {
                let idx = range.ind_sample(rng);
                contestents.push(pop_with_fit[idx].clone());
            }

            let best = contestents
                .into_iter()
                .max_by(|a, b| a.0.cmp(&b.0))
                .unwrap()
                .1;

            chosen.push(best);
        }

        chosen
    }
}
