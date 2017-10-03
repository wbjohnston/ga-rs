//! Primitives for genetic programming algorithms

#[cfg(feature = "lgp")]
pub mod lgp;

#[cfg(feature = "cgp")]
pub mod cgp;

#[cfg(feature = "tgp")]
pub mod tgp;

#[cfg(feature = "ga")]
pub mod ga;
