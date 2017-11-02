//! FlipBit mutation operator

use std::ops::Not;
use rand::Rng;
use super::MutateOperator;

/// A selection operator that performs an bitflip operator on each chromosome
/// of a genome with a given probability
#[derive(Debug, Serialize, Deserialize)]
pub struct FlipBit {
    pb: u32,
}

impl FlipBit {
    /// Create a new FlipBit selection operator with a given probability
    pub fn with_pb(pb: f32) -> Self
    {
        assert!(
            pb >= 0.0 && pb <= 1.0,
            "Probability must be a value between 0.0 and 1.0"
        );

        // conert to u32 for use with gen_weighted_bool
        let pb = (1.0 / pb) as u32;

        Self { pb }
    }
}

// This operator will work on any genome with an invertible chromsome
impl<C> MutateOperator<Vec<C>> for FlipBit
where
    C: Clone + Not<Output = C>,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        let mut cloned = g.clone();

        for i in 0..g.len()
        {
            if rng.gen_weighted_bool(self.pb)
            {
                cloned[i] = !cloned[i].clone();
            }
        }

        cloned
    }
}


#[cfg(test)]
mod test {
    use super::*;

    /// Test that trying to create a `FlipBit` operator with a probability of
    /// less than 0.0 causes a panic
    #[test]
    #[should_panic]
    fn panics_on_lt_0()
    {
        FlipBit::with_pb(-0.001);
    }

    /// Test that trying to create a `FlipBit` operator with a probability of
    /// more than 1.0 causes a panic
    #[test]
    #[should_panic]
    fn panics_on_gt_1()
    {
        FlipBit::with_pb(1.001);
    }

    /// Test that the bitflip operator works correctly
    #[test]
    fn mutates_correctly()
    {
        use rand::{StdRng, SeedableRng};

        let mut genome =
            vec![true, false, true, false, true, true, false, false];
        let pb = 0.05; // independent probability of swapping

        let seed: &[_] = &[0, 0, 0, 0];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // sample and reset rng
        let mut sampled = vec![];
        for _ in 0..genome.len()
        {
            sampled.push(rng.gen_weighted_bool((1.0 / pb) as u32));
        }
        rng.reseed(seed); // reset rng

        let original = genome.clone();
        let mutated = FlipBit::with_pb(pb).mutate(&mut genome, &mut rng);

        // verify flips
        for i in 0..genome.len()
        {
            if sampled[i]
            {
                assert_eq!(!original[i], mutated[i]);
            }
            else
            {
                assert_eq!(original[i], mutated[i]);
            }
        }

    }
}
