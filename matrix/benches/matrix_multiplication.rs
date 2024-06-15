use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use matrix::Matrix;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_multiplication");
    group.sample_size(10);
    for size in (1..10).map(|x| 100 * x) {
        group.bench_with_input(BenchmarkId::new("parallel", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 1..100));
            let right = black_box(Matrix::random(size, size, 1..100));
            b.iter(|| parallel::multiply(&left, &right, None));
        });
        group.bench_with_input(BenchmarkId::new("sequential", size), &size, |b, &size| {
            let left = black_box(Matrix::random(size, size, 1..100));
            let right = black_box(Matrix::random(size, size, 1..100));
            b.iter(|| sequential::multiply(&left, &right));
        });
    }
    // for num_threads in (1..6).map(|x| 4 * x) {
    //     group.bench_with_input(
    //         BenchmarkId::new("parallel_num_threads", num_threads),
    //         &num_threads,
    //         |b, &num_threads| {
    //             let size = 500;
    //             let left = black_box(Matrix::random(size, size, 1..100));
    //             let right = black_box(Matrix::random(size, size, 1..100));
    //             b.iter(|| parallel::multiply(&left, &right, Some(num_threads)));
    //         },
    //     );
    // }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
