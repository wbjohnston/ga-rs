//! Operators for evaluating genomes

use genome::Genome;
use individual::Individual;

/// Operating for evaluating the fitness of an individual
pub trait EvaluteOperator<It, O>
where
    It: Individual,
    O: Ord,
{
    /// Evaluate the fitness of an individual
    fn evaluate(&self, g: &It) -> O;
}
