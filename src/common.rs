use std::fmt::Display;
use std::io::stdout;
use std::io::Write;

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
    write_matrix(matrix, &mut stdout());
}

#[allow(dead_code)]
pub fn print_matrix_spaced<T>(matrix: &Matrix<T>, space: String)
where
    T: Display,
{
    write_matrix_spaced(matrix, &mut stdout(), space);
}

#[allow(dead_code)]
pub fn write_matrix<T>(matrix: &Matrix<T>, file: &mut impl Write)
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
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            write!(file, "{}", s).expect("Unable to write to file");
        }
        write!(file, "\n").expect("Unable to write to file");
    }
}
#[allow(dead_code)]
pub fn write_matrix_spaced<T>(matrix: &Matrix<T>, file: &mut impl Write, space: String)
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
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            write!(file, "{}{space}", s).expect("Unable to write to file");
        }
        write!(file, "\n").expect("Unable to write to file");
    }
}
