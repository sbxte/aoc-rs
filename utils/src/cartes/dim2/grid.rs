use std::ops::{Add, Div, Mul, Rem};

use crate::cartes::dim2::vec::Vec2;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Grid2<T> {
    data: Vec<T>,
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

    /// Parses string input using a filter map function
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
            data: s.bytes().filter_map(p).collect(),
            cols: s
                .lines()
                .next()
                .expect("Input string must have at least one line")
                .len()
                + 1,
            rows: s.lines().count(),
        }
    }

    /// # Safety
    /// `idx` must be within bounds of the internal grid vector
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        unsafe { self.data.get_unchecked(idx) }
    }

    pub fn get_v(&self, v: Vec2<usize>) -> &T {
        &self.data[v.1 * self.cols + v.0]
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

    pub fn into_iter(self) {}
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
