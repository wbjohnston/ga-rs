//! Shuffle indexes mutation operator

use super::MutateOperator;
use Genome;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ShuffleIndexes {
    pub ind_pb: f32,
}

impl<C> MutateOperator<Vec<C>, C> for ShuffleIndexes
where
    C: Clone + Sized,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        let (mut shuffled, mut cloned) = (g.clone(), g.clone());
        rng.shuffle(&mut shuffled);

        let pb = (100. * self.ind_pb) as u32;

        for i in 0..g.len()
        {
            if rng.gen_weighted_bool(pb)
            {
                cloned[i] = shuffled[i].clone();
            }
        }

        cloned
    }
}
