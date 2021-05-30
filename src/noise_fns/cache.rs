use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};
use std::cell::{Cell, RefCell};

/// Noise function that caches the last output value generated by the source
/// function.
///
/// If the input coordinates passed to `Cache::get` are equal to the previous
/// call, the function returns the cached result of the previous call to
/// `Source::get`. Otherwise, `Source::get` is called with the new coordinates,
/// overwriting the cache with the result, and returning the result to the
/// caller.
///
/// Caching a noise function is useful if it is used as a source function for
/// multiple noise functions. If a source function is not cached, the source
/// function will redundantly calculate the same output value once for each
/// noise function in which it is included.
#[derive(Clone, Debug)]
pub struct Cache<Source> {
    /// Outputs the value to be cached.
    pub source: Source,

    value: Cell<Option<f64>>,

    point: RefCell<Vec<f64>>,
}

impl<Source> Cache<Source> {
    pub fn new(source: Source) -> Self {
        Cache {
            source,
            value: Cell::new(None),
            point: RefCell::new(Vec::new()),
        }
    }
}

impl<Source, const N: usize> NoiseFn<N> for Cache<Source>
where
    Source: NoiseFn<N>,
{
    fn get(&self, point: [f64; N]) -> f64 {
        match self.value.get() {
            Some(value) if quick_eq(&*self.point.borrow(), &point) => value,
            Some(_) | None => {
                let value = self.source.get(point);
                self.value.set(Some(value));

                let mut cached_point = self.point.borrow_mut();
                cached_point.clear();
                cached_point.extend_from_slice(&point);

                value
            }
        }
    }
}
impl<T> Seedable for Cache<T>
where
    T: Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
            value: Cell::new(None),
            point: RefCell::new(Vec::new()),
        }
    }

    fn set_seed(self, seed: u32) -> Self {
        Self {
            source: self.source.set_seed(seed),
            value: self.value,
            point: self.point,
        }
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}

impl<T> MultiFractal for Cache<T>
where
    T: MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self {
            source: self.source.set_octaves(octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self {
            source: self.source.set_frequency(frequency),
            ..self
        }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self {
            source: self.source.set_lacunarity(lacunarity),
            ..self
        }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            source: self.source.set_persistence(persistence),
            ..self
        }
    }
}

fn quick_eq(a: &[f64], b: &[f64]) -> bool {
    assert_eq!(a.len(), b.len());

    a.iter().eq(b)
}
