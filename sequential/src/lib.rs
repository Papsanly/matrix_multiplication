use matrix::Matrix;
use num_traits::{Num, NumAssign};

pub fn multiply<T: Num + NumAssign + Clone + Copy>(left: Matrix<T>, right: Matrix<T>) -> Matrix<T> {
    assert_eq!(left.cols, right.rows, "Invalid matrix dimensions");
    assert_eq!(left.rows, right.cols, "Invalid matrix dimensions");
    let mut res = Matrix::new(T::zero(), left.rows, right.cols);
    for i in 0..left.rows {
        for j in 0..right.cols {
            let mut sum = T::zero();
            for k in 0..left.cols {
                sum += left[i][k] * right[k][j];
            }
            res[i][j] = sum;
        }
    }
    res
}
