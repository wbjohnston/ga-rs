//! Genetic algorithm runners and supporting data structures

use std::collections::HashSet;
use select::SelectOp;
use crossover::CrossoverOp;
use mutate::MutateOp;
use num_traits::Zero;

use rand::Rng;

/// A Simple Genetic algorithm runner
#[derive(Debug, Clone)]
pub struct SimpleGARunner<S, M, C, E, G, O, R>
where
    S: SelectOp<G, O>,
    M: MutateOp<G>,
    C: CrossoverOp<G>,
    E: Fn(&G) -> O,
    G: Clone,
    O: Ord + Clone,
    R: Rng,
{
    select: S,
    mutate: M,
    crossover: C,
    evaluate: E,
    cx_pb: u32,
    mut_pb: u32, // 1 out n chance
    rng: R,
    state: State<G, O>,
}

impl<S, M, C, E, G, O, R> SimpleGARunner<S, M, C, E, G, O, R>
where
    S: SelectOp<G, O>,
    M: MutateOp<G>,
    C: CrossoverOp<G>,
    E: Fn(&G) -> O,
    G: Clone,
    O: Ord + Clone + Zero,
    R: Rng,
{
    /// Create a new runner and intialize the initial population using
    /// a function
    pub fn initialize_with_fn<F>(
        select: S,
        mutate: M,
        crossover: C,
        evaluate: E,
        cx_pb: f32,
        mut_pb: f32,
        rng: R,
        size: usize,
        f: F,
    ) -> Self
    where
        F: Fn() -> G,
    {
        // TODO(will): add way to specify terminating condition
        Self {
            select,
            crossover,
            mutate,
            evaluate,
            cx_pb: (1.0 / cx_pb) as u32,
            mut_pb: (1.0 / mut_pb) as u32,
            rng,
            state: State {
                invalid: HashSet::new(),
                generation: 0,
                population: (0..size).map(|_| (f(), O::zero())).collect(),
            },
        }
    }

    /// Return the fitnesses of all individuals
    pub fn fitnesses(&self) -> Vec<O> {
        self.state
            .population
            .iter()
            .cloned()
            .map(|(_, f)| f)
            .collect()
    }

    /// Return the current population
    pub fn population(&self) -> Vec<G> {
        self.state
            .population
            .iter()
            .cloned()
            .map(|(indv, _)| indv)
            .collect()
    }

    /// Returns a reference to the runnner state
    pub fn state(&self) -> &State<G, O>
    {
        &self.state
    }

    /// Return the size of the population
    pub fn size(&self) -> usize {
        self.state.population.len()
    }

    /// Return the current generation number
    pub fn generation(&self) -> usize {
        self.state.generation
    }

    /// Advance the algorithm state one generation
    pub fn advance(&mut self) {
        self.validate();

        // select step
        self.state.population = {
            let pop = self.state.population.as_slice();
            self.select.select(pop, pop.len(), &mut self.rng)
        };

        // mutate step
        {
            // scope so that iterator doesn't live for too long
            let iter = self.state.population.iter_mut().enumerate();
            for (i, &mut (ref mut indv, _)) in iter {
                if self.rng.gen_weighted_bool(self.mut_pb) {
                    *indv = self.mutate.mutate(indv, &mut self.rng);
                    self.state.invalid.insert(i);
                }
            }
        }

        // crossover step
        let mut other_idx: Vec<usize> = (0..self.size()).collect();
        self.rng.shuffle(&mut other_idx);
        let pair_iter = (0..self.size()).zip(other_idx);

        for (idx1, idx2) in pair_iter {
            if self.rng.gen_weighted_bool(self.cx_pb) {
                // generate children
                let (c1, c2) = {
                    let p1 = &self.state.population[idx1].0;
                    let p2 = &self.state.population[idx2].0;

                    self.crossover.crossover(p1, p2, &mut self.rng)
                };

                // update children
                self.state.population[idx1].0 = c1;
                self.state.population[idx2].0 = c2;

                // invalidate the children
                self.state.invalid.insert(idx1);
                self.state.invalid.insert(idx2);
            }
        }

        self.state.generation += 1;
    }

    /// Validate all individuals in the population and return the number of individuals that did 
    /// not have a valid fitness
    fn validate(&mut self) -> usize {
        // we're _going_ to evaluate all of the genomes that have an index in
        // the invalid hashset
        let validated = self.state.invalid.len();

        for i in &self.state.invalid {
            let &mut (ref mut g, ref mut fit) = &mut self.state.population[*i];
            *fit = (self.evaluate)(g);
        }

        self.state.invalid.clear();
        validated
    }
}

/// An evoluationary algorithm state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State<G: Clone, O: Ord + Clone> {
    invalid: HashSet<usize>,
    generation: usize,
    population: Vec<(G, O)>,
}
