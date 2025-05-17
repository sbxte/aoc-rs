use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Pos: Eq + Copy + Add + Sub + Neg {
    type N: Eq + Add + Sub + Neg + Mul + Div;

    /// Euclidean distance squared
    fn euclid_dst_sq(self, other: Self) -> Self::N;

    /// Taxicab distance
    fn taxicab_dst(self, other: Self) -> Self::N;
}
