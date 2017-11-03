//! Trait for sequential representation of genomes

use bit_vec::BitVec;

// TODO(will): determine what traits ought to be required for this
/// Marker trait for structs that can be used as sequential representations of
/// a genome
pub trait Sequence: Clone {}

impl<C: Clone> Sequence for Vec<C> {}

impl Sequence for BitVec {}
