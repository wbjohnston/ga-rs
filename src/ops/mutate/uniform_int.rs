//! Uniform Integer mutation operator

use ops::traits::MutateOperator;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UniformInt;

impl<C> MutateOperator<Vec<C>> for UniformInt
where
    C: Clone,
{
    /// Mutate an indiviudal
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        unimplemented!();
    }
}
