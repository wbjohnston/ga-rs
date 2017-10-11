//! Selection operators for genetic algorithms

use rand::Rng;

use ga::primitives::Population;
use ga::traits::Select;

/*
 * Notes: How to handle multi-dimensional fitness?
 */

/// Select the most fit individuals by running a tournament
pub struct Tournament {
    k: usize,
    size: usize,
}

impl Select for Tournament {
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population {
        unimplemented!();
    }
}

/// TODO
pub struct Roulette
{
    k: usize,
}

impl Select for Roulette
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct NSGA2
{
    // TODO
}

impl Select for NSGA2
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct SPEA2;

impl Select for SPEA2
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// Randomly select k individuals
pub struct Random
{
    k: usize
}

impl Select for Random
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// Deterministically select the K least fit individual
pub struct Best
{
    k: usize
}

impl Select for Best
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// Deterministically select the K least fit individual
pub struct Worst
{
    k: usize
}

impl Select for Worst
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct DoubleTournament
{
    // TODO
}

impl Select for DoubleTournament
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct StochasticUniversalSampling
{
    // TODO
}

impl Select for StochasticUniversalSampling
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

/// TODO
pub struct TournamentDCD
{
    // TODO
}

impl Select for TournamentDCD
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

pub struct Lexicase
{
    // TODO
}

impl Select for Lexicase
{
    fn select(population: &Population, k: usize, rng: &mut Rng) -> Population
    {
        unimplemented!();
    }
}

