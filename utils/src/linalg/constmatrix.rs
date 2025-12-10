use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

use crate::num::{One, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Matrix<const N: usize, const M: usize, T> {
    inner_vec: Vec<T>,
}

impl<const N: usize, const M: usize, T: Copy> Matrix<N, M, T> {
    /// Creates a new matrix initialized with `default_value`
    /// Usually good idea to set it to zero
    pub fn new(default_value: T) -> Self {
        Self {
            inner_vec: vec![default_value; N * M],
        }
    }

    /// Creates a matrix from a slice, resizing to the appropriate size,
    /// empty spots are initialized as `default_value`
    pub fn from_slice(data: &[T], default_value: T) -> Self {
        let mut data = data.to_vec();
        data.resize(N * M, default_value);
        Self { inner_vec: data }
    }

    /// Creates a matrix from a slice, resizing to the appropriate size,
    /// empty spots are initialized as `default_value`
    pub fn from_vec(mut data: Vec<T>, default_value: T) -> Self {
        data.resize(N * M, default_value);
        Self { inner_vec: data }
    }
}

impl<const A: usize, const B: usize, const C: usize,T> Mul<Matrix<B, C, T>> for Matrix<A, B, T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<A, C, T>;

    fn mul(self, rhs: Matrix<B, C, T>) -> Self::Output {
        let mut v = Vec::with_capacity(A * C);
        for r in 0..A {
            for c in 0..B {
                let mut sum = T::zero();
                for i in 0..B {
                    sum = sum + *self.get(r, i) * *rhs.get(i, c);
                }
                v.push(sum);
            }
        }
        Matrix::<A, C, T> {
            inner_vec: v,
        }
    }
}

impl<const N: usize, const M: usize, T: Copy> Matrix<N, M, T> {
    pub fn map<F: Fn(T) -> T>(&mut self, f: F) {
        self.inner_vec.iter_mut().for_each(|x| *x = f(*x));
    }
}

impl<const N: usize, const M: usize, T> Matrix<N, M, T> {
    /// 0 <= row < N, 0 <= col < M
    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < N);
        assert!(col < M);
        &self.inner_vec[N * row + col]
    }

    /// 0 <= row < N, 0 <= col < M
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        assert!(row < N);
        assert!(col < M);
        &mut self.inner_vec[N * row + col]
    }
}

impl<const N: usize, T: Copy> From<SquareMatrix<N, T>> for Matrix<N, N, T> {
    fn from(value: SquareMatrix<N, T>) -> Self {
        Self {
            inner_vec: value.inner_matrix.inner_vec,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SquareMatrix<const N: usize, T> {
    inner_matrix: Matrix<N, N, T>,
}

impl<const N: usize, T> SquareMatrix<N, T>
where
    T: One + Default + Copy,
{
    pub fn identity() -> Self {
        let mut m = Matrix::new(T::default());
        for i in 0..N {
            *m.get_mut(i, i) = T::one();
        }
        Self::from(m)
    }
}
impl<const N: usize, T> SquareMatrix<N, T>
where
    T: PartialEq + One + Copy + Zero,
{
    pub fn is_identity(&self) -> bool {
        let mut identity = true;
        for r in 0..N {
            for c in 0..N {
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
impl<const N: usize, T: Copy> SquareMatrix<N, T> {
    /// Transposes the matrix in-place
    pub fn transpose(&mut self) {
        for r in 0..N {
            for c in r..N {
                // Swap the two
                let temp = *self.inner_matrix.get(r, c);
                *self.inner_matrix.get_mut(r, c) = *self.inner_matrix.get(c, r);
                *self.inner_matrix.get_mut(c, r) = temp;
            }
        }
    }
}

impl<const N: usize, T> SquareMatrix<N, T>
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
        let mut augment = Self::identity();
        let mut copy = self.clone();

        // Reduced row echelon
        for row in 0..N {
            for n_row in (row + 1)..N {
                let first_non_zero_index = if let Some((i, _x)) = copy.inner_vec
                    [row * N..(row + 1) * N]
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
                for c in 0..N {
                    *copy.get_mut(n_row, c) = *copy.get(n_row, c) - *copy.get(row, c) * multiplier;
                    *augment.get_mut(n_row, c) =
                        *augment.get(n_row, c) - *augment.get(row, c) * multiplier;
                }
            }
        }

        // Identity matrix
        for row in (0..N).rev() {
            for n_row in 0..row {
                let first_non_zero_index = if let Some((i, _x)) = copy.inner_vec
                    [row * N..(row + 1) * N]
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
                for c in 0..N {
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

impl<const N: usize, T> From<Matrix<N, N, T>> for SquareMatrix<N, T> {
    fn from(value: Matrix<N, N, T>) -> Self {
        Self {
            inner_matrix: value,
        }
    }
}

impl<const N: usize, T> AsRef<Matrix<N, N, T>> for SquareMatrix<N, T> {
    fn as_ref(&self) -> &Matrix<N, N, T> {
        &self.inner_matrix
    }
}
impl<const N: usize, T> AsMut<Matrix<N, N, T>> for SquareMatrix<N, T> {
    fn as_mut(&mut self) -> &mut Matrix<N, N, T> {
        &mut self.inner_matrix
    }
}
impl<const N: usize, T> Deref for SquareMatrix<N, T> {
    type Target = Matrix<N, N, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner_matrix
    }
}
impl<const N: usize, T> DerefMut for SquareMatrix<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_matrix
    }
}

impl<const N: usize, const M: usize, T> Mul<Matrix<M, N, T>> for SquareMatrix<M, T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<M, N, T>;

    fn mul(self, rhs: Matrix<M, N, T>) -> Self::Output {
        self.inner_matrix * rhs
    }
}
impl<const N: usize, const M: usize, T> Mul<SquareMatrix<N, T>> for Matrix<M, N, T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<M, N, T>;

    fn mul(self, rhs: SquareMatrix<N, T>) -> Self::Output {
        self * rhs.inner_matrix
    }
}
impl<const N: usize, T> Mul<SquareMatrix<N, T>> for SquareMatrix<N, T>
where
    T: Copy + Zero + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: SquareMatrix<N, T>) -> Self::Output {
        SquareMatrix::from(self.inner_matrix * rhs.inner_matrix)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqmat_transpose() {
        let mut matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1, 0, 0, 1], 0));
        matrix.transpose();
        assert_eq!(matrix.inner_vec, [1, 0, 0, 1]);

        let mut matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1, 2, 3, 4], 0));
        matrix.transpose();
        assert_eq!(matrix.inner_vec, [1, 3, 2, 4]);

        let mut matrix = SquareMatrix::from(Matrix::<3, 3, _>::from_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            0,
        ));
        matrix.transpose();
        assert_eq!(matrix.inner_vec, [1, 4, 7, 2, 5, 8, 3, 6, 9]);
    }

    #[test]
    fn sqmat_inversion() {
        let matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1., 2., 3., 4.], 0.));
        let inversion = matrix.get_inverted();
        assert!(inversion.is_some());
        assert_eq!(inversion.unwrap().inner_vec, [-2., 1., 1.5, -0.5]);

        let matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1., 0., 0., 1.], 0.));
        let inversion = matrix.get_inverted();
        assert!(inversion.is_some());
        assert_eq!(inversion.unwrap().inner_vec, [1., 0., 0., 1.]);
    }

    #[test]
    fn mat_mul() {
        let m1: SquareMatrix<2, _> = SquareMatrix::from(Matrix::from_slice(&[1, 0, 0, 1], 0));
        let m2: SquareMatrix<2, _> = SquareMatrix::from(Matrix::from_slice(&[1, 0, 0, 1], 0));
        let m3 = m1 * m2;
        assert_eq!(m3.inner_vec, [1, 0, 0, 1]);

        let m1: SquareMatrix<2, _> = SquareMatrix::from(Matrix::from_slice(&[1, 2, 3, 4], 0));
        let m2: SquareMatrix<2, _> = SquareMatrix::from(Matrix::from_slice(&[5, 6, 7, 8], 0));
        let m3 = m1 * m2;
        assert_eq!(m3.inner_vec, [19, 22, 43, 50]);
    }
}
