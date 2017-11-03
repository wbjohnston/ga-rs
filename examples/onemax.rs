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
                .help("Number of generations")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("population")
                .long("population")
                .short("p")
                .takes_value(true)
                .help("Number of individuals in the population")
                .default_value("100"),
        )
        .arg(
            Arg::with_name("bits")
                .long("bits")
                .short("b")
                .takes_value(true)
                .help("Number of bits in each genome")
                .default_value("65"),
        )
        .arg(
            Arg::with_name("tournament-size")
                .long("tournament-size")
                .short("t")
                .takes_value(true)
                .help("Number of individuals in each round of tournament")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("crossover-pb")
                .long("crossover-pb")
                .takes_value(true)
                .help("Probability individuals in the population will mate")
                .default_value("0.05"),
        )
        .arg(
            Arg::with_name("mutate-pb")
                .long("mutate-pb")
                .takes_value(true)
                .help("Probability individuals will mutate")
                .default_value("0.02"),
        )
        .arg(
            Arg::with_name("flip-pb")
                .long("flip-pb")
                .takes_value(true)
                .help("Probability a chromsome will be flipped during mutation")
                .default_value("0.02"),
        )
        .get_matches();


    let n_generations = value_t!(matches, "generations", usize)
        .unwrap_or_else(|_| {
            eprintln!("Failed to parse argument: generations");
            process::exit(1);
        });

    let n_population =
        value_t!(matches, "population", usize).unwrap_or_else(|_| {
            eprintln!("Failed to parse argument: population");
            process::exit(1);
        });

    let n_bits = value_t!(matches, "bits", usize).unwrap_or_else(|_| {
        eprintln!("Failed to parse argument: bits");
        process::exit(1);
    });

    let tourn_size = value_t!(matches, "tournament-size", usize)
        .unwrap_or_else(|_| {
            eprintln!("Failed to parse argument: tourn-size");
            process::exit(1);
        });

    let mut_pb = value_t!(matches, "mutate-pb", f32).unwrap_or_else(|_| {
        eprintln!("Failed to parse argument: mutate-pb");
        process::exit(1);
    });

    let cx_pb = value_t!(matches, "crossover-pb", f32).unwrap_or_else(|_| {
        eprintln!("Failed to parse argument: crossover-pb");
        process::exit(1);
    });

    let flip_pb = value_t!(matches, "flip-pb", f32).unwrap_or_else(|_| {
        eprintln!("Failed to parse argument: flip-pb");
        process::exit(1);
    });

    let rng = thread_rng();

    let init_fn = {
        move || vec![false; n_bits]
    };

    let mut ea = Simple::new(
        // operators
        Tournament::with_size(tourn_size), // Select
        TwoPoint::default(), // Crossover
        FlipBit::with_pb(flip_pb), // Mutation
        fitness_fn, // Fitness
        mut_pb,
        cx_pb,
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
    println!("Max Fitness: {}", n_bits);
}
