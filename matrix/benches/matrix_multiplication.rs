use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use matrix::Matrix;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_multiplication");
    group.measurement_time(Duration::from_secs(5));
    for size in (1..=10).map(|x| 100 * x) {
        group.bench_with_input(BenchmarkId::new("parallel", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 0..100));
            let right = black_box(Matrix::random(size, size, 0..100));
            b.iter(|| parallel::multiply(&left, &right));
        });
        group.bench_with_input(BenchmarkId::new("sequential", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 0..100));
            let right = black_box(Matrix::random(size, size, 0..100));
            b.iter(|| sequential::multiply(&left, &right));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
