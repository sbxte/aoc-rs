use std::ops::{Index, IndexMut};

use crate::cartes::{dim2::vec::Vec2, grid::Grid};
/// Position types should be [Negateable][::std::ops::Neg]
pub type Pos = Vec2<isize>;

impl Pos {
    pub fn from_idx(idx: usize, cols: usize) -> Self {
        Vec2((idx % cols) as isize, (idx / cols) as isize)
    }

    pub fn to_idx(self, cols: usize) -> usize {
        (self.1 * cols as isize + self.0) as usize
    }
}
impl crate::cartes::pos::Pos for Pos {
    type N = isize;
    fn taxicab_dst(self, other: Self) -> Self::N {
        let x = other.0.abs_diff(self.0);
        let y = other.1.abs_diff(self.1);
        (x + y) as isize
    }

    fn euclid_dst_sq(self, other: Self) -> Self::N {
        let x = other.0 - self.0;
        let y = other.1 - self.1;
        x * x + y * y
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// 2-dimensional grid with bounds `(0,0)..(cols,rows)`
pub struct Grid2<C> {
    pub(crate) data: Vec<C>,
    pub cols: usize,
    pub rows: usize,
}
impl<C> Grid2<C> {
    pub fn from_raw(data: Vec<C>, cols: usize, rows: usize) -> Self {
        Self { data, cols, rows }
    }
}

impl<C> Grid for Grid2<C>
where
    C: Eq,
{
    type Pos = Pos;
    type Cell = C;

    fn contains_pos(&self, pos: Pos) -> bool {
        (0..self.cols as isize).contains(&pos.0) && (0..self.rows as isize).contains(&pos.1)
    }

    fn get_cell_unchecked(&self, pos: Self::Pos) -> &Self::Cell {
        &self.data[pos.to_idx(self.cols)]
    }

    fn get_cell_mut_unchecked(&mut self, pos: Self::Pos) -> &mut Self::Cell {
        &mut self.data[pos.to_idx(self.cols)]
    }

    fn get_neighbours(&self, pos: Self::Pos) -> impl Iterator<Item = &Self::Cell> {
        [
            self.get_cell(pos + From::from((1, 0))),
            self.get_cell(pos + From::from((-1, 0))),
            self.get_cell(pos + From::from((0, 1))),
            self.get_cell(pos + From::from((0, -1))),
        ]
        .into_iter()
        .flatten()
    }

    fn get_neighbours_pos(&self, pos: Self::Pos) -> impl Iterator<Item = Self::Pos> {
        [
            pos + From::from((1, 0)),
            pos + From::from((-1, 0)),
            pos + From::from((0, 1)),
            pos + From::from((0, -1)),
        ]
        .into_iter()
    }

    fn map<F, T>(self, f: F) -> impl Grid<Pos = Self::Pos, Cell = T>
    where
        F: Fn(Self::Cell) -> T,
        T: Eq,
    {
        let mut data: Vec<T> = Vec::with_capacity(self.data.len());
        for d in self.data {
            data.push(f(d));
        }
        Grid2 {
            data,
            cols: self.cols,
            rows: self.rows,
        }
    }
}
impl<C> Index<Pos> for Grid2<C>
where
    C: Eq,
    Self: Grid,
{
    type Output = C;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.data[index.to_idx(self.cols)]
    }
}
impl<C> IndexMut<Pos> for Grid2<C>
where
    C: Eq,
    Self: Grid,
{
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.data[index.to_idx(self.cols)]
    }
}

impl<C> Grid2<C>
where
    C: Eq,
    Self: Grid,
{
    /// Parses string input including newlines as a grid cell
    pub fn from_str_1<F>(s: &str, p: F) -> Self
    where
        F: Copy + Fn(u8) -> C,
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
        F: Copy + Fn(u8) -> Option<C>,
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

    pub fn new_fill_with<F>(f: F, cols: usize, rows: usize) -> Self
    where
        F: Fn() -> C,
    {
        let mut data = Vec::with_capacity(cols * rows);
        for _ in 0..cols * rows {
            data.push(f());
        }
        Self { data, cols, rows }
    }

    /// # Safety
    /// `idx` must be within bounds of the internal grid vector
    pub unsafe fn get_unchecked(&self, idx: usize) -> &C {
        unsafe { self.data.get_unchecked(idx) }
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

    pub fn as_slice(&self) -> &[C] {
        &self.data
    }

    pub fn iter(&self) -> Grid2Iterator<C> {
        Grid2Iterator {
            grid: self,
            selector: Pos::zero(),
        }
    }

    pub fn iter_mut(&mut self) -> Grid2MutIterator<C> {
        Grid2MutIterator {
            grid: self,
            selector: Pos::zero(),
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
    selector: Pos,
}

impl<'g, T> Iterator for Grid2Iterator<'g, T>
where
    T: Eq,
{
    type Item = (Pos, &'g T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.selector.1 + 1 >= self.grid.rows as isize
            && self.selector.0 >= self.grid.cols as isize
        {
            return None;
        }
        if self.selector.0 >= self.grid.cols as isize {
            self.selector.0 = 0;
            self.selector.1 += 1;
        }
        let ret = Some((self.selector, &self.grid[self.selector]));
        self.selector.0 += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper_bound =
            self.grid.rows * self.grid.cols - Pos::to_idx(self.selector, self.grid.cols);
        (upper_bound, Some(upper_bound))
    }
}

impl<'g, T> ExactSizeIterator for Grid2Iterator<'g, T> where T: Eq {}
impl<'g, T> ::core::iter::FusedIterator for Grid2Iterator<'g, T> where T: Eq {}

/// This is cursed
pub struct Grid2MutIterator<'g, T> {
    grid: &'g mut Grid2<T>,
    selector: Pos,
}

impl<'g, T> Iterator for Grid2MutIterator<'g, T>
where
    T: Eq,
{
    type Item = (Pos, &'g mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.selector.1 + 1 >= self.grid.rows as isize
            && self.selector.0 >= self.grid.cols as isize
        {
            return None;
        }
        if self.selector.0 >= self.grid.cols as isize {
            self.selector.0 = 0;
            self.selector.1 += 1;
        }
        let idx = Pos::to_idx(self.selector, self.grid.cols);
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
            self.grid.rows * self.grid.cols - Pos::to_idx(self.selector, self.grid.cols);
        (upper_bound, Some(upper_bound))
    }
}

impl<'g, T> ExactSizeIterator for Grid2MutIterator<'g, T> where T: Eq {}
impl<'g, T> ::core::iter::FusedIterator for Grid2MutIterator<'g, T> where T: Eq {}

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
