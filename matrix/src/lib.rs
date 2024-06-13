use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
    Rng,
};
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T: Clone> {
    pub rows: usize,
    pub cols: usize,
    pub buf: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(fill: T, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            buf: vec![fill; rows * cols],
        }
    }

    pub fn random(rows: usize, cols: usize, range: Range<T>) -> Self
    where
        Standard: Distribution<T>,
        T: SampleUniform,
    {
        let distribution = Uniform::try_from(range).unwrap();
        let buf = rand::thread_rng()
            .sample_iter(distribution)
            .take(rows * cols)
            .collect();
        Self { rows, cols, buf }
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

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T: Clone> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index * self.cols..(index + 1) * self.cols]
    }
}
