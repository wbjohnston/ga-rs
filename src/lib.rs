//! An evolutionary computing toolkit
#![cfg_attr(feature="nightly", feature(test))]

#[cfg(feature="nightly")]
extern crate test;

extern crate rand;
#[cfg(feature = "serde_support")]
extern crate serde;
#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;
extern crate num_traits;
extern crate bit_vec;
extern crate itertools;

pub mod runner;
pub mod crossover;
pub mod mutate;
pub mod select;
