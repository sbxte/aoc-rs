use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::num::{One, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Matrix<T> {
    inner_vec: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix initialized with `default_value`
    /// Usually good idea to set it to zero
    pub fn new(default_value: T, rows: usize, cols: usize) -> Self {
        Self {
            inner_vec: vec![default_value; rows * cols],
            rows,
            cols,
        }
    }

    /// Creates a matrix from a slice, resizing to the appropriate size,
    /// empty spots are initialized as `default_value`
    pub fn from_slice(data: &[T], default_value: T, rows: usize, cols: usize) -> Self {
        let mut data = data.to_vec();
        data.resize(rows * cols, default_value);
        Self {
            inner_vec: data,
            rows,
            cols,
        }
    }

    /// Creates a matrix from a slice, resizing to the appropriate size,
    /// empty spots are initialized as `default_value`
    pub fn from_vec(mut data: Vec<T>, default_value: T, rows: usize, cols: usize) -> Self {
        data.resize(rows * cols, default_value);
        Self {
            inner_vec: data,
            rows,
            cols,
        }
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut v = Vec::with_capacity(self.rows * rhs.cols);
        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut sum = T::zero();
                for i in 0..self.cols {
                    sum = sum + *self.get(r, i) * *rhs.get(i, c);
                }
                v.push(sum);
            }
        }
        Self {
            inner_vec: v,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl<T: Copy> Matrix<T> {
    pub fn map<F: Fn(T) -> T>(&mut self, f: F) {
        self.inner_vec.iter_mut().for_each(|x| *x = f(*x));
    }
}

impl<T> Matrix<T> {
    /// 0 <= row < N, 0 <= col < M
    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        &self.inner_vec[self.cols * row + col]
    }

    /// 0 <= row < N, 0 <= col < M
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        &mut self.inner_vec[self.cols * row + col]
    }
}

impl<T: Copy> From<SquareMatrix<T>> for Matrix<T> {
    fn from(value: SquareMatrix<T>) -> Self {
        Self {
            rows: value.size,
            cols: value.size,
            inner_vec: value.inner_matrix.inner_vec,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SquareMatrix<T> {
    inner_matrix: Matrix<T>,
    size: usize,
}

impl<T> SquareMatrix<T>
where
    T: Default + Copy,
{
    pub fn new(size: usize) -> Self {
        let mut m = Matrix::new(T::default(), size, size);
        for i in 0..size {
            *m.get_mut(i, i) = T::default();
        }
        Self::from(m)
    }
}
impl<T> SquareMatrix<T>
where
    T: One + Default + Copy,
{
    pub fn identity(size: usize) -> Self {
        let mut m = Matrix::new(T::default(), size, size);
        for i in 0..size {
            *m.get_mut(i, i) = T::one();
        }
        Self::from(m)
    }
}
impl<T> SquareMatrix<T>
where
    T: PartialEq + One + Copy + Zero,
{
    pub fn is_identity(&self) -> bool {
        let mut identity = true;
        for r in 0..self.size {
            for c in 0..self.size {
                if r == c {
                    identity &= self.get(r, c) == &T::one();
                } else {
                    identity &= self.get(r, c) == &T::zero();
                }
            }
        }
        identity
    }
}
impl<T: Copy> SquareMatrix<T> {
    /// Transposes the matrix in-place
    pub fn transpose(&mut self) {
        for r in 0..self.size {
            for c in r..self.size {
                // Swap the two
                let temp = *self.get(r, c);
                *self.get_mut(r, c) = *self.get(c, r);
                *self.get_mut(c, r) = temp;
            }
        }
    }
}

impl<T> SquareMatrix<T>
where
    T: Copy
        + Default
        + PartialEq
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + One
        + Zero
        + core::fmt::Debug,
{
    /// Inverts the matrix in-place
    pub fn get_inverted(&self) -> Option<Self> {
        // Use gaussian elimination
        let mut augment = Self::identity(self.size);
        let mut copy = self.clone();

        // Reduced row echelon
        for row in 0..copy.size {
            for n_row in (row + 1)..copy.size {
                let first_non_zero_index = if let Some((i, _x)) = copy.inner_vec
                    [row * copy.size..(row + 1) * copy.size]
                    .iter()
                    .enumerate()
                    .find(|(_i, x)| **x != T::zero())
                {
                    i
                } else {
                    break;
                };
                let multiplier =
                    *copy.get(n_row, first_non_zero_index) / *copy.get(row, first_non_zero_index);
                for c in 0..copy.size {
                    *copy.get_mut(n_row, c) = *copy.get(n_row, c) - *copy.get(row, c) * multiplier;
                    *augment.get_mut(n_row, c) =
                        *augment.get(n_row, c) - *augment.get(row, c) * multiplier;
                }
            }
        }

        // Identity matrix
        for row in (0..copy.size).rev() {
            for n_row in 0..row {
                let first_non_zero_index = if let Some((i, _x)) = copy.inner_vec
                    [row * copy.size..(row + 1) * copy.size]
                    .iter()
                    .enumerate()
                    .find(|(_i, x)| x != &&T::default())
                {
                    i
                } else {
                    break;
                };
                let multiplier =
                    *copy.get(n_row, first_non_zero_index) / *copy.get(row, first_non_zero_index);
                for c in 0..copy.size {
                    *copy.get_mut(n_row, c) = *copy.get(n_row, c) - *copy.get(row, c) * multiplier;
                    *augment.get_mut(n_row, c) =
                        *augment.get(n_row, c) - *augment.get(row, c) * multiplier;

                    *augment.get_mut(row, c) =
                        *augment.get(row, c) / *copy.get(row, first_non_zero_index);
                }
                *copy.get_mut(row, first_non_zero_index) = T::one();
            }
        }

        if copy.is_identity() {
            Some(augment)
        } else {
            None
        }
    }
}

impl<T> From<Matrix<T>> for SquareMatrix<T> {
    fn from(value: Matrix<T>) -> Self {
        Self {
            size: value.rows,
            inner_matrix: value,
        }
    }
}

impl<T> AsRef<Matrix<T>> for SquareMatrix<T> {
    fn as_ref(&self) -> &Matrix<T> {
        &self.inner_matrix
    }
}
impl<T> AsMut<Matrix<T>> for SquareMatrix<T> {
    fn as_mut(&mut self) -> &mut Matrix<T> {
        &mut self.inner_matrix
    }
}
impl<T> Deref for SquareMatrix<T> {
    type Target = Matrix<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_matrix
    }
}
impl<T> DerefMut for SquareMatrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_matrix
    }
}

impl<T> Mul<Matrix<T>> for SquareMatrix<T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self.inner_matrix * rhs
    }
}
impl<T> Mul<SquareMatrix<T>> for Matrix<T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: SquareMatrix<T>) -> Self::Output {
        self * rhs.inner_matrix
    }
}
impl<T> Mul<SquareMatrix<T>> for SquareMatrix<T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = SquareMatrix<T>;

    fn mul(self, rhs: SquareMatrix<T>) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        SquareMatrix::from(self.inner_matrix * rhs.inner_matrix)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqmat_transpose() {
        let mut matrix = SquareMatrix::from(Matrix::from_slice(&[1, 0, 0, 1], 0, 2, 2));
        matrix.transpose();
        assert_eq!(matrix.inner_matrix.inner_vec, [1, 0, 0, 1]);

        let mut matrix = SquareMatrix::from(Matrix::from_slice(&[1, 2, 3, 4], 0, 2, 2));
        matrix.transpose();
        assert_eq!(matrix.inner_matrix.inner_vec, [1, 3, 2, 4]);

        let mut matrix =
            SquareMatrix::from(Matrix::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 0, 3, 3));
        matrix.transpose();
        assert_eq!(matrix.inner_matrix.inner_vec, [1, 4, 7, 2, 5, 8, 3, 6, 9]);
    }

    #[test]
    fn sqmat_inversion() {
        let matrix = SquareMatrix::from(Matrix::from_slice(&[1., 2., 3., 4.], 0., 2, 2));
        let inversion = matrix.get_inverted();
        assert!(inversion.is_some());
        assert_eq!(inversion.unwrap().inner_vec, [-2., 1., 1.5, -0.5]);

        let matrix = SquareMatrix::from(Matrix::from_slice(&[1., 0., 0., 1.], 0., 2, 2));
        let inversion = matrix.get_inverted();
        assert!(inversion.is_some());
        assert_eq!(inversion.unwrap().inner_vec, [1., 0., 0., 1.]);
    }

    #[test]
    fn mat_mul() {
        let m1 = SquareMatrix::from(Matrix::from_slice(&[1, 0, 0, 1], 0, 2, 2));
        let m2 = SquareMatrix::from(Matrix::from_slice(&[1, 0, 0, 1], 0, 2, 2));
        let m3 = m1 * m2;
        assert_eq!(m3.inner_vec, [1, 0, 0, 1]);

        let m1 = SquareMatrix::from(Matrix::from_slice(&[1, 2, 3, 4], 0, 2, 2));
        let m2 = SquareMatrix::from(Matrix::from_slice(&[5, 6, 7, 8], 0, 2, 2));
        let m3 = m1 * m2;
        assert_eq!(m3.inner_vec, [19, 22, 43, 50]);
    }
}
