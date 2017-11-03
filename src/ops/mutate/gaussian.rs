//! Gaussian mutate operator

use serde::{Serialize, Deserialize};

use rand::Rng;

use ops::traits::MutateOperator;

// NOTE(will): this applies only to real valued repr's

/// TODO
#[derive(Debug, Clone, Copy)]
pub struct Gaussian;

impl<'a, C> MutateOperator<'a, Vec<C>> for Gaussian
where
    C: Clone
        + Serialize
        + Deserialize<'a>,
{
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        unimplemented!();
    }
}
