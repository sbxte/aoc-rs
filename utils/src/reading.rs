pub mod grid {
    use crate::cartes::dim2::grid::Grid2;

    pub fn byte_with<F, T>(_input: &[u8], _f: F) -> Grid2<T>
    where
        F: Fn(u8) -> T,
    {
        todo!()
    }

    pub fn byte_and<F>(input: &[u8], f: F)
    where
        F: Fn(usize, u8),
    {
        input.iter().enumerate().for_each(|(i, c)| f(i, *c));
    }
}
