//! A collection of chromosomes

use petgraph::Graph;

pub trait Genome
{}

// Interface for randomly mutating, crossing over, and creating a genome used
// in genetic algorithms
// pub trait Genome: From<Graph> + Into<Graph>
// {
//     /// Get the fitness of the organism
//     fn fitness<O>(&self) -> O
//     where
//         O: Ord;
// }
