//! Selection operators

mod best;
pub use self::best::Best;

mod worst;
pub use self::worst::Worst;

mod random;
pub use self::random::Random;

mod roulette;
// pub use self::roulette::Roulette;

mod tournament;
pub use self::tournament::Tournament;
