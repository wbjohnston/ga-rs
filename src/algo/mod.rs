
use rand::Rng;
use traits::Individual;

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
pub fn ga<G, R, O>(
    pop_size: usize,
    n_iter: usize,
    replace_rate: f32,
    mutate_rate: f32,
    mut rng: R,
) -> Vec<(G, O)>
where
    G: Individual + Clone,
    R: Rng,
    O: Ord,
{
    // initial population, zipped with fitness and sorted
    let pop = init_generation(pop_size, &mut rng);

    ga_with_pop(pop, n_iter, replace_rate, mutate_rate, rng)
}

pub fn ga_with_pop<G, R, O>(
    mut pop: Vec<(G, O)>,
    n_iter: usize,
    replace_rate: f32,
    mutate_rate: f32,
    mut rng: R,
) -> Vec<(G, O)>
where
    G: Individual + Clone,
    R: Rng,
    O: Ord,
{
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
where
    G: Individual,
    R: Rng,
    O: Ord,
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
fn next_generation<G, R, O>(
    gen: Vec<(G, O)>,
    replace_rate: f32,
    mutate_rate: f32,
    rng: &mut R,
) -> Vec<(G, O)>
where
    G: Individual + Clone,
    R: Rng,
    O: Ord,
{
    let mut new_gen: Vec<G> = Vec::with_capacity(gen.len());

    // Copy over the top performers
    let highest_idx = ((1f32 - replace_rate) * new_gen.len() as f32) as usize;
    for i in 0..highest_idx {
        new_gen[i] = gen[i].0.clone();
    }

    // create a mapping between candidates, this gaurentees no "self-love" will
    // occur
    let crosses = new_gen.len() - highest_idx;
    let mut mapping: Vec<usize> = (0..crosses).collect();
    rng.shuffle(&mut mapping);

    // cross over the top candidates with eachother
    for (i, j) in mapping.into_iter().enumerate() {
        // TODO: I would like this to not clone the two operands
        let (a, b) = (new_gen[i].clone(), new_gen[j].clone());
        new_gen.push(a.cross(&b, rng));
    }

    // Mutate step
    for e in new_gen.iter_mut() {
        e.mutate(mutate_rate, rng);
    }

    let mut zipped_with_fit: Vec<(G, O)> = new_gen
        .into_iter()
        .map(|x| {
            let fitness = x.fitness();
            (x, fitness)
        })
        .collect();

    zipped_with_fit.sort_by(|a, b| a.1.cmp(&b.1));
    zipped_with_fit
}
