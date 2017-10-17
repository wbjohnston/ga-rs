//! Operators for crossing genomes

use genome::Genome;
use genome::BitString;

use rand::Rng;

/// Operator for crossing two genomes to crate offspring
pub trait CrossoverOperator<Gr>
where
    Gr: Genome,
{
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(&self, g1: &Gr, g2: &Gr, rng: &mut R) -> (Gr, Gr);
}

/// Cross two genomes at one point
pub struct OnePoint;

impl OnePoint {
    pub fn new() -> Self
    {
        Self {}
    }
}

impl CrossoverOperator<BitString> for OnePoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &BitString,
        g2: &BitString,
        rng: &mut R,
    ) -> (BitString, BitString)
    {
        unimplemented!();
    }
}

/// Cross two genomes at one point
pub struct TwoPoint;

impl TwoPoint {
    pub fn new() -> Self
    {
        Self {}
    }
}

impl CrossoverOperator<BitString> for TwoPoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &BitString,
        g2: &BitString,
        rng: &mut R,
    ) -> (BitString, BitString)
    {
        unimplemented!();
    }
}
