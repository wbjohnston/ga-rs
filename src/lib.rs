//! An evolutionary computing toolkit

extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate num_traits;

pub mod runner;
pub mod crossover;
pub mod mutate;
pub mod select;
