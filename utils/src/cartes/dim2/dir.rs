use std::ops::Neg;

use crate::cartes::dim2::vec::Vec2;
use crate::num::{One, Zero};

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
