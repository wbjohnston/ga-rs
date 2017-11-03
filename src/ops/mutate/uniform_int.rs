//! Uniform Integer mutation operator

use ops::traits::MutateOperator;
use rand::Rng;
use serde::{Serialize, Deserialize};

/// TODO
#[derive(Debug, Clone, Copy)]
pub struct UniformInt;

impl<'a, C> MutateOperator<'a, Vec<C>> for UniformInt
where
    C: Clone
        + Serialize
        + Deserialize<'a>,
{
    /// Mutate an individual
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        unimplemented!();
    }
}
