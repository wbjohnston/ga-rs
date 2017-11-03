
use std::marker::PhantomData;

use serde::Deserialize;

use genomes::Sequence;

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State<'a, G, O>
where
    G: Sequence<'a>,
    O: Ord + Clone
{
    n_gen: usize,
    max_gen: usize,
    population: Vec<(G, O)>,

    _marker: PhantomData<(&'a ())>
}
