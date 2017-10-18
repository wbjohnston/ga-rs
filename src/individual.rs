
use genome::Genome;

/// An indiviudal containing a genome and fitness value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual<G: Genome, O: Ord + Clone> {
    genome: G,

    fitness: Option<O>,
}

impl<G: Genome, O: Ord + Clone> From<G> for Individual<G, O> {
    fn from(g: G) -> Self
    {
        Self {
            genome: g,
            fitness: None,
        }
    }
}

impl<G: Genome, O: Ord + Clone> Individual<G, O> {
    /// Invalidate individual's fitness
    pub fn invalidate(&mut self)
    {
        self.fitness = None
    }

    /// Get genome
    pub fn genome(&self) -> G
    {
        self.genome.clone()
    }

    /// Get fitness
    pub fn fitness(&self) -> Option<O>
    {
        self.fitness.clone()
    }

    /// Validate fitness with value
    pub fn validate(&mut self, val: O)
    {
        self.fitness = Some(val);
    }

    /// Is the individual's fitness valid
    pub fn is_valid(&self) -> bool
    {
        self.fitness.is_some()
    }
}
