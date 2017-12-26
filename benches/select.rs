#![cfg_attr(not(feature="criterion_support"), allow(dead_code, unused_imports))]

extern crate petri;
#[cfg(feature="criterion_support")]
#[macro_use]
extern crate criterion;
extern crate rand;

use petri::select::{Best, Tournament, SelectOp, Random};
use rand::{XorShiftRng, SeedableRng, Rng};
#[cfg(feature="criterion_support")]
use criterion::Criterion;

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

#[cfg(feature="criterion_support")]
fn tournament_benchmark(c: &mut Criterion) 
{
    // setup
    let selector = Tournament::with_size(5);
    let population = generate_population(100, 100);

    c.bench_function(
        "Tournament",
        |b| b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
            selector.select(&population, population.len(), &mut rng)
        })
    );
}

#[cfg(feature="criterion_support")]
fn best_benchmark(c: &mut Criterion)
{
    // setup
    let selector = Best;
    let population = generate_population(100, 100);

    c.bench_function(
        "Best",
        |b| b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
            selector.select(&population, population.len(), &mut rng)
        })
    );
}

#[cfg(feature="criterion_support")]
fn random_benchmark(c: &mut Criterion)
{
    // setup
    let selector = Random;
    let population = generate_population(100, 100);

    c.bench_function(
        "Random",
        |b| b.iter(|| {
            let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
            selector.select(&population, population.len(), &mut rng)
        })
    );
}

#[cfg(feature="criterion_support")]
criterion_group!(
    select_benches,
    tournament_benchmark,
    best_benchmark,
    random_benchmark
    );
#[cfg(feature="criterion_support")]
criterion_main!(select_benches);

#[cfg(not(feature="criterion_support"))]
fn main() {}



