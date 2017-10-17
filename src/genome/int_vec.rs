//! Vector of integer values

use num::Integer;

use super::Genome;

/// Vector of integer values
pub struct IntVec<I: Integer>(Vec<I>);

impl<I: Integer> Genome for IntVec<I> {
    type Chromosome = I;

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
