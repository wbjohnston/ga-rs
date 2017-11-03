//! Gaussian mutate operator

// NOTE: this applies only to real valued repr's

use ops::traits::MutateOperator;

use bit_vec::BitVec;

use rand::Rng;

/// TODO
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gaussian;

impl<C> MutateOperator<Vec<C>> for Gaussian
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

impl MutateOperator<BitVec> for Gaussian
{
    /// Mutate an indiviudal
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, indv: &BitVec, rng: &mut R) -> BitVec
    {
        unimplemented!();
    }
}
