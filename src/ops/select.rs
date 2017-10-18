//! Operators for selecting genomes

use rand::Rng;

use individual::Individual;
use genome::Genome;

/// Operator for selecting individuals out of a population
pub trait SelectOperator<G: Genome, O: Ord + Clone> {
    /// Select k individuals from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<Individual<G, O>>,
        k: usize,
        rng: &mut R,
    ) -> Vec<Individual<G, O>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Best;

impl<G: Genome, O: Ord + Clone> SelectOperator<G, O> for Best {
    /// Select k individuals from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<Individual<G, O>>,
        k: usize,
        rng: &mut R,
    ) -> Vec<Individual<G, O>>
    {
        let mut c = population.clone();
        c.sort_by_key(|x| x.fitness());
        c.into_iter().take(k).collect()
    }
}
