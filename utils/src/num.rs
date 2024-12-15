use std::ops::{Add, Shr};

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero_int {
    ($($t:tt)+) => {
        $(
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }
        )+
    };
}
macro_rules! impl_zero_float {
    ($($t:tt)+) => {
        $(
        impl Zero for $t {
            fn zero() -> Self {
                0.0
            }
        }
        )+
    };
}

impl_zero_int!(i8 i16 i32 i64 i128  u8 u16 u32 u64 u128);
impl_zero_float!(f32 f64);
impl_zero_int!(usize isize);

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one_int {
    ($($t:tt)+) => {
        $(
        impl One for $t {
            fn one() -> Self {
                1
            }
        }
        )+
    };
}

macro_rules! impl_one_float {
    ($($t:tt)+) => {
        $(
        impl One for $t {
            fn one() -> Self {
                1.0
            }
        }
        )+
    };
}

impl_one_int!(i8 i16 i32 i64 i128  u8 u16 u32 u64 u128);
impl_one_float!(f32 f64);
impl_one_int!(usize isize);

pub trait Ten {
    fn ten() -> Self;
}

macro_rules! impl_ten_int {
    ($($t:tt)+) => {
        $(
        impl Ten for $t {
            fn ten() -> Self {
                10
            }
        }
        )+
    };
}

macro_rules! impl_ten_float {
    ($($t:tt)+) => {
        $(
        impl Ten for $t {
            fn ten() -> Self {
                10.0
            }
        }
        )+
    };
}

impl_ten_int!(i8 i16 i32 i64 i128  u8 u16 u32 u64 u128);
impl_ten_float!(f32 f64);
impl_ten_int!(usize isize);

pub trait BitwiseAvg
where
    Self: Add<Output = Self> + Shr<Output = Self> + Copy + Sized + One,
{
    fn bitwise_avg(self, other: Self) -> Self {
        (self + other) >> Self::one()
    }
}

macro_rules! impl_bitwise_avg {
    ($($t:tt)+) => {
        $(
        impl BitwiseAvg for $t {}
        )+
    }
}

impl_bitwise_avg!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);

pub trait RemEuclid {
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_remeuclid {
    ($method:ident, $($t:tt)+) => {
        $(
        impl RemEuclid for $t {
            fn rem_euclid(self, rhs: Self) -> Self {
                self.$method(rhs)
            }
        }
        )+
    }
}

impl_remeuclid!(rem_euclid, i8 i16 i32 i64 i128 u8 u16 u32 u64 u128);
