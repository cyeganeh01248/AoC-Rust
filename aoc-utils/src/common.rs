#![allow(dead_code)]
use std::fmt::Display;
use std::io::Write;

pub use fxhash::FxHashMap as HashMap;
pub use fxhash::FxHashSet as HashSet;
use pad::Alignment;
use pad::PadStr;

pub type MyMatrix<T> = Vec<Vec<T>>;

pub fn int_len<T>(num: T) -> u32
where
    f64: From<T>,
{
    f64::from(num).log10().floor() as u32 + 1u32
}

pub fn print_matrix<T>(matrix: &MyMatrix<T>)
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
            print!("{}", s);
        }
        println!();
    }
}

pub fn print_matrix_spaced<T>(matrix: &MyMatrix<T>, space: String)
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
            print!("{}{space}", s);
        }
        println!();
    }
}

pub fn write_matrix<T>(matrix: &MyMatrix<T>, file: &mut impl Write)
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
        writeln!(file).expect("Unable to write to file");
    }
}
pub fn write_matrix_spaced<T>(matrix: &MyMatrix<T>, file: &mut impl Write, space: String)
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
        writeln!(file).expect("Unable to write to file");
    }
}
pub const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub mod maze_solving {
    use pathfinding::{directed::bfs::bfs, matrix::Matrix};

    pub fn parse_maze(maze_str: &str) -> Matrix<char> {
        let maze_vec = maze_str
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Matrix::from_fn(maze_vec.len(), maze_vec[0].len(), |(r, c)| maze_vec[r][c])
    }

    pub fn parse_maze_with_start_end(
        maze_str: &str,
    ) -> (Matrix<char>, (usize, usize), (usize, usize)) {
        let mut maze_vec = maze_str
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (r, row) in maze_vec.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                if *cell == 'S' {
                    start = (r, c);
                    *cell = '.';
                }
                if *cell == 'E' {
                    end = (r, c);
                    *cell = '.';
                }
            }
        }

        (
            Matrix::from_fn(maze_vec.len(), maze_vec[0].len(), |(r, c)| maze_vec[r][c]),
            start,
            end,
        )
    }

    pub fn solve_maze(
        maze: &Matrix<char>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        bfs(
            &start,
            |(r, c)| {
                let mut scs = vec![];
                for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (nr, nc) = ((*r as i32 + dir.0) as usize, (*c as i32 + dir.1) as usize);
                    if let Some(cell) = maze.get((nr, nc)) {
                        if *cell == '.' {
                            scs.push((nr, nc));
                        }
                    }
                }
                scs
            },
            |p| *p == end,
        )
    }

    pub fn print_maze(maze: &Matrix<char>) {
        for row in maze.iter() {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}
