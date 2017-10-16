//! CrossOverover operators for genetic algorithms

use rand::Rng;

use ga::traits::CrossOver;
use ga::primitives::Genome;

/// CrossOver genomes at one point
pub struct OnePoint;

impl CrossOver for OnePoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// CrossOver genomes at two points
pub struct TwoPoint;

impl CrossOver for TwoPoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// Uniform cross-over
pub struct Uniform;

impl CrossOver for Uniform {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct PartialyMatched;

impl CrossOver for PartialyMatched {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct UniformPartialyMatched;

impl CrossOver for UniformPartialyMatched {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct Ordered;

impl CrossOver for Ordered {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct Blend;

impl CrossOver for Blend {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct SimulatedBinaryBounded;

impl CrossOver for SimulatedBinaryBounded {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct MessyOnePoint;

impl CrossOver for MessyOnePoint {
    #[allow(unused_variables)]
    fn crossover<R: Rng>(
        &self,
        g1: &Genome,
        g2: &Genome,
        rng: &mut R,
    ) -> (Genome, Genome)
    {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}
