
use genome::Genome;

pub trait EvaluateOperator<G, C, O>
where
    G: Genome<C>,
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Calculate the fitness of a genome
    fn evaluate(&self, indv: &G) -> O;
}
