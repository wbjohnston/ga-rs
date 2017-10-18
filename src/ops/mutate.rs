//! Operators for mutating genomes

use genome::Genome;
use individual::Individual;

use rand::Rng;

/// Operator for mutating a genome
pub trait MutateOperator<G: Genome, O: Ord + Clone> {
    /// Mutate an indiviudal
    fn mutate<R: Rng>(
        &self,
        indv: &Individual<G, O>,
        rng: &mut R,
    ) -> Individual<G, O>;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FlipBit
{
    pub ind_pb: f32
}

impl<O: Ord + Clone> MutateOperator<Vec<u32>, O> for FlipBit {
    /// Mutate an indiviudal
    fn mutate<R: Rng>(
        &self,
        indv: &Individual<Vec<u32>, O>,
        rng: &mut R,
    ) -> Individual<Vec<u32>, O>
    {
        let mut c = indv.genome();

        for chrom in c.iter_mut() {
            if rng.gen_weighted_bool((self.ind_pb * 100.0) as u32) {
                *chrom = !*chrom;
            }
        }

        Individual::from(c)
    }
}
