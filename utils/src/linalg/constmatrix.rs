use std::ops::{Deref, DerefMut, Div, Mul, Sub};

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
            inner_vec: value.0.inner_vec,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SquareMatrix<const N: usize, T>(Matrix<N, N, T>);

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
                let temp = *self.0.get(r, c);
                *self.0.get_mut(r, c) = *self.0.get(c, r);
                *self.0.get_mut(c, r) = temp;
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
        Self(value)
    }
}

impl<const N: usize, T> AsRef<Matrix<N, N, T>> for SquareMatrix<N, T> {
    fn as_ref(&self) -> &Matrix<N, N, T> {
        &self.0
    }
}
impl<const N: usize, T> AsMut<Matrix<N, N, T>> for SquareMatrix<N, T> {
    fn as_mut(&mut self) -> &mut Matrix<N, N, T> {
        &mut self.0
    }
}
impl<const N: usize, T> Deref for SquareMatrix<N, T> {
    type Target = Matrix<N, N, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize, T> DerefMut for SquareMatrix<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqmat_transpose() {
        let mut matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1, 0, 0, 1], 0));
        matrix.transpose();
        assert_eq!(matrix.0.inner_vec, [1, 0, 0, 1]);

        let mut matrix = SquareMatrix::from(Matrix::<2, 2, _>::from_slice(&[1, 2, 3, 4], 0));
        matrix.transpose();
        assert_eq!(matrix.0.inner_vec, [1, 3, 2, 4]);

        let mut matrix = SquareMatrix::from(Matrix::<3, 3, _>::from_slice(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            0,
        ));
        matrix.transpose();
        assert_eq!(matrix.0.inner_vec, [1, 4, 7, 2, 5, 8, 3, 6, 9]);
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
}
