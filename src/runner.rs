//! Genetic algorithm runners and supporting data structures

use bit_vec::BitVec;
use rand::Rng;

pub struct Runner<R, O>
where
    R: Rng,
    O: Ord + Clone,
{
    state: State<R, O>,
}

impl<R, O> Runner<R, O>
where
    R: Rng,
    O: Ord + Clone,
{
}

pub struct State<R, O>
where
    R: Rng,
    O: Ord + Clone,
{
    population: Vec<(O, BitVec)>,
    rng: R,
}
