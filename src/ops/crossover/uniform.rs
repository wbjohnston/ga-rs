use ops::traits::CrossoverOperator;

use rand::Rng;
use serde::{Serialize, Deserialize};

/// A crossover that uniformly swaps chromosomes of a genome.
///
/// This operator is paramaterized over the following:
/// * `ind_pb`: probability a single chromosome will be swapped
#[derive(Clone, Copy)]
pub struct Uniform {
    pb: u32,
}

impl Uniform {
    /// Create a new `Uniform` Crossover operator with a given probability
    pub fn with_pb(pb: f32) -> Self
    {
        assert!(
            pb >= 0.0 && pb <= 1.0,
            "Probability must be a value between 0.0 and 1.0"
        );

        // convert to u32 for use with gen_weighted_bool
        let pb = (1.0 / pb) as u32;

        Self { pb }
    }
}
impl<'a, C> CrossoverOperator<'a, Vec<C>> for Uniform
where
    C: Clone
        + Serialize
        + Deserialize<'a>,
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
            if rng.gen_weighted_bool(self.pb)
            {
                let temp = g1[i].clone();
                g1[i] = g2[i].clone();
                g2[i] = temp;
            }
        }

        (g1, g2)
    }
}
