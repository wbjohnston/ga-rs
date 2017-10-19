//! Operators for selecting genomes

use rand::Rng;

use genome::Genome;

/// Operator for selecting genomes out of a population
pub trait SelectOperator<G, C, O>
where
    G: Genome<C>,
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        population: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Best;

impl<C, O> SelectOperator<Vec<C>, C, O> for Best
where
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, Vec<C>)>,
        k: usize,
        _: &mut R,
    ) -> Vec<Vec<C>>
    {
        let mut c = pop_with_fit.clone();
        c.sort_by(|a, b| a.0.cmp(&b.0));
        c.into_iter().map(|x| x.1).take(k).collect()
    }
}
