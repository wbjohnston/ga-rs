
use traits::Genome;


pub fn init_repeat<G, F>(func: F, n: usize) -> Vec<G>
where
    G: Genome,
    F: Fn() -> G,
{
    (0..n).map(|_| func()).collect()
}
