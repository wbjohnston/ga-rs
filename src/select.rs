//! Selection genetic operators and traits

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// A genetic selection operator
pub trait SelectOp<G, O: Ord + Clone> {
    /// Return k individuals from a population
    fn select<R: Rng>(
        &self,
        pop: &[(G, O)],
        k: usize,
        rng: &mut R,
    ) -> Vec<(G, O)>;
}

/// A tournament selection genetic operator
#[derive(Debug, Copy, Clone)]
pub struct Tournament {
    size: usize,
}

impl Tournament {
    /// Create a tournament operator with specified size
    pub fn with_size(size: usize) -> Tournament
    {
        Tournament { size }
    }
}

impl<C: Clone, O: Ord + Clone> SelectOp<Vec<C>, O> for Tournament {
    fn select<R: Rng>(
        &self,
        pop: &[(Vec<C>, O)],
        k: usize,
        rng: &mut R,
    ) -> Vec<(Vec<C>, O)>
    {
        let mut selected = vec![];

        for _ in 0..k
        {
            let tourn = Random.select(pop, self.size, rng);
            let best =
                tourn.iter().cloned().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

            selected.push(best);
        }

        selected
    }
}


/// A random selection genetic operator
#[derive(Debug, Copy, Clone)]
pub struct Random;

impl<C: Clone, O: Ord + Clone> SelectOp<Vec<C>, O> for Random {
    fn select<R: Rng>(
        &self,
        pop: &[(Vec<C>, O)],
        k: usize,
        rng: &mut R,
    ) -> Vec<(Vec<C>, O)>
    {
        let mut selected = vec![];
        let range = Range::new(0, pop.len());

        for _ in 0..k
        {
            selected.push(pop[range.ind_sample(rng)].clone());
        }

        selected
    }
}
