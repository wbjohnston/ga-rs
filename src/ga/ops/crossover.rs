//! Crossover operators for genetic algorithms

use ga::traits::Cross;
use rand::Rng;
use ga::primitives::Genome;

/// Cross genomes at one point
pub struct OnePoint;

impl Cross for OnePoint {
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// Cross genomes at two points
pub struct TwoPoint;

impl Cross for TwoPoint {
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}
