//! Selection genetic operators and traits

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// A genetic selection operator
pub trait SelectOp<G, O: Ord + Clone> {
    /// Return k individuals from a population
    ///
    /// # Panics
    /// panics if `k` is greater than the length of `population`
    fn select<R: Rng>(&self, population: &[(G, O)], k: usize, rng: &mut R) -> Vec<(G, O)>;
}

/// A selection genetic operator that runs tournaments and selects the best individual from each
///
/// This operator can select the same individual multiple times
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


/// A selection genetic operator that randomly selects individuals.
///
/// This operator can select the same individual multiple times
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

/// A selection genetic operator that selects the best individuals from the population
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


#[cfg(test)]
mod test {
    use super::*;

    use rand::XorShiftRng;

    /// `Tournament::select` should panic if k is larger than the size of the population
    #[test]
    #[should_panic]
    fn tournament_panics_with_invalid_k()
    {
        let pop = vec![(0, 0), (1, 0), (3, 0)];
        let mut rng = XorShiftRng::new_unseeded();
        Tournament::with_size(3).select(&pop, 5, &mut rng);
    }

}

#[cfg(all(feature="nightly", test))]
mod bench {
    use super::*;
    use rand::{SeedableRng, XorShiftRng};
    use test::Bencher;

    // NOTE: all benchmarks include the instantiation of the XorShiftRng. If this wasn't the case
    // different rounds of the benchmark would take different execution paths, leading to 
    // an unrepresentative increase in runtime variance.

    fn generate_population(size: usize, n_chromosomes: usize) -> Vec<(Vec<bool>, usize)>
    {
        let mut population: Vec<(Vec<bool>, usize)> = vec![];
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);

        for _ in 0..size {
            let individual: Vec<bool> = rng.gen_iter::<bool>().take(n_chromosomes).collect();
            let fitness = individual.iter().filter(|&x| *x).count();

            population.push((individual, fitness));
        }

        population
    }

    /// Bench the `Tournament`  operator with a population size of 100
    #[bench]
    fn tournament_100_xorshift(b: &mut Bencher)
    {
        let population = generate_population(100, 100);
        let selector = Tournament::with_size(3);

        b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
           selector.select(&population, population.len(), &mut rng)
       });
    }

    /// Bench the `Random` operator with a population size of a 100
    #[bench]
    fn random_100_xorshift(b: &mut Bencher)
    {
        let population = generate_population(100, 100);
        let selector = Random;

        b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
            selector.select(&population, population.len(), &mut rng)
        });
    }

    /// Bench the `Best` operator with a population size of a 100
    #[bench]
    fn best_100_xorshift(b: &mut Bencher)
    {
        let population = generate_population(100, 100);
        let selector = Best;

        b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
            selector.select(&population, population.len(), &mut rng)
        });
    }
}
