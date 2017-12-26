//! Mutation genetic operators and traits

use rand::Rng;
use std::ops::Not;

/// A mutation genetic operator
pub trait MutateOp<G> {
    /// Return a mutated version of a genome
    fn mutate<R: Rng>(&self, g: &G, rng: &mut R) -> G;
}

/// A bit-flip mutation genetic operator
#[derive(Debug, Copy, Clone)]
pub struct BitFlip {
    indv_pb: u32,
}

impl BitFlip {
    /// Create a Bitflip operator with a specified per chromosome mutation probability
    pub fn with_pb(indv_pb: f32) -> Self {
        Self { indv_pb: (1.0 / indv_pb) as u32 }
    }
}
impl<C: Clone + Not<Output = C>> MutateOp<Vec<C>> for BitFlip {
    fn mutate<R: Rng>(&self, g: &Vec<C>, rng: &mut R) -> Vec<C> {
        g.iter()
            .cloned()
            .map(|c| if rng.gen_weighted_bool(self.indv_pb) {
                !c
            } else {
                c
            })
            .collect()
    }
}

#[cfg(test)]
mod test {}

#[cfg(all(feature="nightly", test))]
mod bench 
{
    use super::*;
    use test::Bencher;
    use rand::{XorShiftRng, SeedableRng};

    #[bench]
    fn bitflip_10_xorshift(b: &mut Bencher)
    {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let genome: Vec<u32> = (0..10).collect();
        b.iter(|| {
            let _ = BitFlip::with_pb(0.1).mutate(&genome, &mut rng); 
        });
    }

    #[bench]
    fn bitflip_100_xorshift(b: &mut Bencher)
    {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let genome: Vec<u32> = (0..100).collect();
        b.iter(|| {
            let _ = BitFlip::with_pb(0.1).mutate(&genome, &mut rng); 
        });
    }

    #[bench]
    fn bitflip_1000_xorshift(b: &mut Bencher)
    {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let genome: Vec<u32> = (0..1000).collect();
        b.iter(|| {
            let _ = BitFlip::with_pb(0.1).mutate(&genome, &mut rng); 
        });
    }
}
