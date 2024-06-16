use matrix::Matrix;
use num_traits::NumAssign;
use std::{
    fmt::Debug,
    iter::{once, zip},
    sync::mpsc::{self, Sender},
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
    let (thread_txs, mut thread_rxs): (Vec<_>, Vec<_>) =
        (0..num_threads).map(|_| mpsc::channel()).unzip();
    thread_rxs.rotate_right(1);

    thread::scope(|s| {
        for (thread_idx, (thread_tx, thread_rx)) in zip(thread_txs, thread_rxs).enumerate() {
            let tx = Sender::clone(&tx);
            s.spawn(move || {
                let from_row = thread_idx * res.rows / num_threads;
                let to_row = (thread_idx + 1) * res.rows / num_threads;
                for col in once(thread_idx).chain(thread_rx).take(res.cols) {
                    for row in from_row..to_row {
                        let mut sum = T::zero();
                        for i in 0..left.cols {
                            sum += left[row][i] * right[i][col];
                        }
                        tx.send((row, col, sum)).unwrap();
                    }
                    thread_tx
                        .send(if thread_idx == num_threads - 1 {
                            (col + res.cols - num_threads) % res.cols
                        } else {
                            col
                        })
                        .unwrap_or(());
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
