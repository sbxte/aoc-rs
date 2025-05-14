use std::ops::{Add, Div, Mul, Rem};

use crate::cartes::dim2::vec::Vec2;

pub type Grid2Pos = Vec2<usize>;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Grid2<T> {
    pub(crate) data: Vec<T>,
    pub cols: usize,
    pub rows: usize,
}

impl<T> Grid2<T> {
    pub fn idx_to_vec2(idx: T, cols: T) -> Vec2<T>
    where
        T: Rem<Output = T> + Div<Output = T> + Copy,
    {
        Vec2(idx % cols, idx / cols)
    }

    pub fn vec2_to_idx(v: Vec2<T>, cols: T) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        v.1 * cols + v.0
    }

    /// Parses string input including newlines as a grid cell
    pub fn from_str_1<F>(s: &str, p: F) -> Self
    where
        F: Copy + Fn(u8) -> T,
    {
        if s.is_empty() {
            return Self {
                data: vec![],
                cols: 0,
                rows: 0,
            };
        }

        let mut data = s.bytes().map(p).collect::<Vec<_>>();
        data.push(p(b'\n'));
        Self {
            data,
            cols: s
                .lines()
                .next()
                .expect("input string must have at least one line")
                .len()
                + 1,
            rows: s.lines().count(),
        }
    }

    /// Parses string input ignoring whitespaces while using a filter map function
    ///
    /// If `p` returns `None`, the char is filtered out
    /// otherwise for `let Some(x) = p`, x is used
    pub fn from_str_2<F>(s: &str, p: F) -> Self
    where
        F: Copy + Fn(u8) -> Option<T>,
    {
        if s.is_empty() {
            return Self {
                data: vec![],
                cols: 0,
                rows: 0,
            };
        }

        Self {
            data: s
                .bytes()
                .filter(|c| !(*c as char).is_whitespace())
                .filter_map(p)
                .collect(),
            cols: s
                .lines()
                .next()
                .expect("Input string must have at least one line")
                .len(),
            rows: s.lines().count(),
        }
    }

    /// # Safety
    /// `idx` must be within bounds of the internal grid vector
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        unsafe { self.data.get_unchecked(idx) }
    }

    pub fn get_v(&self, v: Grid2Pos) -> &T {
        &self.data[Grid2::vec2_to_idx(v, self.cols)]
    }

    pub fn get_v_mut(&mut self, v: Grid2Pos) -> &mut T {
        &mut self.data[Grid2::vec2_to_idx(v, self.cols)]
    }

    /// # Safety
    /// `idx` must be within bounds of the internal grid vector
    pub unsafe fn swap_idx_unchecked(&mut self, src: usize, dst: usize) {
        unsafe {
            let s = self.data.as_mut_ptr().add(src);
            let d = self.data.as_mut_ptr().add(dst);
            std::ptr::swap(s, d);
        }
    }

    pub fn swap_idx(&mut self, src: usize, dst: usize) {
        self.data.swap(src, dst);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn iter(&self) -> Grid2Iterator<T> {
        Grid2Iterator {
            grid: self,
            selector: Grid2Pos::zero(),
        }
    }
}

impl<T> Grid2<T>
where
    T: std::fmt::Display,
{
    pub fn print_display(&self) {
        println!();
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}", self.data[r * self.cols + c]);
            }
            println!();
        }
    }
}

impl<T> AsRef<[T]> for Grid2<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for Grid2<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

pub struct Grid2Iterator<'g, T> {
    grid: &'g Grid2<T>,
    selector: Grid2Pos,
}

impl<'g, T> Iterator for Grid2Iterator<'g, T> {
    type Item = (Grid2Pos, &'g T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.selector.1 + 1 >= self.grid.rows && self.selector.0 >= self.grid.cols {
            return None;
        }
        if self.selector.0 >= self.grid.cols {
            self.selector.0 = 0;
            self.selector.1 += 1;
        }
        let ret = Some((self.selector, self.grid.get_v(self.selector)));
        self.selector.0 += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper_bound =
            self.grid.rows * self.grid.cols - Grid2::vec2_to_idx(self.selector, self.grid.cols);
        (upper_bound, Some(upper_bound))
    }
}

impl<'g, T> ExactSizeIterator for Grid2Iterator<'g, T> {}
impl<'g, T> ::core::iter::FusedIterator for Grid2Iterator<'g, T> {}

/// This is cursed
pub struct Grid2MutIterator<'g, T> {
    grid: &'g mut Grid2<T>,
    selector: Grid2Pos,
}

impl<'g, T> Iterator for Grid2MutIterator<'g, T> {
    type Item = (Grid2Pos, &'g mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.selector.1 + 1 >= self.grid.rows && self.selector.0 >= self.grid.cols {
            return None;
        }
        if self.selector.0 >= self.grid.cols {
            self.selector.0 = 0;
            self.selector.1 += 1;
        }
        let idx = Grid2::vec2_to_idx(self.selector, self.grid.cols);
        let v = if idx < self.grid.data.len() {
            unsafe { &mut *self.grid.data.as_mut_ptr().add(idx) }
        } else {
            return None;
        };
        let ret = Some((self.selector, v));
        self.selector.0 += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper_bound =
            self.grid.rows * self.grid.cols - Grid2::vec2_to_idx(self.selector, self.grid.cols);
        (upper_bound, Some(upper_bound))
    }
}

impl<'g, T> ExactSizeIterator for Grid2MutIterator<'g, T> {}
impl<'g, T> ::core::iter::FusedIterator for Grid2MutIterator<'g, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let grid = Grid2 {
            data: vec![1, 2, 3, 4],
            rows: 2,
            cols: 2,
        };

        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some((Vec2::from((0, 0)), &1)));
        assert_eq!(iter.next(), Some((Vec2::from((1, 0)), &2)));
        assert_eq!(iter.next(), Some((Vec2::from((0, 1)), &3)));
        assert_eq!(iter.next(), Some((Vec2::from((1, 1)), &4)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_size_hint() {
        let grid = Grid2 {
            data: vec![1, 2, 3, 4],
            rows: 2,
            cols: 2,
        };
        let mut iter = grid.iter();
        assert_eq!(iter.size_hint(), (4, Some(4)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
