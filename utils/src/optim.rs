#[macro_export]
macro_rules! noreach {
    () => {
        if cfg!(debug_assertions) {
            unreachable!();
        } else {
            unsafe { ::std::hint::unreachable_unchecked() }
        }
    };
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        if cfg!(debug_assertions) {
            unreachable!($fmt $(, $args)*);
        } else {
            unsafe { ::std::hint::unreachable_unchecked() }
        }
    }
}

pub mod prelude {
    pub use noreach;
}
