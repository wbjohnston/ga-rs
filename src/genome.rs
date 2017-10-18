//! Genome marker trait

/// A genome encoding
pub trait Genome: Clone {}

// impls
impl Genome for Vec<u32> {}

impl Genome for Vec<u16> {}

// Float vec
impl Genome for Vec<f32> {}
