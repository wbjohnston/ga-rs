//! Crossover operators

use Genome;

use rand::Rng;

mod one_point;
pub use self::one_point::OnePoint;

mod two_point;
pub use self::two_point::TwoPoint;

mod uniform;
pub use self::uniform::Uniform;

mod ordered;
// pub use self::ordered::Ordered;

mod partially_matched;
// pub use self::partially_matched::PartiallyMatched;

/// Operator for crossing two genomes to crate offspring
pub trait CrossoverOperator<G, C>
where
    G: Genome<C>,
    C: Clone + Sized,
{
    /// Cross two genomes to produce two children
    fn crossover<R: Rng>(&self, g1: &G, g2: &G, rng: &mut R) -> (G, G);
}
