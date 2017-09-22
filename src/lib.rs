//! Genetic algorithm tools

mod organism;
pub use self::organism::Organism;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
