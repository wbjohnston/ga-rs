//! Crossover operators
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
