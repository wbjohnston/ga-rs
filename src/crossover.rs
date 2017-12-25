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
    fn crossover<R: Rng>(&self, g1: &Vec<C>, g2: &Vec<C>, rng: &mut R) -> (Vec<C>, Vec<C>) {
        assert_eq!(g1.len(), g2.len());
        let size = g1.len();

        let cx_point = Range::new(0, size).ind_sample(rng);

        let mut c1 = g1.clone();
        let mut c2 = g2.clone();

        for i in cx_point..size {
            c1[i] = g2[i].clone();
            c2[i] = g1[i].clone();
        }

        (c1, c2)
    }
}

/// A two point crossover genetic operator
#[derive(Debug, Copy, Clone)]
pub struct TwoPoint;

impl<C: Clone> CrossoverOp<Vec<C>> for TwoPoint {
    fn crossover<R: Rng>(&self, g1: &Vec<C>, g2: &Vec<C>, rng: &mut R) -> (Vec<C>, Vec<C>) {
        assert_eq!(g1.len(), g2.len());
        let size = g1.len();
        let range = Range::new(0, size);

        // get crossover points
        let (p1, p2) = (range.ind_sample(rng), range.ind_sample(rng));
        let (cx1, cx2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

        let mut c1 = g1.clone();
        let mut c2 = g2.clone();

        for i in cx1..cx2 {
            c1[i] = g2[i].clone();
            c2[i] = g1[i].clone();
        }

        (c1, c2)
    }
}
