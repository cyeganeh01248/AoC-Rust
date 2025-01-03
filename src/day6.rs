use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet;

use crate::{common::MyMatrix, parsers::v_grid_no_whitespace};

type Num = u16;

#[aoc_generator(day6)]
fn parse(input: &str) -> (MyMatrix<char>, (isize, isize), u8) {
    let mut guard_r = 0isize;
    let mut guard_c = 0isize;
    let mut dir = 0u8;

    let mut grid = v_grid_no_whitespace(input);
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                '^' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 0;
                }
                '>' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 1;
                }
                'v' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 2;
                }
                '<' => {
                    guard_r = r as isize;
                    guard_c = c as isize;
                    dir = 3;
                }
                _ => {}
            }
        }
    }
    grid[guard_r as usize][guard_c as usize] = '.';
    (grid, (guard_r, guard_c), dir)
}

#[aoc(day6, part1)]
fn part1((grid, (guard_r, guard_c), dir): &(MyMatrix<char>, (isize, isize), u8)) -> Num {
    let grid = grid.clone();

    let (raw_results, _) = traverse_grid(&grid, (*guard_r, *guard_c), *dir);

    let mut unique_results = HashSet::new();
    for (_, r, c) in raw_results {
        unique_results.insert((r, c));
    }
    unique_results.len() as Num
}

#[aoc(day6, part2)]
fn part2((grid, (guard_r, guard_c), dir): &(MyMatrix<char>, (isize, isize), u8)) -> Num {
    let (raw_visited, _) = traverse_grid(grid, (*guard_r, *guard_c), *dir);

    let mut visited = FxHashSet::default();
    visited.reserve(raw_visited.len());
    for (_, r, c) in raw_visited {
        visited.insert((r, c));
    }

    let mut grid_copy = grid.clone();

    // for (new_o_r, new_o_c) in visited {
    visited
        .iter()
        .map(|(new_o_r, new_o_c)| {
            let new_o_r = *new_o_r;
            let new_o_c = *new_o_c;
            if new_o_r == *guard_r && new_o_c == *guard_c {
                return 0;
            }
            grid_copy[new_o_r as usize][new_o_c as usize] = '#';
            let (_, result) = traverse_grid(&grid_copy, (*guard_r, *guard_c), *dir);
            grid_copy[new_o_r as usize][new_o_c as usize] = '.';
            if result == TraverseResult::Loop {
                return 1;
            }
            0
        })
        .sum()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TraverseResult {
    Edge,
    Loop,
}

fn traverse_grid(
    grid: &[Vec<char>],
    (mut guard_r, mut guard_c): (isize, isize),
    mut dir: u8,
) -> (FxHashSet<(u8, isize, isize)>, TraverseResult) {
    let mut visited = FxHashSet::default();
    visited.reserve(grid.len() * grid[0].len());
    visited.insert((dir, guard_r, guard_c));
    loop {
        let (dr, dc) = match dir {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        if guard_r + dr < 0
            || guard_r + dr >= grid.len() as isize
            || guard_c + dc < 0
            || guard_c + dc >= grid[0].len() as isize
        {
            return (visited, TraverseResult::Edge);
        } else if visited.contains(&(dir, guard_r + dr, guard_c + dc)) {
            return (visited, TraverseResult::Loop);
        } else {
            if grid[(guard_r + dr) as usize][(guard_c + dc) as usize] == '#' {
                dir = (dir + 1) % 4;
            } else {
                guard_r += dr;
                guard_c += dc;
                if visited.contains(&(dir, guard_r, guard_c)) {
                    panic!("How did we get here?");
                }
            }
            visited.insert((dir, guard_r, guard_c));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            41
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            6
        );
    }
}
