use crate::{
    math::{scale_shift, vectors::*},
    noise_fns::{MultiFractal, NoiseFn, Perlin, Seedable},
};
use alloc::vec::Vec;

/// Noise function that outputs ridged-multifractal noise.
///
/// This noise function, heavily based on the fBm-noise function, generates
/// ridged-multifractal noise. Ridged-multifractal noise is generated in much
/// the same way as fBm noise, except the output of each octave is modified by
/// an absolute-value function. Modifying the octave values in this way
/// produces ridge-like formations.
///
/// The values output from this function will usually range from -1.0 to 1.0 with
/// default values for the parameters, but there are no guarantees that all
/// output values will exist within this range. If the parameters are modified
/// from their defaults, then the output will need to be scaled to remain in
/// the [-1,1] range.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
#[derive(Clone, Debug)]
pub struct RidgedMulti {
    /// Total number of frequency octaves to generate the noise with.
    ///
    /// The number of octaves control the _amount of detail_ in the noise
    /// function. Adding more octaves increases the detail, with the drawback
    /// of increasing the calculation time.
    pub octaves: u8,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for
    /// each successive octave in the noise function.
    ///
    /// The frequency of each successive octave is equal to the product of the
    /// previous octave's frequency and the lacunarity value.
    ///
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For
    /// almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for
    /// each successive octave in the noise function.
    ///
    /// The amplitude of each successive octave is equal to the product of the
    /// previous octave's amplitude and the persistence value. Increasing the
    /// persistence produces "rougher" noise.
    pub persistence: f64,

    /// The attenuation to apply to the weight on each octave. This reduces
    /// the strength of each successive octave, making their respective
    /// ridges smaller. The default attenuation is 2.0, making each octave
    /// half the height of the previous.
    pub attenuation: f64,

    seed: u32,
    sources: Vec<Perlin>,
}

impl RidgedMulti {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_OCTAVE_COUNT: u8 = 6;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = core::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 1.0;
    pub const DEFAULT_ATTENUATION: f64 = 2.0;

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            attenuation: Self::DEFAULT_ATTENUATION,
            sources: super::build_sources(Self::DEFAULT_SEED, Self::DEFAULT_OCTAVE_COUNT),
        }
    }

    pub fn set_attenuation(self, attenuation: f64) -> Self {
        Self {
            attenuation,
            ..self
        }
    }
}

impl Default for RidgedMulti {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }
}

impl MultiFractal for RidgedMulti {
    fn set_octaves(self, octaves: u8) -> Self {
        if self.octaves == octaves {
            return self;
        }

        Self {
            octaves,
            sources: super::build_sources(self.seed, octaves),
            ..self
        }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            ..self
        }
    }
}

impl Seedable for RidgedMulti {
    fn set_seed(self, seed: u32) -> Self {
        if self.seed == seed {
            return self;
        }

        Self {
            seed,
            sources: super::build_sources(seed, self.octaves),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

/// 2-dimensional `RidgedMulti` noise
impl NoiseFn<f64, 2> for RidgedMulti {
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut point = Vector2::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves as usize {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}

/// 3-dimensional `RidgedMulti` noise
impl NoiseFn<f64, 3> for RidgedMulti {
    fn get(&self, point: [f64; 3]) -> f64 {
        let mut point = Vector3::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves as usize {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}

/// 4-dimensional `RidgedMulti` noise
impl NoiseFn<f64, 4> for RidgedMulti {
    fn get(&self, point: [f64; 4]) -> f64 {
        let mut point = Vector4::from(point);

        let mut result = 0.0;
        let mut weight = 1.0;

        point *= self.frequency;

        for x in 0..self.octaves as usize {
            // Get the value.
            let mut signal = self.sources[x].get(point.into_array());

            // Make the ridges.
            signal = signal.abs();
            signal = 1.0 - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal / self.attenuation;

            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);

            // Scale the amplitude appropriately for this frequency.
            signal *= self.persistence.powi(x as i32);

            // Add the signal to the result.
            result += signal;

            // Increase the frequency.
            point *= self.lacunarity;
        }

        // Scale and shift the result into the [-1,1] range
        let scale = 2.0 - 0.5_f64.powi(self.octaves as i32 - 1);
        scale_shift(result, 2.0 / scale)
    }
}
