//! Mutation genetic operators and traits

use bit_vec::BitVec;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum MutateOperator {
    BitFlip { indv_pb: f64 },
}

impl MutateOperator {
    pub fn mutate<R: Rng>(&self, genome: BitVec, rng: &mut R) -> BitVec {
        match *self {
            MutateOperator::BitFlip { indv_pb } => genome
                .into_iter()
                .map(|bit| if rng.gen_bool(indv_pb) { !bit } else { bit })
                .collect(),
        }
    }
}
