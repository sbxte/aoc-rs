use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::num::{One, RemEuclid, Zero};

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
            Up => Vec2(N::zero(), N::one()),
            Down => Vec2(N::zero(), -N::one()),
            Right => Vec2(N::one(), N::zero()),
            Left => Vec2(N::one(), N::zero()),
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
