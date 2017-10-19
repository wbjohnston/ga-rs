//! Evolutionary algorithm toolkit

extern crate len_trait;

extern crate rand;

extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod ops;

pub mod genome;
pub use self::genome::Genome;

pub mod algo;


