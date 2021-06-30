pub use self::{
    cache::*, combiners::*, generators::*, modifiers::*, selectors::*, transformers::*,
};

mod cache;
mod combiners;
mod generators;
mod modifiers;
mod selectors;
mod transformers;

/// Base trait for noise functions.
///
/// A noise function is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise function uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise function
///     in various ways.
/// * Combining the output values from two noise functions in various ways.
pub trait NoiseFn<T, const DIM: usize>: Send + Sync {
    fn get(&self, point: [T; DIM]) -> f64;
}

impl<'a, T, M: NoiseFn<T, DIM>, const DIM: usize> NoiseFn<T, DIM> for &'a M {
    #[inline]
    fn get(&self, point: [T; DIM]) -> f64 {
        M::get(*self, point)
    }
}

/// Trait for functions that require a seed before generating their values
pub trait Seedable {
    /// Set the seed for the function implementing the `Seedable` trait
    fn set_seed(self, seed: u32) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> u32;
}
