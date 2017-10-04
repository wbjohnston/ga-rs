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

#[cfg(feature = "tgp")]
pub mod tgp;

#[cfg(feature = "lgp")]
pub mod lgp;

#[cfg(feature = "ga")]
pub mod ga;

#[cfg(feature = "cgp")]
pub mod cgp;

