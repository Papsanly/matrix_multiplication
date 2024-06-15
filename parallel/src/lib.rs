use matrix::Matrix;
use num_traits::NumAssign;
use std::{sync::mpsc, thread};

pub fn multiply<T: NumAssign + Clone + Copy + Send + Sync>(
    left: &Matrix<T>,
    right: &Matrix<T>,
) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    let mut res = Matrix::new(T::zero(), left.rows, right.cols);

    let channels = (0..left.rows).map(|_| mpsc::channel()).collect::<Vec<_>>();

    thread::scope(|s| {
        for (i, (tx, _)) in channels.iter().enumerate() {
            s.spawn(move || {
                let mut row_res = vec![T::zero(); right.cols];
                for (j, val) in row_res.iter_mut().enumerate() {
                    let mut sum = T::zero();
                    for k in 0..left.cols {
                        sum += left[i][k] * right[k][j];
                    }
                    *val = sum;
                }
                tx.send(row_res).unwrap();
            });
        }
    });

    for (i, (_, rx)) in channels.into_iter().enumerate() {
        for (j, val) in rx.recv().unwrap().iter().enumerate() {
            res[i][j] = *val;
        }
    }

    res
}
