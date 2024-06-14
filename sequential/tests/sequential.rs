use matrix_macros::matrix;
use sequential::*;

#[test]
fn test_multiply() {
    let left = matrix![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    let right = matrix![[13, 14, 15], [16, 17, 18], [19, 20, 21], [22, 23, 24]];
    let expected = matrix![[190, 200, 210], [470, 496, 522], [750, 792, 834]];
    assert_eq!(multiply(left, right), expected);
}

#[test]
fn test_multiply_different_sizes() {
    let left = matrix![[1, 2, 3], [4, 5, 6]];
    let right = matrix![[7, 8, 9, 10], [11, 12, 13, 14], [15, 16, 17, 18]];
    let expected = matrix![[74, 80, 86, 92], [173, 188, 203, 218]];
    assert_eq!(multiply(left, right), expected);
}

#[test]
#[should_panic(expected = "Invalid matrix dimensions")]
fn test_invalid_dimensions() {
    let left = matrix![[1, 2], [3, 4]];
    let right = matrix![[1, 2], [3, 4], [5, 6]];
    multiply(left, right);
}
