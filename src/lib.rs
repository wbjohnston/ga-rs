//! Genetic algorithm traits, primitives, and algorithms

extern crate rand;

#[cfg(feature = "ga")]
extern crate bit_vec;

#[cfg(any(feature = "lgp", feature = "cgp"))]
extern crate petgraph;

#[cfg(any(feature = "gp-par", feature = "ga-par"))]
extern crate rayon;

#[cfg(feature = "serde-g")]
extern crate serde;

pub mod algo;

pub mod primitives;

pub mod traits;
