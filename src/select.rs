//! Selection genetic operators and traits

use bit_vec::BitVec;
use rand::Rng;
use rand::seq::SliceRandom;

/// A selection operator for genetic algorithms
///
/// Given a population, will output the population that will be provided as input
/// for the next generation
#[derive(Debug, Clone, Copy)]
pub enum SelectOperator {
    Tournament { size: usize },
}

impl SelectOperator {
    pub fn select<R, O>(&self, population: Vec<(O, BitVec)>, rng: &mut R) -> Vec<(O, BitVec)>
    where
        R: Rng,
        O: Ord + Clone
    {
        match *self {
            SelectOperator::Tournament { size } => {
                // for each round take the best candidate in each round
                (0..population.len()).map(|_| population.choose_multiple(rng, size).cloned())
                .map(|round| round.max().unwrap()) // choose the best from the round
                .collect()
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tournament_selects_equal_candidates() {
        let population = vec![
            (1, BitVec::new()),
            (2, BitVec::new()),
            (3, BitVec::new()),
            (4, BitVec::new()),
            (5, BitVec::new()),
            (6, BitVec::new()),
            (7, BitVec::new()),
            (8, BitVec::new()),
            (9, BitVec::new()),
        ];

        let mut rng = rand::thread_rng();
        let op = SelectOperator::Tournament { size: 5 };
        let next_gen = op.select(population.clone(), &mut rng);

        assert_eq!(population.len(), next_gen.len());
        println!("BEFORE:\n{:?}\nAFTER:\n{:?}", population, next_gen);
    }
}
