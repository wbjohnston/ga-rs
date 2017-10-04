//! Primtives for the genetic algorithm

use bit_vec::BitVec;

/// Collection of Genomes
pub type Population = Vec<Genome>;

pub struct Genome
{
    /// Number of bits per chromosome
    chromosome_size: usize,
    genes: BitVec
}

impl Genome
{
    /// Create new Genome
    pub fn new(chromosome_size: usize) -> Self
    {
        Self {
            chromosome_size: chromosome_size,
            genes: BitVec::new()
        }
    }

    /// Create a fixed sized genome
    pub fn with_capacity(capacity: usize, chromosome_size: usize) -> Self
    {
        Self {
            chromosome_size: chromosome_size,
            genes: BitVec::with_capacity(capacity)
        }
    }

    /// number of chromosomes in Genome
    pub fn size(&self) -> usize
    {
        self.genes.len() / self.chromosome_size
    }
}
