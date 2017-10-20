//! Gaussian mutate operator

use super::MutateOperator;
use Genome;
use rand::Rng;

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Gaussian;

impl<C> MutateOperator<Vec<C>, C> for Gaussian
where
    C: Clone + Sized,
{
    /// Mutate an indiviudal
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        unimplemented!();
    }
}
