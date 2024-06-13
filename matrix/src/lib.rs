mod tests;

use derive_more::{Deref, DerefMut};
use num_traits::Num;
use std::ops::{Index, Mul};

#[derive(Deref, DerefMut)]
pub struct Matrix<T: Num + Clone + Copy> {
    rows: usize,
    cols: usize,
    #[deref]
    #[deref_mut]
    buf: Vec<T>,
}

impl<T: Num + Clone + Copy> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            buf: vec![T::zero(); rows * cols],
        }
    }
}

impl<T: Num + Clone + Copy> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index * self.cols..(index + 1) * self.cols]
    }
}
