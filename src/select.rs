//! Selection genetic operators and traits

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// A genetic selection operator
pub trait SelectOp<G, O: Ord + Clone> {
    /// Return k individuals from a population
    ///
    /// # Panics
    /// panics if `k` is greater than the length of pop
    fn select<R: Rng>(&self, population: &[(G, O)], k: usize, rng: &mut R) -> Vec<(G, O)>;
}

/// A tournament selection genetic operator
///
/// Selects k individuals by running k tournaments and selecting the best individuals
#[derive(Debug, Copy, Clone)]
pub struct Tournament {
    size: usize,
}

impl Tournament {
    /// Create a tournament operator with specified size
    pub fn with_size(size: usize) -> Tournament {
        Tournament { size }
    }
}

impl<G: Clone, O: Ord + Clone> SelectOp<G, O> for Tournament {
    fn select<R: Rng>(&self, population: &[(G, O)], k: usize, rng: &mut R) -> Vec<(G, O)> {
        assert!(k <= population.len());
        let mut selected = vec![];

        for _ in 0..k {
            let tourn = Random.select(population, self.size, rng);
            let best = tourn.iter().cloned().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            selected.push(best);
        }

        selected
    }
}


/// A random selection genetic operator
///
/// Selects k random individuals from the population. Can select the same individual mutliple times
#[derive(Debug, Copy, Clone)]
pub struct Random;

impl<G: Clone, O: Ord + Clone> SelectOp<G, O> for Random {
    fn select<R: Rng>(&self, population: &[(G, O)], k: usize, rng: &mut R) -> Vec<(G, O)> {
        assert!(k <= population.len());
        let mut selected = vec![];
        let range = Range::new(0, population.len());

        for _ in 0..k {
            selected.push(population[range.ind_sample(rng)].clone());
        }

        selected
    }
}

/// A best selection genetic operator
///
/// Returns the k best individuals from a population
#[derive(Debug, Copy, Clone)]
pub struct Best;

impl<G: Clone, O: Ord + Clone> SelectOp<G, O> for Best {
    fn select<R: Rng>(&self, population: &[(G, O)], k: usize, _rng: &mut R) -> Vec<(G, O)> {
        assert!(k <= population.len());
        let mut population = population.to_vec();
        population.sort_by(|a, b| a.1.cmp(&b.1));
        population[0..k].to_vec()
    }
}
