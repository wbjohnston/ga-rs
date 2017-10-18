//! Operators for crossing genomes

use genome::Genome;
use individual::Individual;

use rand::Rng;
use rand::distributions::IndependentSample;
use rand::distributions::Range;

/// Operator for crossing two genomes to crate offspring
pub trait CrossoverOperator<G: Genome, O: Ord + Clone> {
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(
        &self,
        i1: &Individual<G, O>,
        i2: &Individual<G, O>,
        rng: &mut R,
    ) -> (Individual<G, O>, Individual<G, O>);
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OnePoint;

impl<O: Ord + Clone> CrossoverOperator<Vec<u32>, O> for OnePoint {
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(
        &self,
        i1: &Individual<Vec<u32>, O>,
        i2: &Individual<Vec<u32>, O>,
        rng: &mut R,
    ) -> (Individual<Vec<u32>, O>, Individual<Vec<u32>, O>)
    {
        let g1 = i1.genome();
        let g2 = i2.genome();

        let range = Range::new(0, g1.len());

        let p = range.ind_sample(rng);

        let (mut c1l, c1r) = (g1[0..p].to_vec(), g1[p..g1.len()].to_vec());
        let (mut c2l, c2r) = (g2[0..p].to_vec(), g2[p..g1.len()].to_vec());

        c1l.extend(c2r);
        c2l.extend(c1r);

        (Individual::from(c1l), Individual::from(c2l))
    }
}
