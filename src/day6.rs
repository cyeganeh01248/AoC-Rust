use aoc_runner_derive::{aoc, aoc_generator};

use crate::parsers::v_grid_no_whitespace;
#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<char>> {
    v_grid_no_whitespace(input)
}

#[aoc(day6, part1)]
fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut grid = grid.clone();
    let mut count = 0;
    let (mut guard_r, mut guard_c) = (0isize, 0isize);
    let mut dir = 0;

    'find_guard: for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if ['^', '>', 'v', '<'].contains(cell) {
                guard_r = r as isize;
                guard_c = c as isize;
                dir = match cell {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                };
                break 'find_guard;
            }
        }
    }

    // grid[guard_r as usize][guard_c as usize] = '#';
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
            count += 1;
            break;
        };
        if grid[(guard_r + dr) as usize][(guard_c + dc) as usize] == '#' {
            dir = (dir + 1) % 4;
            grid[guard_r as usize][guard_c as usize] = match dir {
                0 => '^',
                1 => '>',
                2 => 'v',
                3 => '<',
                _ => unreachable!(),
            };
        } else {
            grid[guard_r as usize][guard_c as usize] = 'X';
            if grid[(guard_r + dr) as usize][(guard_c + dc) as usize] != 'X' {
                count += 1;
            }
            guard_r += dr;
            guard_c += dc;
            grid[guard_r as usize][guard_c as usize] = match dir {
                0 => '^',
                1 => '>',
                2 => 'v',
                3 => '<',
                _ => unreachable!(),
            };
        }
    }

    count
}

#[aoc(day6, part2)]
fn part2(grid: &Vec<Vec<char>>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "....#.....
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
                "....#.....
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
            "<RESULT>"
        );
    }
}
