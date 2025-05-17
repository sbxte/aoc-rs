pub trait ToUsize {
    fn to_usize(self) -> usize;
}

macro_rules! impl_asusize {
    ($($t:tt)+) => {
        $(
            impl ToUsize for $t {
                fn to_usize(self) -> usize {
                    self as usize
                }
            }
        )+
    }
}

impl_asusize!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);
