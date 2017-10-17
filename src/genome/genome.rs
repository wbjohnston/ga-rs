//! Genome marker trait

/// A genome encoding
pub trait Genome {
    /// Chromosome type
    type Chromosome;

    /// Get all chromosomes
    fn chromosomes(&self) -> Vec<Self::Chromosome>;

    /// Get single chromosome
    fn get_chromosome(&self, idx: usize) -> Self::Chromosome;
}
