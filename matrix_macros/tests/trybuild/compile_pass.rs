use matrix_macros::*;

fn main() {
    let mat = matrix![[0, 1, 2], [3, 4, 5]];
    assert_eq!(mat.rows, 2);
    assert_eq!(mat.cols, 3);
    assert_eq!(*mat, vec![0, 1, 2, 3, 4, 5]);
}
