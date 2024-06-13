use derive_more::{Deref, DerefMut};
use num_traits::Num;
use std::ops::Index;

#[derive(Deref, DerefMut)]
pub struct Matrix<T: Num + Clone + Copy> {
    pub rows: usize,
    pub cols: usize,
    #[deref]
    #[deref_mut]
    buf: Vec<T>,
}

impl<T: Num + Clone + Copy> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            buf: vec![T::zero(); rows * cols],
        }
    }

    pub fn from_vec(cols: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len() % cols, 0);
        Self {
            rows: vec.len() / cols,
            cols,
            buf: vec,
        }
    }
}

impl<T: Num + Clone + Copy> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index * self.cols..(index + 1) * self.cols]
    }
}
