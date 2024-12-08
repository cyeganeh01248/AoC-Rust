use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::parsers::v_grid_no_whitespace;

type Num = u16;

#[derive(Debug, Copy, Clone)]
struct Antenna {
    row: isize,
    col: isize,
    code: char,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<Antenna>, usize, usize) {
    let grid: Vec<Vec<char>> = v_grid_no_whitespace(input);
    let mut antennas = Vec::new();
    for (row, row_chars) in grid.iter().enumerate() {
        for (col, code) in row_chars.iter().enumerate() {
            if *code != '.' {
                antennas.push(Antenna {
                    row: row as isize,
                    col: col as isize,
                    code: *code,
                });
            }
        }
    }
    (antennas, grid.len(), grid[0].len())
}

#[aoc(day8, part1)]
fn part1((input, width, height): &(Vec<Antenna>, usize, usize)) -> Num {
    let mut global_pts = HashSet::with_capacity(input.len() * input.len());

    for ant_a_i in 0..(input.len() - 1) {
        for ant_b_i in (ant_a_i + 1)..input.len() {
            let ant_a = input[ant_a_i];
            let ant_b = input[ant_b_i];
            if ant_a.code != ant_b.code {
                continue;
            }
            let slope = ((ant_b.row - ant_a.row), (ant_b.col - ant_a.col));

            let mut pts = HashSet::new();
            pts.insert((ant_a.row - slope.0, ant_a.col - slope.1));
            pts.insert((ant_b.row + slope.0, ant_b.col + slope.1));
            for (pt_r, pt_c) in pts {
                if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                    continue;
                }
                // println!("({}, {})", pt_r, pt_c);
                global_pts.insert((pt_r as usize, pt_c as usize));
            }
        }
    }
    global_pts.len() as Num
}

#[aoc(day8, part2)]
fn part2((input, width, height): &(Vec<Antenna>, usize, usize)) -> Num {
    let mut global_pts = HashSet::with_capacity(input.len() * input.len());

    for ant_a_i in 0..(input.len() - 1) {
        for ant_b_i in (ant_a_i + 1)..input.len() {
            let ant_a = input[ant_a_i];
            let ant_b = input[ant_b_i];
            if ant_a.code != ant_b.code {
                continue;
            }
            let slope = ((ant_b.row - ant_a.row), (ant_b.col - ant_a.col));

            let (mut pt_r, mut pt_c) = (ant_a.row, ant_a.col);
            loop {
                if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                    break;
                }
                global_pts.insert((pt_r, pt_c));
                pt_r -= slope.0;
                pt_c -= slope.1;
            }
            let (mut pt_r, mut pt_c) = (ant_b.row, ant_b.col);
            loop {
                if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                    break;
                }
                global_pts.insert((pt_r, pt_c));
                pt_r += slope.0;
                pt_c += slope.1;
            }
        }
    }
    let mut t = global_pts.iter().collect::<Vec<_>>();
    t.sort();

    global_pts.len() as Num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\
"
            )),
            14
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\
"
            )),
            34
        );
    }
}
