use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    elems: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Sized> Matrix<T> {
    pub fn new(rows: usize, cols: usize, elems: Vec<T>) -> Matrix<T> {
        assert_eq!(rows * cols, elems.len());
        Matrix { rows, cols, elems }
    }

    pub fn print(&self) where T: std::fmt::Display {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{} ", self.row(r)[c]);
            }
            println!("");
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn row(&self, row: usize) -> Row<T> {
        assert!(row < self.rows);
        Row { matrix: self, row }
    }

    pub fn row_mut(&mut self, row: usize) -> RowMut<T> {
        assert!(row < self.rows);
        RowMut { matrix: self, row }
    }

    pub fn col(&self, col: usize) -> Col<T> {
        assert!(col < self.cols);
        Col { matrix: self, col }
    }

    pub fn col_mut(&mut self, col: usize) -> ColMut<T> {
        assert!(col < self.cols);
        ColMut { matrix: self, col }
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        assert!(a < self.rows);
        assert!(b < self.rows);
        for c in 0..self.cols {
            self.elems.swap(c + a * self.cols, c + b * self.cols);
        }
    }

    pub fn multiply_row(&mut self, row: usize, f: &T) where for<'a> &'a T: Mul<&'a T, Output=T> {
        for c in 0..self.cols {
            self.row_mut(row)[c] = &self.row(row)[c] * f;
        }
    }

    pub fn add_row(&mut self, a: usize, b: usize) where for<'a> &'a T: Add<&'a T, Output=T> {
        for c in 0..self.cols {
            self.row_mut(a)[c] = &self.row(a)[c] + &self.row(b)[c];
        }
    }

    pub fn add_row_multiple(&mut self, a: usize, b: usize, f: &T) where for<'a> &'a T: Add<&'a T, Output=T> + Mul<&'a T, Output=T> {
        for c in 0..self.cols {
            self.row_mut(a)[c] = &self.row(a)[c] + &(&self.row(b)[c] * f);
        }
    }
}

impl<'b, T: std::iter::Sum<T>> Mul<&'b Matrix<T>> for &'b Matrix<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, other: &Matrix<T>) -> Self::Output {
        assert_eq!(self.rows, other.cols);
        let rows = other.rows;
        let cols = self.cols;

        let elems = (0..rows)
            .flat_map(|r| {
                (0..cols).map(move |c| {
                    (0..self.rows)
                        .map(|k| &self.row(k)[c] * &other.col(k)[r])
                        .sum()
                })
            })
            .collect::<Vec<T>>();
        Matrix { rows, cols, elems }
    }
}

impl<T: std::iter::Sum<T>> Mul<Matrix<T>> for Matrix<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        &self * &other
    }
}

impl<'b, T: std::iter::Sum<T>> Mul<&'b Matrix<T>> for Matrix<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, other: &Matrix<T>) -> Self::Output {
        &self * other
    }
}

impl<'b, T: std::iter::Sum<T>> Mul<Matrix<T>> for &'b Matrix<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        self * &other
    }
}

pub struct Row<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output {
        assert!(idx < self.matrix.cols);
        &self.matrix.elems[self.row * self.matrix.cols + idx]
    }
}

pub struct RowMut<'a, T> {
    matrix: &'a mut Matrix<T>,
    row: usize,
}

impl<'a, T> Index<usize> for RowMut<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output {
        assert!(idx < self.matrix.cols);
        &self.matrix.elems[self.row * self.matrix.cols + idx]
    }
}

impl<'a, T> IndexMut<usize> for RowMut<'a, T> {
    fn index_mut<'b>(&'b mut self, idx: usize) -> &'b mut Self::Output {
        assert!(idx < self.matrix.cols);
        &mut self.matrix.elems[self.row * self.matrix.cols + idx]
    }
}

pub struct Col<'a, T> {
    matrix: &'a Matrix<T>,
    col: usize,
}

impl<'a, T> Index<usize> for Col<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output {
        assert!(idx < self.matrix.cols);
        &self.matrix.elems[self.col + self.matrix.cols * idx]
    }
}

pub struct ColMut<'a, T> {
    matrix: &'a mut Matrix<T>,
    col: usize,
}

impl<'a, T> Index<usize> for ColMut<'a, T> {
    type Output = T;
    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output {
        assert!(idx < self.matrix.cols);
        &self.matrix.elems[self.col + self.matrix.cols * idx]
    }
}

impl<'a, T> IndexMut<usize> for ColMut<'a, T> {
    fn index_mut<'b>(&'b mut self, idx: usize) -> &'b mut Self::Output {
        assert!(idx < self.matrix.cols);
        &mut self.matrix.elems[self.col + self.matrix.cols * idx]
    }
}
