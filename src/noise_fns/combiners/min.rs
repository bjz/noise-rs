use crate::noise_fns::NoiseFn;

/// Noise function that outputs the smaller of the two output values from two source
/// functions.
pub struct Min<T, U, const N: usize>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    /// Outputs a value.
    pub source1: T,

    /// Outputs a value.
    pub source2: U,
}

impl<T, U, const N: usize> Min<T, U, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    pub fn new(source1: T, source2: U) -> Self {
        Self { source1, source2 }
    }
}

impl<T, U, const N: usize> NoiseFn<N> for Min<T, U, N>
where
    T: NoiseFn<N>,
    U: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        (self.source1.get(point)).min(self.source2.get(point))
    }
}
