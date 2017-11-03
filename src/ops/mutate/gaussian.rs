//! Gaussian mutate operator

// NOTE: this applies only to real valued repr's

use ops::traits::MutateOperator;

use rand::Rng;

use serde::{Serialize, Deserialize};

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
