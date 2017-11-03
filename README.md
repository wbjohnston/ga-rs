# Petri: Evolutionary Computing Toolkit
Tools for running evolutionary algorithms

Please read the [API documentation](https://wbjohnston.github.io/petri)

_Note_: This project is currently a work in progress. If somehow you stumble
upon this library: you **should not** use this library as its incomplete and the
API is **not stable**.

[![Travis Status](https://travis-ci.org/wbjohnston/petri.svg?branch=master)](https://travis-ci.org/wbjohnston/petri)
[![codecov](https://codecov.io/gh/wbjohnston/petri/branch/master/graph/badge.svg)](https://codecov.io/gh/wbjohnston/petri)
[![Crates.io badge](https://img.shields.io/crates/v/petri.svg)](https://crates.io/crates/petri)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Usage
```rust
// Import what algorithms we're going to use
use petri::algo::{Simple, EvolutionaryAlgorithm};
use petri::ops::select::Tournament;
use petri::ops::crossover::TwoPoint;
use petri::ops::mutate::FlipBit;

use rand::thread_rng;

// Create the fitness function to evaluate a genome with
fn fitness_fn(genome: &Vec<bool>) -> u32
{
    let mut ones = 0;
    for c in genome {
        if c {
            ones += 1;
        }
    }

    ones
}

// Create the runner
let mut gen_runner = Simple::new(
    Tournament::with_size(3),
    TwoPoint::default(),
    FlipBit::with_pb(0.02),
    fitness_fn,
    0.01,
    0.05,
    300,
    thread_rng()
);

// Initialize population
gen_runner.initialize(300, || vec![false; 100]);

// run the algorithm
while !gen_runner.is_done() {
    let _ = gen_runner.next();
}

// View our final population
println!("{:?}", gen_runner.population());
```

## Contributing
Please see [CONTRIBUTING.md](/CONTRIBUTING.md)

## License
Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your option. This file may not be
copied, modified, or distributed except according to those terms.
