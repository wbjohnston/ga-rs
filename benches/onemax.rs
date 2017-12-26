/// One max problem benchmark

// TODO(will): I'd like a cleaner way of disabling this benchmark when criterion is not available

#[cfg(feature="criterion_support")]
#[macro_use]
extern crate criterion;
extern crate petri;
extern crate rand;

use rand::{XorShiftRng, SeedableRng};

#[cfg(feature="criterion_support")]
use criterion::Criterion;

use petri::runner::SimpleGARunner;
use petri::select::Tournament;
use petri::crossover::TwoPoint;
use petri::mutate::BitFlip;

#[cfg_attr(not(feature="criterion_support"), allow(dead_code))]
/// Returns the number of generations it took to reach peak fitness
fn onemax_population_size(size: usize, n_codons: usize) -> usize {
    fn evaluate(genome: &Vec<bool>) -> usize {
        genome.iter().filter(|&x| *x).count()
    }

    let rng = XorShiftRng::from_seed([1, 2, 3, 4]);
    let mut runner = SimpleGARunner::initialize_with_fn(
        Tournament::with_size(3),
        BitFlip::with_pb(0.01),
        TwoPoint,
        evaluate,
        0.05,
        0.01,
        rng,
        size,
        || vec![false; n_codons]
    );

    let mut best_fitness = 0;
    while best_fitness < n_codons {
        runner.advance();
        best_fitness = *runner.fitnesses().iter().max().unwrap();
    }

    runner.generation()
}

#[cfg(feature="criterion_support")]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "onemax 100",
        |b| b.iter(|| onemax_population_size(100, 100)),
    );
}

#[cfg(feature="criterion_support")]
criterion_group!(benches, criterion_benchmark);

#[cfg(feature="criterion_support")]
criterion_main!(benches);

#[cfg(not(feature="criterion_support"))]
fn main () {
    println!("WARNING: criterion_support feature must be enabled. Ignoring benchmark");
}
