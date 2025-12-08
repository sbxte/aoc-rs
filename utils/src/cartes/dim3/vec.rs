use std::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use crate::num::{AbsDiff, RemEuclid, Sqrt, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Vec3<N>(pub N, pub N, pub N);

impl<N> Vec3<N>
where
    N: Copy + Ord + Add<Output = N> + Sub<Output = N> + Rem<Output = N> + RemEuclid,
{
    pub fn bounds_wrap(self, corner1: Self, corner2: Self) -> Self {
        Self(
            corner1.0 + ((self.0 - corner1.0).rem_euclid(corner2.0 - corner1.0)),
            corner1.1 + ((self.1 - corner1.1).rem_euclid(corner2.1 - corner1.1)),
            corner1.2 + ((self.2 - corner1.2).rem_euclid(corner2.2 - corner1.2)),
        )
    }
}

impl<N> Vec3<N>
where
    N: Zero,
{
    pub fn zero() -> Self {
        Self(N::zero(), N::zero(), N::zero())
    }

    pub fn map<F, T>(self, f: F) -> Vec3<T>
    where
        F: Fn(N) -> T,
    {
        Vec3(f(self.0), f(self.1), f(self.2))
    }
}

impl<N> Vec3<N>
where
    N: Ord + Copy,
{
    pub fn get_min_max_corners(corner1: Self, corner2: Self) -> (Self, Self) {
        (
            Self(
                corner1.0.min(corner2.0),
                corner1.1.min(corner2.1),
                corner1.2.min(corner2.2),
            ),
            Self(
                corner1.0.max(corner2.0),
                corner1.1.max(corner2.1),
                corner1.2.min(corner2.2),
            ),
        )
    }

    pub fn bounds_clamp(self, corner1: Self, corner2: Self) -> Self {
        let (min, max) = Self::get_min_max_corners(corner1, corner2);
        Self(
            self.0.clamp(min.0, max.0),
            self.1.clamp(min.1, max.1),
            self.2.clamp(min.2, max.2),
        )
    }

    pub fn is_oob_inclusive(self, corner1: Self, corner2: Self) -> bool {
        let (min, max) = Self::get_min_max_corners(corner1, corner2);
        !(min.0..=max.0).contains(&self.0)
            || !(min.1..=max.1).contains(&self.1)
            || !(min.2..=max.2).contains(&self.2)
    }
}

impl<A> Vec3<A> {
    pub fn from_v<B>(v: Vec3<B>) -> Self
    where
        A: From<B>,
    {
        Self(From::from(v.0), From::from(v.1), From::from(v.2))
    }

    pub fn into_v<B>(self) -> Vec3<B>
    where
        A: Into<B>,
    {
        Vec3(self.0.into(), self.1.into(), self.2.into())
    }
}

impl<N, S> Vec3<N>
where
    S: Add<Output = S>,
    N: Copy + Sub<Output = N> + AbsDiff<SignType = S>,
{
    pub fn manhattan_dist(self, other: Self) -> S {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
impl<N> Vec3<N>
where
    N: Copy + Sub<Output = N> + Mul<Output = N> + Add<Output = N>,
{
    pub fn euclidian_dist_sq(self, other: Self) -> N {
        let delta = self - other;
        delta.0 * delta.0 + delta.1 * delta.1 + delta.2 * delta.2
    }
}
impl<N> Vec3<N>
where
    N: Copy + Sub<Output = N> + Mul<Output = N> + Add<Output = N> + Sqrt,
{
    pub fn euclidian_dist(self, other: Self) -> N {
        self.euclidian_dist_sq(other).sqrt()
    }
}

impl<N> Neg for Vec3<N>
where
    N: Neg<Output = N>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
impl<N> Add for Vec3<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl<N> AddAssign for Vec3<N>
where
    N: Add<Output = N> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<N> Sub for Vec3<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl<N> SubAssign for Vec3<N>
where
    N: Sub<Output = N> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<N> Mul<N> for Vec3<N>
where
    N: Mul<Output = N> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl<N> Div<N> for Vec3<N>
where
    N: Div<Output = N> + Copy,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<N> From<(N, N, N)> for Vec3<N> {
    fn from(value: (N, N, N)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
impl<N> From<&(N, N, N)> for Vec3<N>
where
    N: Copy,
{
    fn from(value: &(N, N, N)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
impl<N> From<[N; 3]> for Vec3<N>
where
    N: Copy,
{
    fn from(value: [N; 3]) -> Self {
        unsafe { Self(*value.get_unchecked(0), *value.get_unchecked(1), *value.get_unchecked(2)) }
    }
}
impl<N> From<&[N; 3]> for Vec3<N>
where
    N: Copy,
{
    fn from(value: &[N; 3]) -> Self {
        unsafe { Self(*value.get_unchecked(0), *value.get_unchecked(1), *value.get_unchecked(2)) }
    }
}
impl From<Vec3<isize>> for Vec3<usize> {
    fn from(value: Vec3<isize>) -> Self {
        Self(value.0 as usize, value.1 as usize, value.2 as usize)
    }
}
impl From<Vec3<usize>> for Vec3<isize> {
    fn from(value: Vec3<usize>) -> Self {
        Self(value.0 as isize, value.1 as isize, value.2 as isize)
    }
}

#[cfg(test)]
mod vec_test {
    use super::*;

    #[test]
    fn bounds_wrap() {
        let c1 = Vec3::from((0, 0, 0));
        let c2 = Vec3::from((3, 3, 3));
        assert_eq!(Vec3::from((4, 0, 0)).bounds_wrap(c1, c2), Vec3::from((1, 0, 0)));
        assert_eq!(Vec3::from((0, 4, 0)).bounds_wrap(c1, c2), Vec3::from((0, 1, 0)));
        assert_eq!(Vec3::from((0, 0, 4)).bounds_wrap(c1, c2), Vec3::from((0, 0, 1)));
        assert_eq!(Vec3::from((3, 0, 0)).bounds_wrap(c1, c2), Vec3::from((0, 0, 0)));
        assert_eq!(Vec3::from((0, 3, 0)).bounds_wrap(c1, c2), Vec3::from((0, 0, 0)));
        assert_eq!(Vec3::from((0, 0, 3)).bounds_wrap(c1, c2), Vec3::from((0, 0, 0)));
    }
}
