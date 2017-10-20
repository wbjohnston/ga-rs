//! FlipBit mutation operator

use std::ops::Not;
use genome::Genome;
use rand::Rng;
use super::MutateOperator;

/// TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct FlipBit {
    pub ind_pb: f32,
}

// This operator will work on any genome with an invertible chromsome
impl<C> MutateOperator<Vec<C>, C> for FlipBit
where
    C: Clone + Sized + Not<Output = C>,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        let mut cloned = g.clone();
        let pb = (self.ind_pb * 100.0) as u32;

        for i in 0..g.len()
        {
            if rng.gen_weighted_bool(pb)
            {
                cloned[i] = !cloned[i].clone();
            }
        }

        cloned
    }
}
