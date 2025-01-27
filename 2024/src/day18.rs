use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::matrix::Matrix;

use crate::common::maze_solving::solve_maze;

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let t = line
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            (t[1], t[0])
        })
        .collect::<Vec<_>>()
}

fn find_max_size(input: &[(u8, u8)]) -> (usize, usize) {
    let (mut max_r, mut max_c) = (0, 0);
    for (r, c) in input.iter() {
        if *r > max_r {
            max_r = *r;
        }
        if *c > max_c {
            max_c = *c;
        }
    }
    (max_r as usize + 1, max_c as usize + 1)
}

fn list_to_grid(input: &[(u8, u8)]) -> Vec<Vec<char>> {
    let (num_r, num_c) = find_max_size(input);
    let mut grid = vec![vec!['.'; num_c]; num_r];
    for (r, c) in input.iter() {
        grid[(*r) as usize][(*c) as usize] = '#';
    }
    grid
}

#[aoc(day18, part1)]
fn part1(input: &[(u8, u8)]) -> usize {
    let mi = input.len().min(1024);
    let grid = list_to_grid(&input[..mi]);
    let mut maze = Matrix::from_fn(grid.len(), grid[0].len(), |(r, c)| grid[r][c]);
    let solution = solve_maze(&maze, (0, 0), (grid.len() - 1, grid[0].len() - 1)).unwrap();
    for (r, c) in solution.iter() {
        *(maze.get_mut((*r, *c)).unwrap()) = '*';
    }
    solution.len() - 1
}

#[aoc(day18, part2)]
fn part2(input: &[(u8, u8)]) -> String {
    let (num_r, num_c) = find_max_size(input);
    let mut maze = Matrix::new(num_r, num_c, '.');

    for (r, c) in input.iter() {
        *maze.get_mut((*r as usize, *c as usize)).unwrap() = '#';
        if solve_maze(&maze, (0, 0), (maze.rows - 1, maze.columns - 1)).is_none() {
            return format!("{:?}", (*c, *r));
        }
    }
    format!("{:?}", (0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1"
            )),
            22
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
            )),
            "(6, 1)".to_string()
        );
    }
}
