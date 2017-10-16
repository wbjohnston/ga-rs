//! Primtives for the genetic algorithm

use bit_vec::BitVec;

/// Genetic information encoded as a bitvector
pub type Genome = BitVec;

/// Collection of Genomes
pub type Population = Vec<Genome>;
