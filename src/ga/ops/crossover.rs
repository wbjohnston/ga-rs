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

/// Uniform cross-over
pub struct Uniform
{
    pb_cx: f32
}

impl Cross for Uniform
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct PartialyMatched;

impl Cross for PartialyMatched
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct UniformPartialyMatched;

impl Cross for UniformPartialyMatched
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct Ordered;

impl Cross for Ordered
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct Blend
{
    alpha: f32
}

impl Cross for Blend
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct SimulatedBinaryBounded
{
    eta: f32,
    low: f32,
    up: f32
}

impl Cross for SimulatedBinaryBounded
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

/// TODO
pub struct MessyOnePoint;

impl Cross for MessyOnePoint
{
    fn cross(g1: &Genome, g2: &Genome, rng: &mut Rng) -> Genome {
        unimplemented!();
    }
}

