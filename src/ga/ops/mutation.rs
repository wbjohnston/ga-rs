//! Mutation operators for genetic algorithms

use rand::Rng;

use ga::traits::Mutate;
use ga::primitives::Genome;

/// TODO
pub struct Gaussian;

impl Mutate for Gaussian {
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        unimplemented!();
    }
}

/// TODO
pub struct ShuffleIndexes;

impl Mutate for ShuffleIndexes {
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        unimplemented!();
    }
}

/// TODO
pub struct FlipBit;

impl Mutate for FlipBit {
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        unimplemented!();
    }
}

/// TODO
pub struct PolynomialBounded;

impl Mutate for PolynomialBounded {
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        unimplemented!();
    }
}

/// TODO
pub struct UniformInt;

impl Mutate for UniformInt {
    #[allow(unused_variables)]
    fn mutate<R: Rng>(&self, target: &mut Genome, rng: &mut R)
    {
        unimplemented!();
    }
}
