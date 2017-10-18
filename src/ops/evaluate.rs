
use individual::Individual;
use genome::Genome;

pub trait EvaluateOperator<G: Genome, O: Ord + Clone> {
    /// Calculate the fitness of a genome
    fn evaluate(&self, indv: &Individual<G, O>) -> O;
}
