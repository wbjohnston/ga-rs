
extern crate rand;
use rand::thread_rng;

extern crate petri;
use petri::genome::Genome;
use petri::individual::Individual;

use petri::algo::EvolutionaryAlgorithm;
use petri::ops::select::Best;
use petri::ops::crossover::OnePoint;
use petri::ops::mutate::FlipBit;
use petri::ops::evaluate::EvaluateOperator;

use petri::algo::Simple;

type Genes = Vec<u32>;

struct OneMaxEval;

impl EvaluateOperator<Genes, u32> for OneMaxEval {
    /// Calculate the fitness of a genome
    fn evaluate(&self, indv: &Individual<Genes, u32>) -> u32
    {
        indv.genome().iter().map(|x| x.count_ones()).sum()
    }
}

fn main()
{
    let sel = Best;
    let cx = OnePoint;
    let mt = FlipBit {ind_pb: 0.05};
    let eval = OneMaxEval;

    let rng = thread_rng();

    let init_fn = || Individual::from(vec![0, 0, 0, 0]);

    let mut ea = Simple::new(sel, cx, mt, eval, 0.01, 0.05, 100_000, rng);

    ea.initialize(5, init_fn);

    while !ea.is_done()
    {
        let pop = ea.next();
    }

    let final_pop = ea.population();

    for indv in final_pop {
        println!("{:?}", indv);
    }
}
