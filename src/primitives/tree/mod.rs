//! Tree based genetic programming primitives

use rand::Rng;

use traits::{Chromosome, Genome};

/*******************************************************************************
* Notes:
*   * Possibly rewrite the `genome::genesis` and mutation function to take in a 
*       "strategy" field instead of having multiple types sharing the same 
*       fields
*   * Mutation and Genesis need to take in some sort of "contraint" struct,
*       otherwise there is literally NO way to control how these things are
*       mutated. How though...
*******************************************************************************/

struct BinaryTree<C>
where
    C: Chromosome
{
    value: C,
    left:  Option<Box<BinaryTree<C>>>,
    right: Option<Box<BinaryTree<C>>> // FAT POINTER ALERT
}

/// Binary tree with fixed size
pub struct FixedBinaryTree<C>(BinaryTree<C>)
where
    C: Chromosome;

impl<C> Genome for FixedBinaryTree<C>
where
    C: Chromosome
{
    fn genesis(rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn mutate(&self, rate: f32, rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn cross(&self, other: &Self, rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn fitness<O>(&self) -> O
    where
        O: Ord
    {
        unimplemented!();
    }
}

/// Binary tree with non-fixed size and non-fixed depth mutation strategy
pub struct VariableBinaryTree<C>(BinaryTree<C>)
where
    C: Chromosome;

impl<C> Genome for VariableBinaryTree<C>
where
    C: Chromosome
{
    fn genesis(rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn mutate(&self, rate: f32, rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn cross(&self, other: &Self, rng: &mut Rng) -> Self
    {
        unimplemented!();
    }

    fn fitness<O>(&self) -> O
    where
        O: Ord
    {
        unimplemented!();
    }
}

