use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use matrix::Matrix;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group_1 = c.benchmark_group("matrix_multiplication");
    for size in (1..5).map(|x| 200 * x) {
        group_1.bench_with_input(BenchmarkId::new("parallel", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 1..100));
            let right = black_box(Matrix::random(size, size, 1..100));
            b.iter(|| parallel::multiply(&left, &right, None));
        });
        group_1.bench_with_input(BenchmarkId::new("sequential", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 1..100));
            let right = black_box(Matrix::random(size, size, 1..100));
            b.iter(|| sequential::multiply(&left, &right));
        });
    }
    drop(group_1);
    let mut group_2 = c.benchmark_group("matrix_multiplication_parallel");
    for num_threads in (1..2).chain((1..6).map(|x| 4 * x)) {
        group_2.bench_with_input(
            BenchmarkId::new("parallel", num_threads),
            &num_threads,
            |b, &num_threads| {
                let size = 500;
                let left = black_box(Matrix::random(size, size, 1..100));
                let right = black_box(Matrix::random(size, size, 1..100));
                b.iter(|| parallel::multiply(&left, &right, Some(num_threads)));
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
