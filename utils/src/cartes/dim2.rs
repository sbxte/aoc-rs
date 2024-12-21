use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::num::{One, RemEuclid, Zero};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    /// 90 degrees CCW
    pub fn rot90(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    /// 90 degrees CW
    pub fn rot270(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// 180 degrees
    pub fn rot180(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }

    pub fn step<N>(self) -> Vec2<N>
    where
        N: Neg<Output = N> + One + Zero,
    {
        use self::Direction::*;
        match self {
            Up => Vec2(N::zero(), -N::one()),
            Down => Vec2(N::zero(), N::one()),
            Right => Vec2(N::one(), N::zero()),
            Left => Vec2(-N::one(), N::zero()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Vec2<N>(pub N, pub N);

impl<N> Vec2<N>
where
    N: Copy + Ord + Add<Output = N> + Sub<Output = N> + Rem<Output = N> + RemEuclid,
{
    pub fn bounds_wrap(self, corner1: Self, corner2: Self) -> Self {
        Self(
            corner1.0 + ((self.0 - corner1.0).rem_euclid(corner2.0 - corner1.0)),
            corner1.1 + ((self.1 - corner1.1).rem_euclid(corner2.1 - corner1.1)),
        )
    }
}

impl<N> Vec2<N> {
    pub fn map<F, T>(self, f: F) -> Vec2<T>
    where
        F: Fn(N) -> T,
    {
        Vec2(f(self.0), f(self.1))
    }
}

impl<N> Vec2<N>
where
    N: Ord + Copy,
{
    pub fn get_min_max_corners(corner1: Self, corner2: Self) -> (Self, Self) {
        (
            Self(corner1.0.min(corner2.0), corner1.1.min(corner2.1)),
            Self(corner1.0.max(corner2.0), corner1.1.max(corner2.1)),
        )
    }
    pub fn bounds_clamp(self, corner1: Self, corner2: Self) -> Self {
        let (min, max) = Self::get_min_max_corners(corner1, corner2);
        Self(self.0.clamp(min.0, max.0), self.1.clamp(min.1, max.1))
    }

    pub fn is_oob_inclusive(self, corner1: Self, corner2: Self) -> bool {
        let (min, max) = Self::get_min_max_corners(corner1, corner2);
        !(min.0..=max.0).contains(&self.0) || !(min.1..=max.1).contains(&self.1)
    }
}

impl<A> Vec2<A> {
    pub fn from_v<B>(v: Vec2<B>) -> Self
    where
        A: From<B>,
    {
        Self(From::from(v.0), From::from(v.1))
    }

    pub fn into_v<B>(self) -> Vec2<B>
    where
        A: Into<B>,
    {
        Vec2(self.0.into(), self.1.into())
    }
}

impl<N> Add for Vec2<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<N> AddAssign for Vec2<N>
where
    N: Add<Output = N> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<N> Sub for Vec2<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<N> SubAssign for Vec2<N>
where
    N: Sub<Output = N> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<N> Mul<N> for Vec2<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl<N> Div<N> for Vec2<N>
where
    N: Div<Output = N> + Copy,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl<N> From<(N, N)> for Vec2<N> {
    fn from(value: (N, N)) -> Self {
        Self(value.0, value.1)
    }
}
impl<N> From<&(N, N)> for Vec2<N>
where
    N: Copy,
{
    fn from(value: &(N, N)) -> Self {
        Self(value.0, value.1)
    }
}
impl<N> From<[N; 2]> for Vec2<N>
where
    N: Copy,
{
    fn from(value: [N; 2]) -> Self {
        unsafe { Self(*value.get_unchecked(0), *value.get_unchecked(1)) }
    }
}
impl<N> From<&[N; 2]> for Vec2<N>
where
    N: Copy,
{
    fn from(value: &[N; 2]) -> Self {
        unsafe { Self(*value.get_unchecked(0), *value.get_unchecked(1)) }
    }
}

#[cfg(test)]
mod vec_test {
    use super::*;

    #[test]
    fn bounds_wrap() {
        let c1 = Vec2::from((0, 0));
        let c2 = Vec2::from((3, 3));
        assert_eq!(Vec2::from((4, 0)).bounds_wrap(c1, c2), Vec2::from((1, 0)));
        assert_eq!(Vec2::from((0, 4)).bounds_wrap(c1, c2), Vec2::from((0, 1)));
        assert_eq!(Vec2::from((3, 0)).bounds_wrap(c1, c2), Vec2::from((0, 0)));
        assert_eq!(Vec2::from((0, 3)).bounds_wrap(c1, c2), Vec2::from((0, 0)));
    }
}
