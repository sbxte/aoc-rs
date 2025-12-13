use std::ops::{Add, Div, Mul, Sub};

use crate::linalg::matrix::Matrix;
use crate::linalg::matrix::row_operations::{div, mul_sub, swap};
use crate::num::Zero;

#[derive(Debug, PartialEq, Eq)]
pub enum GaussElimResult<N> {
    NoSolution,
    OneSolution(Matrix<N>),
    Multisolution(Matrix<N>, Matrix<N>),
}

/// Perform gaussian elimination using row echelon forms
/// If infinite solutions are found, it will be collapsed into one solution and returned
/// `Rhs` is a one column matrix
pub fn gauss_elim<N>(mut matrix: Matrix<N>, mut rhs: Matrix<N>) -> GaussElimResult<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Zero
        + PartialEq
        + Copy
        + core::fmt::Debug,
{
    assert_eq!(matrix.rows(), rhs.rows());
    assert_eq!(rhs.cols(), 1);

    // Forward elimination
    let mut curr_row = 0;
    let mut col_range_end = 0; // inclusive
    for curr_col in 0..matrix.cols() {
        if curr_row >= matrix.rows() {
            break;
        }
        col_range_end = curr_col;

        let non_zero_row = (curr_row..matrix.rows()).find_map(|r| {
            if matrix.get(r, curr_col) == &N::zero() {
                None
            } else {
                Some(r)
            }
        });
        if let Some(row) = non_zero_row {
            swap(&mut matrix, row, curr_row);
            swap(&mut rhs, row, curr_row);

            for row in (curr_row + 1)..matrix.rows() {
                let multiplier = *matrix.get(row, curr_col) / *matrix.get(curr_row, curr_col);
                mul_sub(&mut matrix, curr_row, row, multiplier);
                mul_sub(&mut rhs, curr_row, row, multiplier);
            }
            curr_row += 1;
        } // Otherwise, entire column is already zeros
    }

    matrix.map(|x| if x == N::zero() { N::zero() } else { x });
    rhs.map(|x| if x == N::zero() { N::zero() } else { x });

    if col_range_end + 1 >= matrix.cols() {
        // there exists be a solution
        // if and only if all zero-rows have zeros on the RHS
        let all_zeros = (curr_row..matrix.rows())
            .map(|r| rhs.get(r, 0))
            .all(|c| c == &N::zero());
        if !all_zeros {
            return GaussElimResult::NoSolution;
        }
    }

    if curr_row == 0 {
        // The entire matrix is zero...
        let all_zeros = (0..matrix.rows())
            .map(|r| rhs.get(r, 0))
            .all(|c| c == &N::zero());
        if !all_zeros {
            return GaussElimResult::NoSolution;
        } else {
            return GaussElimResult::OneSolution(Matrix::new(N::zero(), 1, matrix.rows()));
        }
    }

    let row_range_end = curr_row - 1; // inclusive

    // Back substitution
    for row in (0..=row_range_end).rev() {
        let non_zero_col = (0..=col_range_end).find_map(|c| {
            if matrix.get(row, c) == &N::zero() {
                None
            } else {
                Some(c)
            }
        });

        if let Some(col) = non_zero_col {
            for r in 0..row {
                let multiplier = *matrix.get(r, col) / *matrix.get(row, col);
                mul_sub(&mut matrix, row, r, multiplier);
                mul_sub(&mut rhs, row, r, multiplier);
            }
            // Normalize
            let divisor = *matrix.get(row, col);
            div(&mut matrix, row, divisor);
            div(&mut rhs, row, divisor);
        } else {
            // already reached zero-row,
            // so stop
            break;
        }
    }
    matrix.map(|x| if x == N::zero() { N::zero() } else { x });
    rhs.map(|x| if x == N::zero() { N::zero() } else { x });

    // One solution
    if (0..matrix.cols()).all(|c| {
        (0..matrix.rows())
            .filter(|r| matrix.get(*r, c) == &N::zero())
            .count()
            + 1
            >= matrix.rows()
    }) {
        let mut m = Matrix::new(N::zero(), matrix.cols(), 1);
        for c in 0..matrix.cols() {
            if let Some(r) = (0..matrix.rows()).find(|r| matrix.get(*r, c) != &N::zero()) {
                *m.get_mut(c, 0) = *rhs.get(r, 0);
            } else {
                *m.get_mut(c, 0) = N::zero();
            }
        }
        return GaussElimResult::OneSolution(m);
    }
    GaussElimResult::Multisolution(matrix, rhs)
}

pub fn print_matrix<N: core::fmt::Debug>(matrix: &Matrix<N>) {
    for r in 0..matrix.rows() {
        for c in 0..matrix.cols() {
            print!("{:02?} ", matrix.get(r, c));
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ge_1() {
        let matrix = Matrix::from_slice(
            &[
                1., 0., 4., 2., 1., 2., 6., 2., 2., 0., 8., 8., 2., 1., 9., 4.,
            ],
            0.,
            4,
            4,
        );
        let rhs = Matrix::from_slice(&[0., 0., 0., 0.], 0., 4, 1);
        let result = gauss_elim(matrix, rhs);
        assert!(matches!(result, GaussElimResult::Multisolution(..)));
        if let GaussElimResult::Multisolution(mat, rhs) = result {
            // FIXME: finish test case
            panic!();
        }

        // Example taken from
        // https://en.wikipedia.org/wiki/Gaussian_elimination#Example_of_the_algorithm
        let matrix = Matrix::from_slice(&[2., 1., -1., -3., -1., 2., -2., 1., 2.], 0., 3, 3);
        let rhs = Matrix::from_slice(&[8., -11., -3.], 0., 3, 1);
        let result = gauss_elim(matrix, rhs);
        assert!(matches!(result, GaussElimResult::OneSolution(_)));
        if let GaussElimResult::OneSolution(m) = result {
            assert_eq!(*m.get(0, 0), 2.);
            assert_eq!(*m.get(0, 0), 3.);
            assert_eq!(*m.get(0, 0), -1.);
        }
    }
}
