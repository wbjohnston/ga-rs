
extern crate rand;
use rand::thread_rng;

extern crate petri;
use petri::genome::Genome;

use petri::algo::EvolutionaryAlgorithm;
use petri::ops::select::Best;
use petri::ops::crossover::OnePoint;
use petri::ops::mutate::FlipBit;
use petri::ops::evaluate::EvaluateOperator;

use petri::algo::Simple;

type Genes = Vec<Chromosome>;
type Chromosome = bool;
type Fitness = u32;

fn main()
{
    let sel = Best;
    let cx = OnePoint;
    let mt = FlipBit { ind_pb: 0.05 };

    let rng = thread_rng();

    let init_fn = || vec![false; 32];
    let eval = |g: &Genes| {
        g.iter().filter(|x| **x).count() as u32
    };

    let eval1 = |g: &Genes| {
        g.iter().filter(|x| **x).count() as u32
    };

    let mut ea = Simple::new(sel, cx, mt, eval, 0.01, 0.05, 10000, rng);

    ea.initialize(300, init_fn);

    while !ea.is_done()
    {
        let pop = ea.next();
    }

    let final_pop = ea.population();

    println!("BEST GENOME");
    // println!("{:?}", &final_pop[0]);
    println!("Fitness: {}", eval1(&final_pop[0]));
}
