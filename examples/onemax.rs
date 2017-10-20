//! One max toy example
//!
//! Given a bitstring, attempt to flip as many bits as possible to true

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate rand;
use rand::thread_rng;

extern crate petri;
use petri::algo::{Simple, EvolutionaryAlgorithm};
use petri::ops::select::Tournament;
use petri::ops::crossover::TwoPoint;
use petri::ops::mutate::FlipBit;

use std::process;

type Genome = Vec<Chromosome>;
type Chromosome = bool;
type Fitness = u32;

/// Count the number of true values in a bool vec
#[inline]
fn fitness_fn(g: &Genome) -> Fitness
{
    g.iter().filter(|x| **x).count() as u32
}

fn main()
{
    let matches = App::new("onemax")
        .arg(
            Arg::with_name("generations")
                .long("generations")
                .short("g")
                .takes_value(true)
                .default_value("100"),
        )
        .arg(
            Arg::with_name("population")
                .long("population")
                .short("p")
                .takes_value(true)
                .default_value("100"),
        )
        .get_matches();

    let n_generations = value_t!(matches, "generations", usize)
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse argument: generations");
            process::exit(1);
        });

    let n_population =
        value_t!(matches, "population", usize).unwrap_or_else(|e| {
            eprintln!("Failed to parse argument: population");
            process::exit(1);
        });

    let rng = thread_rng();

    let init_fn = || vec![false; 65];

    let mut ea = Simple::new(
        Tournament { size: 10 },
        TwoPoint,
        FlipBit { ind_pb: 20 },
        fitness_fn,
        0.02,
        0.05,
        n_generations,
        rng,
    );

    // run algorithm
    ea.initialize(n_population, init_fn);
    while !ea.is_done()
    {
        let _ = ea.next();
    }

    let final_pop = ea.population();
    // get best genome

    let max = final_pop
        .into_iter()
        .map(|x| (fitness_fn(&x), x))
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    // convert to a bitstring for easy reading
    let as_bit_str = max.1
        .iter()
        .map(|x| if *x { "1" } else { "0" })
        .collect::<String>();

    println!("BEST GENOME");
    println!("Genome: [{}]", as_bit_str);
    println!("Number of Ones -- Fitness: {:?}", max.0);
    println!("Max Fitness: 65");
}
