//! Two point crossover operator

use Genome;
use super::CrossoverOperator;
use rand::distributions::{Range, IndependentSample};
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TwoPoint;

impl<C> CrossoverOperator<Vec<C>, C> for TwoPoint
where
    C: Clone + Sized,
{
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(
        &self,
        g1: &Vec<C>,
        g2: &Vec<C>,
        rng: &mut R,
    ) -> (Vec<C>, Vec<C>)
    {
        assert_eq!(g1.len(), g2.len());

        // sample two points
        let s1 = Range::new(0, g1.len()).ind_sample(rng);
        let s2 = Range::new(0, g1.len()).ind_sample(rng);

        // sort
        let (p1, p2) = if s1 < s2 { (s1, s2) } else { (s2, s2) };

        let mut g1 = g1.clone();
        let mut g2 = g2.clone();

        // swap first segment
        for i in 0..p1
        {
            // swap
            let temp = g1[i].clone();
            g1[i] = g2[i].clone();
            g2[i] = temp;
        }

        // swap third second
        for i in p2..g1.len()
        {
            // swap
            let temp = g1[i].clone();
            g1[i] = g2[i].clone();
            g2[i] = temp;
        }

        (g1, g2)
    }
}
