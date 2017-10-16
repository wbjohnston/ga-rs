//! Selection operators for genetic algorithms

use rand::Rng;

use ga::primitives::Population;
use ga::traits::Select;

/*
 * Notes: How to handle multi-dimensional fitness?
 */

/// Select the most fit individuals by running a tournament
pub struct Tournament {
    size: usize,
}

impl Select for Tournament {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct Roulette;

impl Select for Roulette {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct NSGA2;

impl Select for NSGA2 {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct SPEA2;

impl Select for SPEA2 {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// Randomly select k individuals
pub struct Random;

impl Select for Random {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// Deterministically select the K least fit individual
pub struct Best;

impl Select for Best {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// Deterministically select the K least fit individual
pub struct Worst;

impl Select for Worst {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct DoubleTournament; // TODO

impl Select for DoubleTournament {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct StochasticUniversalSampling;

impl Select for StochasticUniversalSampling {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct TournamentDCD;

impl Select for TournamentDCD {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}

pub struct Lexicase;

impl Select for Lexicase {
    fn select<R: Rng>(
        &self,
        population: &Population,
        k: usize,
        rng: &mut R,
    ) -> Population
    {
        unimplemented!();
    }
}
