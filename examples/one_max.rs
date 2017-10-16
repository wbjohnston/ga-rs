
use std::env::args;

extern crate rand;
use rand::{thread_rng, Rng};

extern crate bit_vec;
use bit_vec::BitVec;

extern crate ga;
use ga::ga::algorithms::simple;
use ga::ga::primitives::Genome;

use ga::ga::ops::selection::Best;
use ga::ga::ops::crossover::OnePoint;
use ga::ga::ops::mutation::Gaussian;

const DEFAULT_CX_PB: f32 = 0.05;
const DEFAULT_MUT_PB: f32 = 0.01;
const DEFAULT_N_GEN: usize = 40;

fn main()
{
    let select = Best;
    let cx = OnePoint;
    let muta = Gaussian;

    let mut args = args();

    let n_gen = args.nth(1).unwrap().parse().unwrap_or(DEFAULT_N_GEN);

    let cx_pb = args.nth(2).unwrap().parse().unwrap_or(DEFAULT_CX_PB);

    let mut_pb = args.nth(3).unwrap().parse().unwrap_or(DEFAULT_MUT_PB);

    let mut rng = thread_rng();

    let init_pop = vec![];
    let pop = simple(
        &init_pop,
        cx_pb,
        mut_pb,
        n_gen,
        &select,
        &cx,
        &muta,
        &mut rng,
    );

    println!("{:?}", pop);
}
