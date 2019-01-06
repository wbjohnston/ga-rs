//! Crossover genetic operators and traits

use bit_vec::BitVec;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum CrossoverOperator {
    SinglePoint,
}

impl CrossoverOperator {
    pub fn crossover<R>(&self, parent_a: BitVec, parent_b: BitVec, rng: &mut R) -> (BitVec, BitVec)
    where
        R: Rng,
    {
        match *self {
            CrossoverOperator::SinglePoint => {
                assert!(parent_a.len() == parent_b.len());
                let crossover_idx = rng.gen_range(0, parent_a.len());

                // NOTE: this is probably an implicit clone
                let child_a = parent_a.iter()
                    .take(crossover_idx)
                    .chain(parent_b.iter().skip(crossover_idx))
                    .collect();

                let child_b = parent_b.iter()
                    .take(crossover_idx)
                    .chain(parent_a.iter().skip(crossover_idx))
                    .collect();

                (child_a, child_b)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_point() {
        let parent_a = BitVec::from_elem(50, true);
        let parent_b = BitVec::from_elem(50, false);

        let mut rng = rand::thread_rng();
        let (c1, c2) = CrossoverOperator::SinglePoint.crossover(
            parent_a.clone(), // clone for later assertions
            parent_b.clone(),
            &mut rng
        );

        assert_ne!(c1, c2);
        assert_eq!(c1.len(), parent_a.len());
        assert_eq!(c2.len(), parent_b.len());
        // TODO: verify property
    }
}
