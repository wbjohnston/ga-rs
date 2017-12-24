// One Max problem example

extern crate petri;
extern crate rand;

use petri::runner::SimpleGARunner;
use petri::crossover::OnePoint;
use petri::select::Tournament;
use petri::mutate::BitFlip;

use rand::thread_rng;

/// Genome evaluation function, counts the number of true values in the genome
fn evaulate(g: &Vec<bool>) -> usize
{
    g.iter().filter(|&x| *x).count()
}

fn main()
{
    let mut runner = SimpleGARunner::initialize_with_fn(
        Tournament::with_size(3),
        BitFlip::with_pb(20),
        OnePoint,
        evaulate,
        20,
        20,
        thread_rng(),
        100,
        || vec![false; 1000]
    );

    for _ in 0..100
    {
        runner.advance();
        println!(
            "{}: {:?}",
            runner.generation(),
            runner.fitnesses().iter().max().unwrap()
        );
    }
}
