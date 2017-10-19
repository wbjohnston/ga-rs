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
        let range = Range::new(0, g1.len());

        let p = range.ind_sample(rng) as usize;

        let (mut c1l, c1r) = (g1[0..p].to_vec(), g1[p..g1.len()].to_vec());
        let (mut c2l, c2r) = (g2[0..p].to_vec(), g2[p..g2.len()].to_vec());

        c1l.extend(c2r);
        c2l.extend(c1r);

        (c1l, c2l)
    }
}

