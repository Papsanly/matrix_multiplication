use matrix::Matrix;
use num_traits::NumAssign;
use std::{
    fmt::Debug,
    sync::{
        mpsc::{self, Sender},
        Arc, Barrier,
    },
    thread,
};

pub fn multiply<T: NumAssign + Clone + Copy + Send + Sync + Debug>(
    left: &Matrix<T>,
    right: &Matrix<T>,
    num_threads: Option<usize>,
) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    let mut res = Matrix::new(T::zero(), left.rows, right.cols);

    let (tx, rx) = mpsc::channel();
    let num_threads = num_threads
        .unwrap_or(
            thread::available_parallelism()
                .map(|v| v.get())
                .unwrap_or(1),
        )
        .min(res.rows);
    let barrier = Arc::new(Barrier::new(num_threads));

    thread::scope(|s| {
        for thread_idx in 0..num_threads {
            let tx = Sender::clone(&tx);
            let barrier = Arc::clone(&barrier);
            s.spawn(move || {
                let from_row = thread_idx * res.rows / num_threads;
                let to_row = (thread_idx + 1) * res.rows / num_threads;
                for col in (0..right.cols).map(|v| (v + thread_idx) % right.cols) {
                    for row in from_row..to_row {
                        let mut sum = T::zero();
                        for i in 0..left.cols {
                            sum += left[row][i] * right[i][col];
                        }
                        tx.send((row, col, sum)).unwrap();
                    }
                    barrier.wait();
                }
            });
        }
    });

    drop(tx);
    for (row, col, val) in rx {
        res[row][col] = val;
    }

    res
}
