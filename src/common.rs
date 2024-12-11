#![allow(dead_code)]

pub fn int_len<T>(num: T) -> u32
where
    f64: From<T>,
{
    return f64::from(num).log10().floor() as u32 + 1u32;
}
