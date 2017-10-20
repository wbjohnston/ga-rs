//! Uniform Integer mutation operator

use super::MutateOperator;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UniformInt;

impl<C> MutateOperator<Vec<C>, C> for UniformInt
where
    C: Clone + Sized,
{
    /// Mutate an indiviudal
    #[inline]
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        unimplemented!();
    }
}
