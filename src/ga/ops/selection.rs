//! Selection operators for genetic algorithms

use ga::traits::Select;

use rand::Rng;
use ga::primitives::Population;

/// Select the most fit individuals by running a tournament
pub struct Tournament {
    k: usize,
    size: usize,
}

impl Select for Tournament {
    fn select(population: &Population, rng: &mut Rng) -> Population {
        unimplemented!();
    }
}

// pub struct NSGA2
// {
// }

// impl Select for NSGA2
// {
//     fn select(population: &Population, rng: &mut Rng) -> Population
//     {
//         unimplemented!();
//     }
// }
