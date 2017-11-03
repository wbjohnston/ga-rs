//! Shuffle indexes mutation operator

use ops::traits::MutateOperator;
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

impl<C> MutateOperator<Vec<C>> for ShuffleIndexes
where
    C: Clone,
{
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        // create a map to shuffle indexes with
        let mut idxs: Vec<_> = (0..g.len()).collect();
        rng.shuffle(&mut idxs);

        let mut cloned = g.clone();

        for i in 0..g.len()
        {
            if rng.gen_weighted_bool(self.pb)
            {
                cloned[i] = g[idxs[i]].clone();
            }
        }

        cloned
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that the operator can only be created with a valid probability
    #[test]
    #[should_panic]
    fn test_panics_on_lt_0()
    {
        ShuffleIndexes::with_pb(-0.001);
    }

    /// Test that the operator can only be created with a valid probability
    #[test]
    #[should_panic]
    fn test_panics_on_gt_1()
    {
        ShuffleIndexes::with_pb(1000.0);
    }
}
