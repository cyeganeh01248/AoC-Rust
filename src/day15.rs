use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::VecDeque, io::Write};

use crate::{
    common::{print_matrix, HashSet, Matrix},
    parsers::v_grid_no_whitespace,
};

type Num = u64;

#[aoc_generator(day15)]
fn parse(input: &str) -> (Matrix<char>, Vec<u8>, (i64, i64)) {
    let mut i = input.split("\n\n");
    let top = i.next().unwrap();
    let bottom = i.next().unwrap().replace("\n", "");
    let grid = v_grid_no_whitespace(top);
    let (mut robot_r, mut robot_c) = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == '@' {
                (robot_r, robot_c) = (r as i64, c as i64);
            }
        }
    }
    let mut moves = vec![];
    for move_i in bottom.chars() {
        match move_i {
            '^' => {
                moves.push(0);
            }
            '>' => {
                moves.push(1);
            }
            'v' => {
                moves.push(2);
            }
            '<' => {
                moves.push(3);
            }
            _ => {
                unreachable!("Invalid move: {}", move_i);
            }
        }
    }
    (grid, moves, (robot_r, robot_c))
}

#[aoc(day15, part1)]
fn part1((grid, moves, (robot_r, robot_c)): &(Matrix<char>, Vec<u8>, (i64, i64))) -> Num {
    let mut grid = grid.to_owned();
    let mut robot_r = *robot_r;
    let mut robot_c = *robot_c;
    for robot_move in moves {
        if grid[robot_r as usize][robot_c as usize] != '@' {
            panic!("Robot was unaligned");
        }
        let (dr, dc) = match robot_move {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        let mut can_move = true;
        let mut i = 1;
        loop {
            let (nr, nc) = (robot_r + dr * i, robot_c + dc * i);
            if nr < 0
                || nr >= grid.len() as i64
                || nc < 0
                || nc >= grid[0].len() as i64
                || grid[nr as usize][nc as usize] == '#'
            {
                can_move = false;
                break;
            }
            if grid[nr as usize][nc as usize] == '.' {
                break;
            }
            i += 1;
        }
        if can_move {
            for j in (1..=i).rev() {
                let (nr, nc) = (robot_r + dr * j, robot_c + dc * j);
                let (nr_2, nc_2) = (robot_r + dr * (j - 1), robot_c + dc * (j - 1));
                grid[nr as usize][nc as usize] = grid[nr_2 as usize][nc_2 as usize];
            }
            grid[robot_r as usize][robot_c as usize] = '.';
            robot_r += dr;
            robot_c += dc;
        }
    }
    let mut sum = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 'O' {
                sum += (r * 100 + c) as Num;
            }
        }
    }
    sum
}

#[aoc(day15, part2)]
fn part2((grid, moves, (robot_r, robot_c)): &(Matrix<char>, Vec<u8>, (i64, i64))) -> Num {
    let mut wider_grid = vec![vec!['.'; 0]; grid.len()];
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let mut new_cell = match *cell {
                '.' => ('.', '.'),
                'O' => ('[', ']'),
                '#' => ('#', '#'),
                '@' => ('@', '.'),
                _ => unreachable!(),
            };
            wider_grid[r].push(new_cell.0);
            wider_grid[r].push(new_cell.1);
        }
    }
    let mut robot_r = *robot_r;
    let mut robot_c = *robot_c * 2;
    println!();
    for robot_move in moves {
        println!("before");
        print_matrix(&wider_grid);
        move_robot(&mut wider_grid, &mut robot_r, &mut robot_c, *robot_move);
        println!("after");
        print_matrix(&wider_grid);
    }
    todo!()
}
fn move_robot(grid: &mut Matrix<char>, robot_r: &mut i64, robot_c: &mut i64, robot_move: u8) {
    move_object(grid, *robot_r, *robot_c, robot_move);
}

fn move_object(grid: &mut Matrix<char>, object_r: i64, object_c: i64, object_move: u8) -> bool {
    let (dr, dc) = match object_move {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    };
    let (nr, nc) = (object_r + dr, object_c + dc);
    if grid[nr as usize][nc as usize] == '.' {
        grid[nr as usize][nc as usize] = grid[object_r as usize][object_c as usize];
        return true;
    } else if grid[nr as usize][nc as usize] == '#' {
        return false;
    } else if grid[nr as usize][nc as usize] == '[' {
        let result = move_object(grid, nr + dr, nc + dc, object_move);
        if result {
            grid[nr as usize][nc as usize] = grid[object_r as usize][object_c as usize];
            if object_move == 0 || object_move == 2 {
                grid[nr as usize][(nc + 1) as usize] = '.';
            }
        }
        return result;
    } else if grid[nr as usize][nc as usize] == ']' {
        let result = move_object(grid, nr + dr, nc + dc, object_move);
        if result {
            grid[nr as usize][nc as usize] = grid[object_r as usize][object_c as usize];
            if object_move == 0 || object_move == 2 {
                grid[nr as usize][(nc - 1) as usize] = '.';
            }
        }
        return result;
    } else {
        let result = move_object(grid, nr + dr, nc + dc, object_move);
        grid[nr as usize][nc as usize] = grid[object_r as usize][object_c as usize];
        return result;
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
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\
"
            )),
            10092
        );
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(
            part2(&parse(
                "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\
"
            )),
            9021
        );
    }
    #[test]
    fn part2_example_1() {
        assert_eq!(
            part2(&parse(
                "\
#######
#...#.#
#.OO..#
#.OOO@#
#..O..#
#.....#
#######

<vv<<^^<<^^\
"
            )),
            9021
        );
    }
}
