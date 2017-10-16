//! Full genetic algorithms for selecting, crossing, and mutating a Population

use ga::traits::*;
use ga::primitives::Population;
use ga::ops::ToolBox;

use rand::Rng;

/// Simple genetic algorithm
///
/// # Arguments
/// * `pop`: Initial population
/// * `cx_pb`: Crossover probability
/// * `mut_pb`: Mutation probability
/// * `n_generations`: Number of generations to run algorithm over
/// * `toolbox`: packed operators
/// * `rng`: Random number generator to pull randomness from
pub fn simple<S, C, M, R>(
    pop: &Population,
    cx_pb: f32,
    mut_pb: f32,
    n_generations: usize,
    toolbox: &ToolBox<S, C, M>,
    rng: &mut R,
) -> Population
where
    S: Select,
    C: CrossOver,
    M: Mutate,
    R: Rng,
{
    let mut curr_pop = pop.clone();
    for _ in 0..n_generations
    {
        // select new individuals
        curr_pop = toolbox.select(&curr_pop, curr_pop.len(), rng);

        // cross_over
        let mut offspring = vec![];
        for g in curr_pop.iter()
        {
            if rng.gen_weighted_bool((cx_pb * 100.0) as u32)
            {
                // TODO: select another genome to cross with
                let (c1, c2) = toolbox.crossover(g, g, rng);
                offspring.push(c1);
                offspring.push(c2);
            }
        }

        // Add offspring to population
        let n_pop_to_keep = curr_pop.len() - offspring.len();
        curr_pop = curr_pop.into_iter().take(n_pop_to_keep).collect();
        curr_pop.extend(offspring.into_iter());

        // Mutate
        for g in curr_pop.iter_mut()
        {
            if rng.gen_weighted_bool((mut_pb * 100.00) as u32)
            {
                toolbox.mutate(g, rng);
            }
        }
    }

    curr_pop
}

/// (μ + λ) genetic algorithm
///
/// # Arugments
/// * `init_pop`: initial population
/// * `mu`: TODO
/// * `lambda`: TODO
/// * `cx_pb`: crossover probability
/// * `mut_pb`: mutation probability
/// * `n_generations`: number of generations to run algorithm
/// * `toolbox`: Packed operators
/// * `rng`: Random number generator to pull randomness from
#[allow(unused_variables)]
pub fn mu_plus_lambda<S, C, M, R>(
    init_pop: &Population,
    mu: usize,
    lambda: usize,
    cx_pb: f32,
    mut_pb: f32,
    n_generations: usize,
    toolbox: &ToolBox<S, C, M>,
    rng: &mut R,
) -> Population
where
    S: Select,
    C: CrossOver,
    M: Mutate,
    R: Rng,
{
    unimplemented!();
}

/// (μ + λ) genetic algorithm
///
/// # Arugments
/// * `init_pop`: initial population
/// * `mu`: TODO
/// * `lambda`: TODO
/// * `cx_pb`: crossover probability
/// * `mut_pb`: mutation probability
/// * `n_generations`: number of generations to run algorithm
/// * `toolbox`: Packed operators
/// * `rng`: Random number generator to pull randomness from
#[allow(unused_variables)]
pub fn mu_comma_lambda<S, C, M, R>(
    pop: &Population,
    mu: usize,
    lambda: usize,
    cx_pb: f32,
    mut_pb: f32,
    n_generations: usize,
    toolbox: &ToolBox<S, C, M>,
    rng: &mut R,
) -> Population
where
    S: Select,
    C: CrossOver,
    M: Mutate,
    R: Rng,
{
    unimplemented!();
}
