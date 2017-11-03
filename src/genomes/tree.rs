//! Trait for tree representation of genomes

use serde::{Serialize, Deserialize};

/// Marker trait for types that can be used as tree representations of a genome
pub trait Tree<'a>: Copy + Serialize + Deserialize<'a> {}
