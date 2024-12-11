use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{common::Matrix, parsers::v_grid_no_whitespace};

type Num = u32;

#[aoc_generator(day10)]
fn parse(input: &str) -> Matrix<u8> {
    v_grid_no_whitespace(input)
}

#[aoc(day10, part1)]
fn part1(input: &Matrix<u8>) -> Num {
    let mut trail_heads = vec![];

    for (r, row) in input.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 0 {
                trail_heads.push((r as isize, c as isize));
            }
        }
    }

    let mut count = 0;
    for (th_r, th_c) in trail_heads.iter() {
        let mut nine_poses = HashSet::new();
        bfs_to_peak_cnt_num_nines(input, *th_r, *th_c, 0, &mut nine_poses);
        count += nine_poses.len() as Num;
    }

    // for
    count
}

fn bfs_to_peak_cnt_num_nines(
    input: &[Vec<u8>],
    r: isize,
    c: isize,
    d: u8,
    nine_poses: &mut HashSet<(isize, isize)>,
) {
    // println!("{}, {}, {}", r, c, d);
    if d == 9 {
        nine_poses.insert((r, c));
        return;
    }
    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let p_r = r + dr;
        let p_c = c + dc;
        if p_r < 0 || p_r >= input.len() as isize || p_c < 0 || p_c >= input[0].len() as isize {
            continue;
        }
        if input[p_r as usize][p_c as usize] == d + 1 {
            bfs_to_peak_cnt_num_nines(input, p_r, p_c, d + 1, nine_poses);
        }
    }
}

fn bfs_to_peak_cnt(input: &[Vec<u8>], r: isize, c: isize, d: u8) -> Num {
    // println!("{}, {}, {}", r, c, d);
    if d == 9 {
        return 1;
    }
    let mut count = 0;
    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let p_r = r + dr;
        let p_c = c + dc;
        if p_r < 0 || p_r >= input.len() as isize || p_c < 0 || p_c >= input[0].len() as isize {
            continue;
        }
        if input[p_r as usize][p_c as usize] == d + 1 {
            count += bfs_to_peak_cnt(input, p_r, p_c, d + 1);
        }
    }
    count
}

#[aoc(day10, part2)]
fn part2(input: &Matrix<u8>) -> Num {
    let mut trail_heads = vec![];

    for (r, row) in input.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 0 {
                trail_heads.push((r as isize, c as isize));
            }
        }
    }

    let mut count = 0;
    for (th_r, th_c) in trail_heads.iter() {
        count += bfs_to_peak_cnt(input, *th_r, *th_c, 0);
    }

    // for
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732\
"
            )),
            36
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732\
"
            )),
            81
        );
    }
}
