//! Crossover genetic operators and traits

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// A crossover genetic operator
pub trait CrossoverOp<G> {
    /// Return a two children of the given parent genomes
    fn crossover<R: Rng>(&self, g1: &G, g2: &G, rng: &mut R) -> (G, G);
}

/// A one point crossover genetic operator
#[derive(Debug, Copy, Clone)]
pub struct OnePoint;

impl<C: Clone> CrossoverOp<Vec<C>> for OnePoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Vec<C>,
        g2: &Vec<C>,
        rng: &mut R,
    ) -> (Vec<C>, Vec<C>)
    {
        assert_eq!(g1.len(), g2.len());
        let size = g1.len();

        let cx_point = Range::new(0, size).ind_sample(rng);

        // first half of crossed genome
        let mut c1 = g1[0..cx_point].to_vec();
        let mut c2 = g2[0..cx_point].to_vec();

        // append second half of crossed genome
        c1.extend_from_slice(&g2[cx_point..size]);
        c2.extend_from_slice(&g1[cx_point..size]);

        (c1, c2)
    }
}

/// A two point crossover genetic operator
#[derive(Debug, Copy, Clone)]
pub struct TwoPoint;

impl<C: Clone> CrossoverOp<Vec<C>> for TwoPoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Vec<C>,
        g2: &Vec<C>,
        rng: &mut R,
    ) -> (Vec<C>, Vec<C>)
    {
        unimplemented!();
    }
}
