use matrix::*;
use matrix_macros::matrix;

fn get_matrix() -> Matrix<i32> {
    matrix![[0, 1, 2], [3, 4, 5]]
}

#[test]
fn index() {
    let matrix = get_matrix();
    assert_eq!(matrix[0][0], 0);
    assert_eq!(matrix[0][1], 1);
    assert_eq!(matrix[1][1], 4);
    assert_eq!(matrix[1][2], 5);
}

#[test]
#[should_panic(expected = "out of range")]
fn index_out_of_range() {
    let matrix = get_matrix();
    let _ = matrix[2][1];
}

#[test]
#[should_panic(expected = "out of bounds")]
fn index_out_of_bounds() {
    let matrix = get_matrix();
    let _ = matrix[1][6];
}

#[test]
fn from_vec() {
    let matrix = Matrix::from_vec(3, vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(matrix[0][0], 0);
    assert_eq!(matrix[0][1], 1);
    assert_eq!(matrix[1][1], 4);
    assert_eq!(matrix[1][2], 5);
}

#[test]
#[should_panic(expected = "Invalid number of elements")]
fn from_vec_invalid() {
    let _ = Matrix::from_vec(3, vec![0, 1, 2, 3, 4]);
}

#[test]
fn random() {
    let matrix = Matrix::random(3, 3, 1..10);
    assert_eq!(matrix.rows, 3);
    assert_eq!(matrix.cols, 3);
    assert_eq!(matrix.buf.len(), 9);
    for val in matrix.buf {
        assert!((1..10).contains(&val));
    }
}
