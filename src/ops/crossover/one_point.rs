//! Single point crossover

use rand::Rng;
use rand::distributions::IndependentSample;
use rand::distributions::Range;

use super::CrossoverOperator;

/// A Crossover operator that crosses two genomes at a single point
#[derive(Debug, Serialize, Deserialize)]
pub struct OnePoint;

impl OnePoint {
    /// Create a new OnePoint crossover operator
    pub fn new() -> Self
    {
        Self {/* No fields */}
    }
}

impl<C> CrossoverOperator<Vec<C>, C> for OnePoint
where
    C: Clone + Sized,
{
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(
        &self,
        g1: &Vec<C>,
        g2: &Vec<C>,
        rng: &mut R,
    ) -> (Vec<C>, Vec<C>)
    {
        assert_eq!(g1.len(), g2.len(), "Genomes must be the same length");

        let p = Range::new(0, g1.len()).ind_sample(rng);

        let mut g1 = g1.clone();
        let mut g2 = g2.clone();

        // swap chromosomes
        for i in p..g1.len()
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
    fn panics_on_different_len()
    {
        use rand::thread_rng;

        // different lengths should cause a panic
        let g1 = vec![0, 1, 2, 3];
        let g2 = vec![4, 5];

        let mut rng = thread_rng();
        let cx = OnePoint;

        cx.crossover(&g1, &g2, &mut rng);
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
    /// with a pivot point of 2 should result in:
    ///
    /// ```
    ///       - Pivot
    ///       |
    ///       V
    /// | A B H I J |
    ///       | | |
    /// | F G C D E |
    /// ```
    #[test]
    fn crosses_correctly()
    {
        use rand::{StdRng, SeedableRng};
        use rand::distributions::{Range, IndependentSample};

        let g1 = vec![0, 1, 2, 3, 4];
        let g2 = vec![5, 6, 7, 8, 9];

        let seed: &[_] = &[0, 0, 0, 0];
        let range = Range::new(0, g1.len());
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // sample and reseed
        let sampled_cx_point = range.ind_sample(&mut rng);
        rng.reseed(seed);

        let cx = OnePoint;

        let (c1, c2) = cx.crossover(&g1, &g2, &mut rng);

        println!("Swap at index: {}", sampled_cx_point);
        println!("G1: {:?}\nG2: {:?}\n", g1, g2);
        println!("C1: {:?}\nC2: {:?}", c1, c2);

        // verify child crossover is correct
        for i in 0..g1.len()
        {
            if i < sampled_cx_point
            {
                // first section of genome should stay the same
                assert_eq!(g1[i], c1[i]);
                assert_eq!(g2[i], c2[i]);
            }
            else
            {
                // second part of genome should be opposite
                assert_eq!(g2[i], c1[i]);
                assert_eq!(g1[i], c2[i]);
            }
        }
    }
}
