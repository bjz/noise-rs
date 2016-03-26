// Copyright 2013 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{Brownian3, Seed};
//!
//! let seed = Seed::new(12);
//! let noise = Brownian3::new(noise::perlin3, 4).wavelength(32.0);
//! let val = noise.apply(&seed, &[42.0, 37.0, 2.0]);
//! ```

#![deny(missing_copy_implementations)]

extern crate num;
extern crate rand;

pub use seed::Seed;
pub use math::{Point2, Point3, Point4};
pub use perlin::{perlin2, perlin3, perlin4};
pub use open_simplex::{open_simplex2, open_simplex3, normalize_simplex};
pub use brownian::{Brownian2, Brownian3, Brownian4};

pub use cell::{range_sqr_euclidian2, range_sqr_euclidian3, range_sqr_euclidian4};
pub use cell::{cell2_seed_point, cell3_seed_point, cell4_seed_point};
pub use cell::{cell2_range, cell3_range, cell4_range};
pub use cell::{cell2_range_inv, cell3_range_inv, cell4_range_inv};
pub use cell::{cell2_value, cell3_value, cell4_value};
pub use cell::{cell2_manhattan, cell3_manhattan, cell4_manhattan};
pub use cell::{cell2_manhattan_inv, cell3_manhattan_inv, cell4_manhattan_inv};
pub use cell::{cell2_manhattan_value, cell3_manhattan_value, cell4_manhattan_value};

mod gradient;
mod math;
mod seed;

mod brownian;
mod perlin;
mod open_simplex;
mod cell;

/// A trait alias for a 2-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn2, Seed, Point2};
///
/// fn apply_noise2<F: GenFn2<f32>>(s: &Seed, p: &Point2<f32>, f: F) -> f32 { f(s, p) }
/// ```
pub trait GenFn2<T>: Fn(&Seed, &Point2<T>) -> T {}

/// A trait alias for a 3-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn3, Seed, Point3};
///
/// fn apply_noise3<F: GenFn3<f32>>(s: &Seed, p: &Point3<f32>, f: F) -> f32 { f(s, p) }
/// ```
pub trait GenFn3<T>: Fn(&Seed, &Point3<T>) -> T {}

/// A trait alias for a 4-dimensional noise function.
///
/// This is useful for succinctly parameterising over valid noise functions.
///
/// # Example
///
/// ```rust
/// use noise::{GenFn4, Seed, Point4};
///
/// fn apply_noise4<F: GenFn4<f32>>(s: &Seed, p: &Point4<f32>, f: F) -> f32 { f(s, p) }
/// ```
pub trait GenFn4<T>: Fn(&Seed, &Point4<T>) -> T {}

impl<T, F> GenFn2<T> for F where F: Fn(&Seed, &Point2<T>) -> T {}
impl<T, F> GenFn3<T> for F where F: Fn(&Seed, &Point3<T>) -> T {}
impl<T, F> GenFn4<T> for F where F: Fn(&Seed, &Point4<T>) -> T {}
