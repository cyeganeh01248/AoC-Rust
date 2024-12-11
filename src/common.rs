#![allow(dead_code)]

pub fn int_len<T>(num: T) -> u32
where
    f64: From<T>,
{
    f64::from(num).log10().floor() as u32 + 1u32
}

pub type Matrix<T> = Vec<Vec<T>>;
pub use fxhash::FxHashMap as HashMap;
pub use fxhash::FxHashSet as HashSet;
