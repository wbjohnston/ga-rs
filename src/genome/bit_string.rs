
use bit_vec::BitVec;

use super::Genome;

/// A genome encoded as a sequence of bits
pub struct BitString(BitVec);

impl Genome for BitString {
    type Chromosome = bool;

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
