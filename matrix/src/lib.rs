use num_traits::Num;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T: Num + Clone> {
    pub rows: usize,
    pub cols: usize,
    buf: Vec<T>,
}

impl<T: Num + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            buf: vec![T::zero(); rows * cols],
        }
    }

    pub fn from_vec(cols: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len() % cols, 0, "Invalid number of elements");
        Self {
            rows: vec.len() / cols,
            cols,
            buf: vec,
        }
    }
}

impl<T: Num + Clone> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T: Num + Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index * self.cols..(index + 1) * self.cols]
    }
}
