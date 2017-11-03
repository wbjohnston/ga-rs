//! Evolutionary algorithm

mod traits;
pub use self::traits::EvolutionaryAlgorithm;

mod simple;
pub use self::simple::Simple;

mod mu_comma_lambda;
pub use self::mu_comma_lambda::MuCommaLambda;

mod mu_plus_lambda;
pub use self::mu_plus_lambda::MuPlusLambda;

// mod state;
// use self::state::State;
