extern crate criterion;
extern crate noise;

use criterion::*;
use noise::{NoiseFn, ReturnType, Worley};
use rand::Rng;

criterion_group!(worley_range, bench_worley2d, bench_worley3d, bench_worley4d,);
criterion_main!(worley_range,);

fn bench_worley2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Worley 2D");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let worley = Worley::default();
    let worley_range = Worley::default().set_return_type(ReturnType::Distance);

    let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());

    for step in 1..=6 {
        let size = 1 << step as u64;

        group.throughput(Throughput::Elements(size * size));

        let mut points: Vec<[f64; 2]> = Vec::new();

        for _ in 0..size * size {
            points.push([rng.gen(), rng.gen()])
        }

        group.bench_function(BenchmarkId::new("worley - value", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley.get(*point));
                }
            })
        });

        group.bench_function(BenchmarkId::new("worley - range", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley_range.get(*point));
                }
            })
        });
    }

    group.finish();
}

fn bench_worley3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Worley 3D");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let worley = Worley::default();
    let worley_range = Worley::default().set_return_type(ReturnType::Distance);

    let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());

    for step in 1..=6 {
        let size = 1 << step as u64;

        group.throughput(Throughput::Elements(size * size));

        let mut points: Vec<[f64; 3]> = Vec::new();

        for _ in 0..size * size {
            points.push([rng.gen(), rng.gen(), rng.gen()])
        }

        group.bench_function(BenchmarkId::new("worley - value", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley.get(*point));
                }
            })
        });

        group.bench_function(BenchmarkId::new("worley - range", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley_range.get(*point));
                }
            })
        });
    }

    group.finish();
}

fn bench_worley4d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Worley 4D");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    group.plot_config(plot_config);

    let worley = Worley::default();
    let worley_range = Worley::default().set_return_type(ReturnType::Distance);

    let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());

    for step in 1..=6 {
        let size = 1 << step as u64;

        group.throughput(Throughput::Elements(size * size));

        let mut points: Vec<[f64; 4]> = Vec::new();

        for _ in 0..size * size {
            points.push([rng.gen(), rng.gen(), rng.gen(), rng.gen()])
        }

        group.bench_function(BenchmarkId::new("worley - value", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley.get(*point));
                }
            })
        });

        group.bench_function(BenchmarkId::new("worley - range", size), |b| {
            b.iter(|| {
                for point in &points {
                    black_box(worley_range.get(*point));
                }
            })
        });
    }

    group.finish();
}
