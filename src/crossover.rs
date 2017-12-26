//! Crossover genetic operators and traits

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

/// A crossover genetic operator
pub trait CrossoverOp<G> {
    /// Return a two children of the given parent genomes
    /// # Panics
    /// panics if the two given genomes differ in length
    fn crossover<R: Rng>(&self, g1: &G, g2: &G, rng: &mut R) -> (G, G);
}

/// A one point crossover genetic operator
#[derive(Debug, Copy, Clone)]
pub struct OnePoint;

impl<C: Clone> CrossoverOp<Vec<C>> for OnePoint {
    fn crossover<R: Rng>(&self, g1: &Vec<C>, g2: &Vec<C>, rng: &mut R) -> (Vec<C>, Vec<C>) {
        assert_eq!(g1.len(), g2.len());
        let size = g1.len();

        let cx_point = Range::new(0, size).ind_sample(rng);

        let mut c1 = g1.clone();
        let mut c2 = g2.clone();

        for i in cx_point..size {
            c1[i] = g2[i].clone();
            c2[i] = g1[i].clone();
        }

        (c1, c2)
    }
}

/// A two point crossover genetic operator
#[derive(Debug, Copy, Clone)]
pub struct TwoPoint;

impl<C: Clone> CrossoverOp<Vec<C>> for TwoPoint {
    fn crossover<R: Rng>(&self, g1: &Vec<C>, g2: &Vec<C>, rng: &mut R) -> (Vec<C>, Vec<C>) {
        assert_eq!(g1.len(), g2.len());
        let size = g1.len();
        let range = Range::new(0, size);

        // get crossover points
        let (p1, p2) = (range.ind_sample(rng), range.ind_sample(rng));
        let (cx1, cx2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

        let mut c1 = g1.clone();
        let mut c2 = g2.clone();

        for i in cx1..cx2 {
            c1[i] = g2[i].clone();
            c2[i] = g1[i].clone();
        }

        (c1, c2)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::{XorShiftRng, SeedableRng};
    use rand::distributions::{Range, IndependentSample};

    /// Verify that `OnePoint::crossover` panics when given genomes of different lengths
    #[test]
    #[should_panic]
    fn one_point_crossover_fails_with_different_lens() {
        let genome_a: Vec<u32> = (0..10).collect();
        let genome_b = (0..11).collect();
        let mut rng = XorShiftRng::new_unseeded(); // doesn't matter if its seeded

        OnePoint.crossover(&genome_a, &genome_b, &mut rng);
    }

    /// Verify that `OnePoint::crossover` crosses the genomes in one place.
    #[test]
    fn one_point_crossover() {
        let genome_a: Vec<u32> = (0..5).collect();
        let mut genome_b: Vec<u32> = genome_a.clone();
        genome_b.reverse();
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);

        // sample the rng for the crossover point, then reseed
        let cx = Range::new(0, genome_a.len()).ind_sample(&mut rng);
        rng.reseed([1, 2, 3, 4]);

        let (c1, c2) = OnePoint.crossover(&genome_a, &genome_b, &mut rng);

        // debug print
        println!("\nParent 1: {:?}\nParent 2: {:?}", genome_a, genome_b);
        println!("===============================================================================");
        println!("Child 1:  {:?}\nChild 2:  {:?}", c1, c2);
        println!("Crossover point: {}", cx);

        // verify
        for i in 0..genome_a.len() {
            println!("current {}", i);
            if i < cx {
                // original
                assert_eq!(c1[i], genome_a[i]);
                assert_eq!(c2[i], genome_b[i]);
            } else {
                // crossover
                assert_eq!(c1[i], genome_b[i]);
                assert_eq!(c2[i], genome_a[i]);
            }
        }
    }

    /// Verify that `TwoPoint::crossover` panics when given genomes of different lengths
    #[test]
    #[should_panic]
    fn two_point_crossover_fails_with_different_lens() {
        let genome_a: Vec<u32> = (0..10).collect();
        let genome_b = (0..11).collect();
        let mut rng = XorShiftRng::new_unseeded(); // doesn't matter if its seeded

        TwoPoint.crossover(&genome_a, &genome_b, &mut rng);
    }

    /// Verify that `TwoPoint::crossover` crosses the given genomes in two places
    #[test]
    fn two_point_crossover() {
        let genome_a: Vec<u32> = (0..100).collect();
        let mut genome_b: Vec<u32> = genome_a.clone();
        genome_b.reverse();

        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let range = Range::new(0, genome_a.len());

        // sample the rng for the crossover point and then reseed
        let (p1, p2) = (range.ind_sample(&mut rng), range.ind_sample(&mut rng));
        let (cx_a, cx_b) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
        rng.reseed([1, 2, 3, 4]);

        let (c1, c2) = TwoPoint.crossover(&genome_a, &genome_b, &mut rng);

        // debug print
        println!("\nParent 1: {:?}\nParent 2: {:?}", genome_a, genome_b);
        println!("===============================================================================");
        println!("Child 1: {:?}\nChild 2: {:?}", c1, c2);
        println!("Crossover points: ({}, {})", cx_a, cx_b);

        // verify
        for i in 0..genome_a.len() {
            if i >= cx_a && i < cx_b {
                // crossover
                assert_eq!(c2[i], genome_a[i]);
                assert_eq!(c1[i], genome_b[i]);
            } else {
                // original
                assert_eq!(c1[i], genome_a[i]);
                assert_eq!(c2[i], genome_b[i]);
            }
        }
    }
}
