use ops::traits::CrossoverOperator;

// use Genome;
use rand::Rng;

/// A crossover that uniformly swaps chromosomes of a genome.
///
/// This operator is paramaterized over the following:
/// * `ind_pb`: probability a single chromsome will be swapped
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Uniform {
    pub ind_pb: u32,
}

impl<C> CrossoverOperator<Vec<C>> for Uniform
where
    C: Sized + Clone,
{
    fn crossover<R: Rng>(
        &self,
        g1: &Vec<C>,
        g2: &Vec<C>,
        rng: &mut R,
    ) -> (Vec<C>, Vec<C>)
    {
        assert_eq!(g1.len(), g2.len(), "Genomes must be the same length");

        let mut g1 = g1.clone();
        let mut g2 = g2.clone();

        // swap chromosomes
        for i in 0..g1.len()
        {
            if rng.gen_weighted_bool(self.ind_pb)
            {
                let temp = g1[i].clone();
                g1[i] = g2[i].clone();
                g2[i] = temp;
            }
        }

        (g1, g2)
    }
}
