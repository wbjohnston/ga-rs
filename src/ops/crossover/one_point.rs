//! Single point crossover

use genome::Genome;

use rand::Rng;
use rand::distributions::IndependentSample;
use rand::distributions::Range;

use super::CrossoverOperator;

#[derive(Debug, Serialize, Deserialize)]
pub struct OnePoint;

impl<C> CrossoverOperator<Vec<C>, C> for OnePoint
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
        let p = Range::new(0, g1.len()).ind_sample(rng);

        let mut g1 = g1.clone();
        let mut g2 = g2.clone();

        for i in 0..p
        {
            // swap
            let temp = g1[i].clone();
            g1[i] = g2[i].clone();
            g2[i] = temp;
        }

        (g1, g2)
    }
}
