//! Tree based genetic programming primitives

use rand::Rng;

use std::collections::{HashSet, VecDeque};

pub enum Primitive<T>
{
    Constant(T),

    EphemeralConstant(fn(&mut Rng) -> T),

    Operation {
        arity: usize,
        op: fn(Vec<T>) -> T,
    }
}

pub struct Tree<T>
{
    data: Primitive<T>,
    children: Vec<Box<Tree<T>>>
}
