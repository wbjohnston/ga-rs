//! Vector of float values

use super::Genome;

use num::traits::Float;

/// Vector of float values
pub struct FloatVec<F: Float>(Vec<F>);

impl<F: Float> Genome for FloatVec<F> {
    type Chromosome = F;

    #[allow(unused_variables)]
    fn chromosomes(&self) -> Vec<Self::Chromosome>
    {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn get_chromosome(&self, idx: usize) -> Self::Chromosome
    {
        unimplemented!();
    }
}
