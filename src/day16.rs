use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    common::{print_matrix, Matrix},
    parsers::v_grid_no_whitespace,
};
use std::{fmt::Display, rc::Rc};

type Num = u64;

#[aoc_generator(day16)]
fn parse(input: &str) -> (Matrix<char>, (isize, isize), (isize, isize)) {
    let mut grid = v_grid_no_whitespace(input);
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (r as isize, c as isize);
            }
            if *cell == 'E' {
                end = (r as isize, c as isize);
            }
        }
    }
    grid[start.0 as usize][start.1 as usize] = '.';
    grid[end.0 as usize][end.1 as usize] = '.';
    (grid, start, end)
}

#[aoc(day16, part1)]
fn part1((grid, start, end): &(Matrix<char>, (isize, isize), (isize, isize))) -> Num {
    a_star(grid.to_owned(), *start, *end);
    todo!()
}

#[derive(Debug, Clone)]
struct Node {
    g: isize,
    h: isize,
    f: isize,
    c: char,
    p: (isize, isize),
    parent: Option<Rc<Node>>,
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

// https://www.geeksforgeeks.org/a-search-algorithm/
fn a_star(grid: Matrix<char>, start: (isize, isize), end: (isize, isize)) -> Vec<(isize, isize)> {
    let mut nodes = Matrix::default();
    for (r, row) in grid.iter().enumerate() {
        let mut row_add = vec![];
        for (c, cell) in row.iter().enumerate() {
            row_add.push(Node {
                g: 0,
                h: 0,
                f: 0,
                c: *cell,
                p: (r as isize, c as isize),
                parent: None,
            });
        }
        nodes.push(row_add);
    }

    print_matrix(&nodes);
    todo!()
}
// #[aoc(day16, part2)]
// fn part2((grid, start, end): &(Matrix<char>, (isize, isize), (isize, isize))) -> Num {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(
            part1(&parse(
                "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############\
            "
            )),
            7036
        );
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(
            part1(&parse(
                "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################\
            "
            )),
            11048
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
