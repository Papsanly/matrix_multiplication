use matrix::Matrix;
use num_traits::{Num, NumAssign};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn multiply<T: Num + NumAssign + Clone + Copy + Send + Sync + 'static>(
    left: Matrix<T>,
    right: Matrix<T>,
) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    let res = Arc::new(Mutex::new(Matrix::new(T::zero(), left.rows, right.cols)));
    let left = Arc::new(left);
    let right = Arc::new(right);

    let mut handles = Vec::new();
    for row in 0..left.rows {
        let res_clone = Arc::clone(&res);
        let left_clone = Arc::clone(&left);
        let right_clone = Arc::clone(&right);
        handles.push(thread::spawn(move || {
            for iteration in 0..right_clone.cols {
                let mut sum = T::zero();
                for col in 0..left_clone.cols {
                    sum += left_clone[row][col] * right_clone[col][iteration];
                }
                res_clone.lock().unwrap()[row][iteration] = sum;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::into_inner(res).unwrap().into_inner().unwrap()
}
