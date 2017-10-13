//! Crossover operators for genetic algorithms

use rand::Rng;

use ga::traits::Cross;
use ga::primitives::Genome;

/// Cross genomes at one point
pub struct OnePoint;

impl Cross for OnePoint {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// Cross genomes at two points
pub struct TwoPoint;

impl Cross for TwoPoint {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// Uniform cross-over
pub struct Uniform;

impl Cross for Uniform {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct PartialyMatched;

impl Cross for PartialyMatched {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct UniformPartialyMatched;

impl Cross for UniformPartialyMatched {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct Ordered;

impl Cross for Ordered {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct Blend;

impl Cross for Blend {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct SimulatedBinaryBounded;

impl Cross for SimulatedBinaryBounded {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}

/// TODO
pub struct MessyOnePoint;

impl Cross for MessyOnePoint {
    fn cross<R: Rng>(&self, g1: &Genome, g2: &Genome, rng: &mut R) -> (Genome, Genome) {
        assert_eq!(g1.len(), g2.len());
        unimplemented!();
    }
}
