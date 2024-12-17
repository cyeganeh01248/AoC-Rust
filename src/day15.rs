use aoc_runner_derive::{aoc, aoc_generator};

use crate::{common::Matrix, parsers::v_grid_no_whitespace};

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
    sum_gps(&grid)
}

fn sum_gps(grid: &Matrix<char>) -> Num {
    let mut sum = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 'O' || *cell == '[' {
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
        for cell in row.iter() {
            let new_cell = match *cell {
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
    for robot_move in moves {
        move_robot(&mut wider_grid, &mut robot_r, &mut robot_c, *robot_move);
    }
    sum_gps(&wider_grid)
}
fn move_robot(grid: &mut Matrix<char>, robot_r: &mut i64, robot_c: &mut i64, robot_move: u8) {
    let (result, mut objects_to_move) = get_objects_to_move(grid, *robot_r, *robot_c, robot_move);
    if !result {
        return;
    }
    let (dr, dc) = match robot_move {
        0 => {
            objects_to_move.sort_by(|(r1, _, _), (r2, _, _)| r1.cmp(r2));
            (-1, 0)
        }
        1 => {
            objects_to_move.sort_by(|(_, c1, _), (_, c2, _)| c2.cmp(c1));
            (0, 1)
        }
        2 => {
            objects_to_move.sort_by(|(r1, _, _), (r2, _, _)| r2.cmp(r1));
            (1, 0)
        }
        3 => {
            objects_to_move.sort_by(|(_, c1, _), (_, c2, _)| c1.cmp(c2));
            (0, -1)
        }
        _ => unreachable!(),
    };

    // let
    if robot_move == 0 || robot_move == 2 {
        for (r, c, t) in objects_to_move.iter() {
            grid[*r as usize][*c as usize] = '.';
            grid[(r + dr) as usize][(c + dc) as usize] = *t;
        }
        grid[*robot_r as usize][*robot_c as usize] = '.';
        *robot_r += dr;
        *robot_c += dc;
    } else {
        for (r, c, _) in objects_to_move.iter() {
            grid[(r + dr) as usize][(c + dc) as usize] = grid[*r as usize][*c as usize];
        }
        grid[*robot_r as usize][*robot_c as usize] = '.';
        *robot_r += dr;
        *robot_c += dc;
    }
}

fn get_objects_to_move(
    grid: &Matrix<char>,
    object_r: i64,
    object_c: i64,
    object_move: u8,
) -> (bool, Vec<(i64, i64, char)>) {
    let (dr, dc) = match object_move {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    };

    match grid[object_r as usize][object_c as usize] {
        '.' => (true, vec![]),
        '[' => {
            let (result, mut objects_straight) =
                get_objects_to_move(grid, object_r + dr, object_c + dc, object_move);
            if !result {
                return (false, vec![]);
            }
            objects_straight.push((object_r, object_c, '['));
            if object_move == 0 || object_move == 2 {
                objects_straight.push((object_r, object_c + 1, ']'));
                let (result, objects_right) =
                    get_objects_to_move(grid, object_r + dr, object_c + dc + 1, object_move);
                if !result {
                    return (false, vec![]);
                }
                objects_straight.extend(objects_right);
            }
            (true, objects_straight)
        }
        ']' => {
            let (result, mut objects_straight) =
                get_objects_to_move(grid, object_r + dr, object_c + dc, object_move);
            if !result {
                return (false, vec![]);
            }
            objects_straight.push((object_r, object_c, ']'));
            if object_move == 0 || object_move == 2 {
                objects_straight.push((object_r, object_c - 1, '['));
                let (result, objects_left) =
                    get_objects_to_move(grid, object_r + dr, object_c + dc - 1, object_move);
                if !result {
                    return (false, vec![]);
                }
                objects_straight.extend(objects_left);
            }
            (true, objects_straight)
        }
        '#' => (false, vec![]),
        '@' => {
            let (result, mut objects_straight) =
                get_objects_to_move(grid, object_r + dr, object_c + dc, object_move);
            if !result {
                return (false, vec![]);
            }
            objects_straight.push((object_r, object_c, '@'));
            (true, objects_straight)
        }
        _ => unreachable!(),
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
            1231
        );
    }
}
