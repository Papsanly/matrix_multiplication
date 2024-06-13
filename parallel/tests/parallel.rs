use matrix_macros::matrix;
use parallel::*;

#[test]
fn test_multiply() {
    let left = matrix![[1, 2, 3], [4, 5, 6]];
    let right = matrix![[7, 8], [9, 10], [11, 12]];
    let expected = matrix![[58, 64], [139, 154]];
    assert_eq!(multiply(left, right), expected);
}

#[test]
#[should_panic(expected = "Invalid matrix dimensions")]
fn test_invalid_dimensions() {
    let left = matrix![[1, 2], [3, 4]];
    let right = matrix![[1, 2], [3, 4], [5, 6]];
    multiply(left, right);
}
