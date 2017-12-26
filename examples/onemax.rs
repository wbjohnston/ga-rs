// One Max problem example

extern crate petri;
extern crate rand;

use petri::runner::SimpleGARunner;
use petri::crossover::TwoPoint;
use petri::select::Tournament;
use petri::mutate::BitFlip;

use rand::thread_rng;

/// Genome evaluation function, counts the number of true values in the genome
fn evaulate(g: &Vec<bool>) -> usize {
    g.iter().filter(|&x| *x).count()
}

fn main() {
    let n_codons = 100;

    let mut runner = SimpleGARunner::initialize_with_fn(
        Tournament::with_size(5),
        BitFlip::with_pb(0.01),
        TwoPoint,
        evaulate,
        0.05,
        0.01,
        thread_rng(),
        100,
        || vec![false; n_codons]
    );

    let mut best_fitness = 0;
    while best_fitness < n_codons {
        runner.advance();
        best_fitness = *runner.fitnesses().iter().max().unwrap();
        println!("{}: {:?}", runner.generation(), best_fitness);
    }
}
