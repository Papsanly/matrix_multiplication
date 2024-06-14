use matrix::Matrix;
use num_traits::NumAssign;
use std::{sync::Arc, thread};

pub fn multiply<T: NumAssign + Clone + Copy + Send + Sync + 'static>(
    left: Matrix<T>,
    right: Matrix<T>,
) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    let mut res = Matrix::new(T::zero(), left.rows, right.cols);
    let left = Arc::new(left);
    let right = Arc::new(right);

    let mut handles = Vec::with_capacity(left.rows);
    for i in 0..left.rows {
        let left = Arc::clone(&left);
        let right = Arc::clone(&right);
        handles.push(thread::spawn(move || {
            let mut row_res = vec![T::zero(); right.cols];
            for (j, val) in row_res.iter_mut().enumerate() {
                let mut sum = T::zero();
                for k in 0..left.cols {
                    sum += left[i][k] * right[k][j];
                }
                *val = sum;
            }
            row_res
        }));
    }

    for (i, handle) in handles.into_iter().enumerate() {
        for (j, val) in handle.join().unwrap().iter().enumerate() {
            res[i][j] = *val;
        }
    }

    res
}
