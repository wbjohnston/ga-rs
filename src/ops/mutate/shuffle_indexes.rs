//! Shuffle indexes mutation operator

use super::MutateOperator;
use rand::Rng;

/// A mutation operator that swaps chromosomes of a genome with a given
/// probability
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ShuffleIndexes {
    pb: u32,
}

impl ShuffleIndexes {
    /// Create a new ShuffleIndexes selection operator with a given probability
    pub fn with_pb(pb: f32) -> Self
    {
        assert!(
            pb >= 0.0 && pb <= 1.0,
            "Probability must be a value between 0.0 and 1.0"
        );

        let pb = (1.0 / pb) as u32;

        Self { pb }
    }
}

impl<C> MutateOperator<Vec<C>, C> for ShuffleIndexes
where
    C: Clone + Sized,
{
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        let (mut shuffled, mut cloned) = (g.clone(), g.clone());
        rng.shuffle(&mut shuffled);

        for i in 0..g.len()
        {
            if rng.gen_weighted_bool(self.pb)
            {
                cloned[i] = shuffled[i].clone();
            }
        }

        cloned
    }
}
