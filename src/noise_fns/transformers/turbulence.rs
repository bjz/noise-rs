use crate::noise_fns::{Fbm, MultiFractal, NoiseFn, Seedable};

/// Noise function that randomly displaces the input value before returning the
/// output value from the source function.
///
/// _ is the pseudo-random displacement of the input value. The
/// get() method randomly displaces the coordinates of the input value before
/// retrieving the output value from the source function. To control the
/// turbulence, an application can modify its frequency, its power, and its
/// roughness.
#[derive(Clone, Debug)]
pub struct Turbulence<Source> {
    /// Source function that outputs a value.
    pub source: Source,

    frequency: f64,

    /// Controls the strength of the turbulence by affecting how much each
    /// point is moved.
    pub power: f64,

    roughness: u8,

    seed: u32,
    x_distort_function: Fbm,
    y_distort_function: Fbm,
    z_distort_function: Fbm,
    u_distort_function: Fbm,
}

impl<Source> Turbulence<Source> {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_POWER: f64 = 1.0;
    pub const DEFAULT_ROUGHNESS: u8 = 3;

    pub fn new(source: Source) -> Self {
        Self {
            source,
            seed: Self::DEFAULT_SEED,
            frequency: Self::DEFAULT_FREQUENCY,
            power: Self::DEFAULT_POWER,
            roughness: Self::DEFAULT_ROUGHNESS,
            x_distort_function: Fbm::default()
                .set_seed(Self::DEFAULT_SEED)
                .set_octaves(Self::DEFAULT_ROUGHNESS)
                .set_frequency(Self::DEFAULT_FREQUENCY),
            y_distort_function: Fbm::default()
                .set_seed(Self::DEFAULT_SEED + 1)
                .set_octaves(Self::DEFAULT_ROUGHNESS)
                .set_frequency(Self::DEFAULT_FREQUENCY),
            z_distort_function: Fbm::default()
                .set_seed(Self::DEFAULT_SEED + 2)
                .set_octaves(Self::DEFAULT_ROUGHNESS)
                .set_frequency(Self::DEFAULT_FREQUENCY),
            u_distort_function: Fbm::default()
                .set_seed(Self::DEFAULT_SEED + 3)
                .set_octaves(Self::DEFAULT_ROUGHNESS)
                .set_frequency(Self::DEFAULT_FREQUENCY),
        }
    }

    /// Affects the roughness of the turbulence. Higher values are rougher.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self {
            frequency,
            x_distort_function: self.x_distort_function.set_frequency(frequency),
            y_distort_function: self.y_distort_function.set_frequency(frequency),
            z_distort_function: self.z_distort_function.set_frequency(frequency),
            u_distort_function: self.u_distort_function.set_frequency(frequency),
            ..self
        }
    }

    pub fn set_power(self, power: f64) -> Self {
        Self { power, ..self }
    }

    /// Affects the roughness of the turbulence. Higher values are rougher.
    pub fn set_roughness(self, roughness: u8) -> Self {
        Self {
            roughness,
            x_distort_function: self.x_distort_function.set_octaves(roughness),
            y_distort_function: self.y_distort_function.set_octaves(roughness),
            z_distort_function: self.z_distort_function.set_octaves(roughness),
            u_distort_function: self.u_distort_function.set_octaves(roughness),
            ..self
        }
    }
}

impl<Source> Seedable for Turbulence<Source> {
    fn set_seed(self, seed: u32) -> Self {
        Self {
            seed,
            x_distort_function: self.x_distort_function.set_seed(seed),
            y_distort_function: self.y_distort_function.set_seed(seed + 1),
            z_distort_function: self.z_distort_function.set_seed(seed + 2),
            u_distort_function: self.u_distort_function.set_seed(seed + 3),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl<Source> NoiseFn<f64, 2> for Turbulence<Source>
where
    Source: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;

        let x_distort = point[0] + (self.x_distort_function.get([x0, y0]) * self.power);
        let y_distort = point[1] + (self.y_distort_function.get([x1, y1]) * self.power);

        self.source.get([x_distort, y_distort])
    }
}

impl<Source> NoiseFn<f64, 3> for Turbulence<Source>
where
    Source: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;
        let z0 = point[2] + 31337.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;
        let z1 = point[2] + 60943.0 / 65536.0;

        let x2 = point[0] + 53820.0 / 65536.0;
        let y2 = point[1] + 11213.0 / 65536.0;
        let z2 = point[2] + 44845.0 / 65536.0;

        let x_distort = point[0] + (self.x_distort_function.get([x0, y0, z0]) * self.power);
        let y_distort = point[1] + (self.y_distort_function.get([x1, y1, z1]) * self.power);
        let z_distort = point[2] + (self.z_distort_function.get([x2, y2, z2]) * self.power);

        self.source.get([x_distort, y_distort, z_distort])
    }
}

impl<Source> NoiseFn<f64, 4> for Turbulence<Source>
where
    Source: NoiseFn<f64, 4>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;
        let z0 = point[2] + 31337.0 / 65536.0;
        let u0 = point[3] + 57948.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;
        let z1 = point[2] + 60943.0 / 65536.0;
        let u1 = point[3] + 48513.0 / 65536.0;

        let x2 = point[0] + 53820.0 / 65536.0;
        let y2 = point[1] + 11213.0 / 65536.0;
        let z2 = point[2] + 44845.0 / 65536.0;
        let u2 = point[3] + 39357.0 / 65536.0;

        let x3 = point[0] + 18128.0 / 65536.0;
        let y3 = point[1] + 44845.0 / 65536.0;
        let z3 = point[2] + 12414.0 / 65536.0;
        let u3 = point[3] + 60943.0 / 65536.0;

        let x_distort = point[0] + (self.x_distort_function.get([x0, y0, z0, u0]) * self.power);
        let y_distort = point[1] + (self.y_distort_function.get([x1, y1, z1, u1]) * self.power);
        let z_distort = point[2] + (self.z_distort_function.get([x2, y2, z2, u2]) * self.power);
        let u_distort = point[3] + (self.u_distort_function.get([x3, y3, z3, u3]) * self.power);

        self.source
            .get([x_distort, y_distort, z_distort, u_distort])
    }
}
