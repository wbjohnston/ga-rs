//! Selection operators for genetic algorithms

use ga::traits::Select;

use rand::Rng;
use ga::primitives::Population;


/// TODO: docstring
pub struct Tournament
{
    k: usize,
    size: usize,
}

impl Select for Tournament
{
    fn select(population: &Population, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO: docstring
pub struct NSGA2
{
}

impl Select for NSGA2
{
    fn select(population: &Population, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}
