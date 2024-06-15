use matrix::Matrix;
use num_traits::NumAssign;
use std::{fmt::Debug, sync::mpsc, thread};

pub fn multiply<T: NumAssign + Clone + Copy + Send + Sync + Debug>(
    left: &Matrix<T>,
    right: &Matrix<T>,
    num_threads: Option<usize>,
) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    let mut res = Matrix::new(T::zero(), left.rows, right.cols);

    let num_threads = num_threads
        .unwrap_or(
            thread::available_parallelism()
                .map(|v| v.get())
                .unwrap_or(1),
        )
        .min(res.rows);

    let iterations = res.cols;
    let (tx, rx) = mpsc::channel();

    let multiply_sub_matrix = |thread_idx| {
        let from_row = thread_idx * res.rows / num_threads;
        let to_row = (thread_idx + 1) * res.rows / num_threads;

        for row in from_row..to_row {
            for iteration in 0..iterations {
                let mut sum = T::zero();
                for col in 0..left.cols {
                    sum += left[row][col] * right[col][iteration];
                }
                tx.send((row, iteration, sum)).unwrap();
            }
        }
    };

    thread::scope(|s| {
        for thread_idx in 0..num_threads {
            s.spawn(move || multiply_sub_matrix(thread_idx));
        }
    });

    drop(tx);
    for (row, col, val) in rx {
        res[row][col] = val;
    }

    res
}
