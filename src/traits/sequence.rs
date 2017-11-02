//! Trait for sequential representation of genomes

use std::ops::IndexMut;

// TODO(will): determine what traits ought to be required for this
/// A sequential representation of a genome
pub trait Sequence: 
    Clone +
    IndexMut<usize>
{}

impl<C: Clone> Sequence for Vec<C> {}
