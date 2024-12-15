pub mod matrix {
    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Matrix<const N: usize, const M: usize, T> {
        data: Vec<T>,
    }

    impl<const N: usize, const M: usize, T> Matrix<N, M, T> {}

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct SquareMatrix<const N: usize, T>(Matrix<N, N, T>);

    impl<const N: usize, T> From<Matrix<N, N, T>> for SquareMatrix<N, T> {
        fn from(value: Matrix<N, N, T>) -> Self {
            Self(value)
        }
    }

    impl<const N: usize, T> From<SquareMatrix<N, T>> for Matrix<N, N, T> {
        fn from(value: SquareMatrix<N, T>) -> Self {
            Self { data: value.0.data }
        }
    }
}
