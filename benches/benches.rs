// Copyright 2015 The Noise-rs Developers.
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

//! An example of using simplectic noise

#![feature(test)]

extern crate noise;
extern crate test;

use noise::Seed;
use noise::{perlin2, perlin3, perlin4};
use noise::{open_simplex2, open_simplex3, normalize_simplex};
use noise::{cell2_range, cell3_range, cell4_range};
use noise::{cell2_range_inv, cell3_range_inv, cell4_range_inv};
use noise::{cell2_value, cell3_value, cell4_value};
use noise::{cell2_manhattan, cell3_manhattan, cell4_manhattan};
use noise::{cell2_manhattan_inv, cell3_manhattan_inv, cell4_manhattan_inv};
use noise::{cell2_manhattan_value, cell3_manhattan_value, cell4_manhattan_value};
use test::{Bencher, black_box};

#[bench]
fn bench_perlin2(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin2(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_perlin3(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin3(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_perlin4(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| perlin4(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_open_simplex2(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| open_simplex2(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_open_simplex3(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| open_simplex3(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_open_simplex2_normalized(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| normalize_simplex(open_simplex2(black_box(&seed), black_box(&[42.0f32, 37.0]))));
}

#[bench]
fn bench_open_simplex3_normalized(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| normalize_simplex(open_simplex3(black_box(&seed), black_box(&[42.0f32, 37.0, 26.0]))));
}

#[bench]
fn bench_cell2_range(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_range(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_range(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_range(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_range(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_range(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_cell2_range_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_range_inv(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_range_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_range_inv(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_range_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_range_inv(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_cell2_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_value(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_value(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_value(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_cell2_manhattan(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_manhattan(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_manhattan(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_manhattan(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_manhattan(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_manhattan(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_cell2_manhattan_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_manhattan_inv(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_manhattan_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_manhattan_inv(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_manhattan_inv(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_manhattan_inv(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_cell2_manhattan_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell2_manhattan_value(black_box(&seed), black_box(&[42.0f64, 37.0])));
}

#[bench]
fn bench_cell3_manhattan_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell3_manhattan_value(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0])));
}

#[bench]
fn bench_cell4_manhattan_value(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| cell4_manhattan_value(black_box(&seed), black_box(&[42.0f64, 37.0, 26.0, 128.0])));
}

#[bench]
fn bench_perlin2_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(perlin2(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_perlin3_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(perlin3(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_perlin4_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(perlin4(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_open_simplex2_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(open_simplex2(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_open_simplex3_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(open_simplex3(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_range_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_range(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_range_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_range(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_range_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_range(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_range_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_range_inv(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_range_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_range_inv(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_range_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_range_inv(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_value(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_value(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_value(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_manhattan_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_manhattan(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_manhattan_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_manhattan(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_manhattan_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_manhattan(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_manhattan_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_manhattan_inv(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_manhattan_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_manhattan_inv(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_manhattan_inv_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_manhattan_inv(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell2_manhattan_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell2_manhattan_value(black_box(&seed), &[x as f64, y as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell3_manhattan_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell3_manhattan_value(black_box(&seed), &[x as f64, y as f64, x as f64]));
            }
        }
    });
}

#[bench]
fn bench_cell4_manhattan_value_64x64(bencher: &mut Bencher) {
    let seed = Seed::new(0);
    bencher.iter(|| {
        for y in 0..64 {
            for x in 0..64 {
                black_box(cell4_manhattan_value(black_box(&seed), &[x as f64, y as f64, x as f64, y as f64]));
            }
        }
    });
}
