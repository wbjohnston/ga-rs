
extern crate rand;
use rand::thread_rng;

extern crate petri;

use petri::algo::EvolutionaryAlgorithm;
use petri::ops::select::Best;
use petri::ops::crossover::OnePoint;
use petri::ops::mutate::FlipBit;

use petri::algo::Simple;

type Genome = Vec<Chromosome>;
type Chromosome = bool;
type Fitness = u32;

#[inline]
fn fitness_fn(g: &Genome) -> Fitness
{
     g.iter().filter(|x| **x).count() as u32
}

fn main()
{
    let sel = Best;
    let cx = OnePoint;
    let mt = FlipBit { ind_pb: 0.05 };

    let rng = thread_rng();

    let init_fn = || vec![false; 32];


    let mut ea = Simple::new(sel, cx, mt, fitness_fn, 0.01, 0.05, 1000, rng);

    ea.initialize(300, init_fn);

    while !ea.is_done()
    {
        let _ = ea.next();
    }

    let final_pop = ea.population();

    println!("BEST GENOME");
    // println!("{:?}", &final_pop[0]);
    println!("Fitness: {}", fitness_fn(&final_pop[0]));
}
