//! Genome marker trait

/// A genome encoding
pub trait Genome<C>: Sized + Clone
where
    C: Sized + Clone,
{
}

// Basically any vector can be used as a Genome
impl<C> Genome<C> for Vec<C>
where
    C: Sized + Clone,
{
}

// impls
// Real string
// impl Genome<f32> for Vec<f32> {}

// // Bit string TODO: optmize with actual bit vector
// impl Genome<bool> for Vec<bool> {}

// #[cfg(feature = "i128_type")]
// impl Genome<i128> for Vec<i128> {}

// impl Genome<i64> for Vec<i64> {}

// impl Genome<i32> for Vec<i32> {}

// impl Genome<i16> for Vec<i16> {}

// impl Genome<i8> for Vec<i8> {}

// #[cfg(feature = "i128_type")]
// impl Genome<u128> for Vec<u128> {}

// impl Genome<u64> for Vec<u64> {}

// impl Genome<u32> for Vec<u32> {}

// impl Genome<u16> for Vec<u16> {}

// impl Genome<u8> for Vec<u8> {}
