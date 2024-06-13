#![cfg(test)]

use crate::*;
use matrix_macros::matrix;

fn get_matrix_1() -> Matrix<i32> {
    matrix![[0, 1, 2], [3, 4, 5]]
}

fn get_matrix_2() -> Matrix<i32> {
    matrix![[0, 1], [2, 3], [4, 5]]
}

#[test]
fn index() {
    let matrix = get_matrix_1();
    assert_eq!(matrix[0][0], 0);
    assert_eq!(matrix[0][1], 1);
    assert_eq!(matrix[1][1], 4);
    assert_eq!(matrix[1][2], 5);
}

#[test]
#[should_panic(expected = "out of range")]
fn index_out_of_range_1() {
    let matrix = get_matrix_1();
    let _ = matrix[2][1];
}

#[test]
#[should_panic(expected = "out of bounds")]
fn index_out_of_range_2() {
    let matrix = get_matrix_1();
    let _ = matrix[1][6];
}
