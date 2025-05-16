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

pub trait SignedType {
    type SignType;

    fn to_signtype(self) -> Self::SignType;
}

macro_rules! impl_signtype {
    ($($a:tt $b:tt),+) => {
        $(
        impl SignedType for $a {
            type SignType = $b;
            fn to_signtype(self) -> Self::SignType {
                self as Self::SignType
            }
        }
        impl SignedType for $b {
            type SignType = $a;
            fn to_signtype(self) -> Self::SignType {
                self as Self::SignType
            }
        }
        )+
    }
}

impl_signtype!(i8 u8, i16 u16, i32 u32, i64 u64, isize usize);

pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ($method:ident, $($t:tt)+) => {
        $(
        impl Abs for $t {
            fn abs(self) -> Self {
                self.$method()
            }
        }
        )+
    }
}

impl_abs!(abs, i8 i16 i32 i64 isize);

pub trait AbsDiff: SignedType {
    fn abs_diff(self, other: Self) -> Self::SignType;
}

macro_rules! impl_abs_diff {
    ($method:ident; $($t:tt $out:tt),+) => {
        $(
        impl AbsDiff for $t {
            fn abs_diff(self, other: Self) -> Self::SignType {
                self.$method(other)
            }
        }
        )+
    }
}

impl_abs_diff!(abs_diff; i8 u8, i16 u16, i32 u32, i64 u64, isize usize);

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($method:ident; $($t:tt)+) => {
        $(
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                self.$method()
            }
        }
        )+
    }
}

impl_sqrt!(isqrt; i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);
impl_sqrt!(sqrt; f32 f64);

pub trait Pow<E> {
    fn pow(self, exp: E) -> Self;
}

macro_rules! impl_pow {
    ($method:ident; $($t:tt $e:tt),+) => {
        $(
        impl Pow<$e> for $t {
            fn pow(self, exp: $e) -> Self {
                self.$method(exp)
            }
        }
        )+
    }
}

impl_pow!(pow; i8 u32, i16 u32, i32 u32, i64 u32, i128 u32, u8 u32, u16 u32, u32 u32, u64 u32, u128 u32);
