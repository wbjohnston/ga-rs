//! Trait for sequential representation of genomes

// TODO(will): determine what traits out to be required for this
/// A sequential representation of a genome
pub trait Sequence: Clone
{
}

impl<C: Clone> Sequence for Vec<C> {}
