//! Two point crossover operator

use ops::traits::CrossoverOperator;
use rand::distributions::{Range, IndependentSample};
use rand::Rng;

use serde::{Serialize, Deserialize};

/// A crossover operator that crosses two genomes at two points
#[derive(Clone, Copy, Default)]
pub struct TwoPoint;

impl<'a, C> CrossoverOperator<'a, Vec<C>> for TwoPoint
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

        // sample two pivot points
        let range = Range::new(0, g1.len());
        let s1 = range.ind_sample(rng);
        let s2 = range.ind_sample(rng);

        // sort
        let (p1, p2) = if s1 < s2 { (s1, s2) } else { (s2, s1) };

        let mut g1 = g1.clone();
        let mut g2 = g2.clone();

        // swap middle segment
        for i in p1..p2
        {
            let temp = g1[i].clone();
            g1[i] = g2[i].clone();
            g2[i] = temp;
        }

        (g1, g2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that if the operator is given two genomes of different lengths it
    /// will panic
    #[test]
    #[should_panic]
    fn test_panics_on_different_len()
    {
        use rand::thread_rng;

        // different lengths should cause a panic
        let g1 = vec![0, 1, 2, 3];
        let g2 = vec![4, 5];

        let mut rng = thread_rng();
        TwoPoint.crossover(&g1, &g2, &mut rng);
    }

    /// Test that the `OnePoint` operator swaps chromosomes correctly
    ///
    /// Given a genome of:
    ///
    /// ```
    /// | A B C D E |
    /// | F G H I J |
    /// ```
    ///
    /// with a pivot point of 1 and 3 should result in:
    ///
    /// ```
    ///     ------- Pivots
    ///     |   |
    ///     v   v
    /// | A G C I J |
    ///     | |
    /// | F B H D E |
    /// ```
    #[test]
    fn test_crosses_correctly()
    {
        use rand::{StdRng, SeedableRng};
        use rand::distributions::{Range, IndependentSample};

        let g1 = vec![0, 1, 2, 3, 4];
        let g2 = vec![5, 6, 7, 8, 9];

        let seed: &[_] = &[0, 0, 0, 0];
        let range = Range::new(0, g1.len());
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // sample and reseed
        let s1 = range.ind_sample(&mut rng);
        let s2 = range.ind_sample(&mut rng);

        // pivots
        let (p1, p2) = if s1 < s2 { (s1, s2) } else { (s2, s1) };

        rng.reseed(seed); // reset rng

        let (c1, c2) = TwoPoint.crossover(&g1, &g2, &mut rng);

        println!("Swap between [{}-{})", p1, p2);
        println!("G1: {:?}\nG2: {:?}\n", g1, g2);
        println!("C1: {:?}\nC2: {:?}", c1, c2);

        // verify child crossover is correct
        for i in 0..g1.len()
        {
            if i >= p1 && i < p2
            // if we're in the swap interval
            {
                // check opposites
                assert_eq!(g2[i], c1[i]);
                assert_eq!(g1[i], c2[i]);
            }
            else
            {
                // otherwise check the same
                assert_eq!(g1[i], c1[i]);
                assert_eq!(g2[i], c2[i]);
            }
        }
    }
}
