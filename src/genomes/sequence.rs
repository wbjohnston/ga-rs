//! Trait for sequential representation of genomes

use serde::{Serialize, Deserialize};

// TODO(will): determine what traits ought to be required for this
/// Marker trait for types that can be used as sequential representations of
/// a genome
pub trait Sequence<'a>: Clone + Serialize + Deserialize<'a> {}

impl<'a, C> Sequence<'a> for Vec<C>
where
    C: Clone + Serialize + Deserialize<'a>,
{
}
