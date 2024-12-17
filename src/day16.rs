use aoc_runner_derive::{aoc, aoc_generator};

use crate::{common::Matrix, parsers::v_grid_no_whitespace};

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
fn part1((_grid, _start, _end): &(Matrix<char>, (isize, isize), (isize, isize))) -> Num {
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
