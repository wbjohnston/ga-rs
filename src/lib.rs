//! Genetic algorithm tools

extern crate rand;
use rand::Rng;

extern crate num;
use num::traits::Float;

mod genome;
pub use self::genome::Genome;

/// Genetic algorithm
///
/// # Arguments
/// * `pop_size`: number of individuals in the population
/// * `n_iter`: number of iterations to run algorithm
/// * `replace_rate`: rate at which individuals will be repleaced with results
///     of crossover
/// * `mut_rate`: rate of mutation in individuals
/// * `rng`: Random number generator that will be used to get randomness
///
/// # Return
/// The final generation of `n_iter` iterations, zipped with its fitness, sorted
/// by fitness
pub fn genetic_algorithm<G, R, O, F>(
    pop_size:     usize,
    n_iter:       usize,
    replace_rate: F,
    mutate_rate:  F,
    mut rng:      R
    ) -> Vec<(G, O)>
where G: Genome + Clone,
      R: Rng,
      O: Ord,
      F: Float
{
    // initial population, zipped with fitness and sorted
    let mut pop = init_generation(pop_size, &mut rng);

    // run survival of the fittest simulation
    for _ in 0..n_iter {
        pop = next_generation(pop, replace_rate, mutate_rate, &mut rng);
    }

    pop
}

/// Create an initial generation of specified size
///
/// # Arguments
/// * `size`: size of population
/// * `rng`: Random number generator to pull randomness from
///
/// # Return
/// An entirely random generation, sorted by fitness
fn init_generation<G, R, O>(size: usize, rng: &mut R) -> Vec<(G, O)>
where G: Genome,
      R: Rng,
      O: Ord
{
    let mut gen: Vec<(G, O)> = Vec::with_capacity(size);
    for _ in 0..size {
        let indv = G::genesis(rng);
        let fitness = indv.fitness();
        gen.push((indv, fitness));
    }

    gen.sort_by(|a, b| a.1.cmp(&b.1));
    gen
}

/// Create the next generation
///
/// # Arguments
/// * `gen`: base generation
/// * `replace_rate`: fraction of population that will be replaced by the
///     results of cross-over
/// * `mutate_rate`: Rate of mutation in individuals
/// * `rng`: Random number generator to pull randomness from
///
/// # Return
/// Successor generation to `gen, zipped with, and sorted by fitness
fn next_generation<G, R, O, F>(
    gen:          Vec<(G, O)>,
    replace_rate: F,
    mutate_rate:  F,
    rng:          R
    ) -> Vec<(G, O)>
where G: Genome + Clone,
      R: Rng,
      O: Ord,
      F: Float
{
    let mut new_gen: Vec<G> = Vec::with_capacity(gen.len());

    // let highest_idx = ((1. - replace_rate) * new_gen.len()) as usize;
    // for i in 0..highest_idx {
    //     new_gen[i] = gen[i].0.clone();
    // }
    
    // TODO: crossover

    // TODO: mutation

    new_gen.into_iter()
        .map(|x| {
            let fitness = x.fitness();
            (x, fitness)
        }).collect()
}

