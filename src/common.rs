use std::fmt::Display;

pub use fxhash::FxHashMap as HashMap;
pub use fxhash::FxHashSet as HashSet;
use pad::Alignment;
use pad::PadStr;

pub type Matrix<T> = Vec<Vec<T>>;

pub fn int_len<T>(num: T) -> u32
where
    f64: From<T>,
{
    f64::from(num).log10().floor() as u32 + 1u32
}

#[allow(dead_code)]
pub fn print_matrix<T>(matrix: &Matrix<T>)
where
    T: Display,
{
    let mut max_len = 0;
    for row in matrix.iter() {
        for item in row.iter() {
            max_len = max_len.max(item.to_string().len());
        }
    }
    for row in matrix {
        for item in row {
            print!(
                "{} ",
                item.to_string()
                    .pad_to_width_with_alignment(max_len, Alignment::Left)
            );
        }
        println!();
    }
}
