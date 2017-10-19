//! Genome marker trait

/// A genome encoding
pub trait Genome<C>: Sized + Clone
where
    C: Sized + Clone
{
}

// Basically any vector can be used as a Genome
impl<C> Genome<C> for Vec<C>
where
    C: Sized + Clone,
{
}
