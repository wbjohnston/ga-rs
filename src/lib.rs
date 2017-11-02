//! Evolutionary algorithm toolkit
#![feature(test)]
extern crate test;

extern crate bit_vec;

extern crate rand;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "par")]
extern crate rayon;

pub mod ops;

pub mod algo;

pub mod traits;
