use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::{
    matrix::Matrix,
    prelude::{astar_bag_collect, dijkstra},
};

use aoc_utils::common::maze_solving::parse_maze_with_start_end;
use aoc_utils::common::HashSet;

type Num = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dirs {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> (Matrix<char>, (usize, usize), (usize, usize)) {
    parse_maze_with_start_end(input)
}

#[aoc(day16, part1)]
fn part1((maze, start, end): &(Matrix<char>, (usize, usize), (usize, usize))) -> Num {
    let pos = (*start, Dirs::Right);

    let path = dijkstra(
        &pos,
        |((r, c), d)| {
            let mut scs = vec![];
            let dir_str = match d {
                Dirs::Up => (-1, 0),
                Dirs::Right => (0, 1),
                Dirs::Down => (1, 0),
                Dirs::Left => (0, -1),
            };
            let (nr, nc) = (*r as isize + dir_str.0, *c as isize + dir_str.1);
            if let Some(cell) = maze.get((nr as usize, nc as usize)) {
                if *cell == '.' {
                    scs.push((((nr as usize, nc as usize), *d), 1));
                }
            }
            scs.push((
                (
                    ({ *r }, { *c }),
                    match d {
                        Dirs::Up => Dirs::Right,
                        Dirs::Right => Dirs::Down,
                        Dirs::Down => Dirs::Left,
                        Dirs::Left => Dirs::Up,
                    },
                ),
                1000,
            ));
            scs.push((
                (
                    ({ *r }, { *c }),
                    match d {
                        Dirs::Up => Dirs::Left,
                        Dirs::Right => Dirs::Up,
                        Dirs::Down => Dirs::Right,
                        Dirs::Left => Dirs::Down,
                    },
                ),
                1000,
            ));

            scs
        },
        |p| p.0 == *end,
    );
    path.unwrap().1
}
#[aoc(day16, part2)]
fn part2((maze, start, end): &(Matrix<char>, (usize, usize), (usize, usize))) -> Num {
    let pos = (*start, Dirs::Right);

    let paths = astar_bag_collect(
        &pos,
        |((r, c), d)| {
            let mut scs = vec![];
            let dir_str = match d {
                Dirs::Up => (-1, 0),
                Dirs::Right => (0, 1),
                Dirs::Down => (1, 0),
                Dirs::Left => (0, -1),
            };
            let (nr, nc) = (*r as isize + dir_str.0, *c as isize + dir_str.1);
            if let Some(cell) = maze.get((nr as usize, nc as usize)) {
                if *cell == '.' {
                    scs.push((((nr as usize, nc as usize), *d), 1));
                }
            }
            scs.push((
                (
                    ({ *r }, { *c }),
                    match d {
                        Dirs::Up => Dirs::Right,
                        Dirs::Right => Dirs::Down,
                        Dirs::Down => Dirs::Left,
                        Dirs::Left => Dirs::Up,
                    },
                ),
                1000,
            ));
            scs.push((
                (
                    ({ *r }, { *c }),
                    match d {
                        Dirs::Up => Dirs::Left,
                        Dirs::Right => Dirs::Up,
                        Dirs::Down => Dirs::Right,
                        Dirs::Left => Dirs::Down,
                    },
                ),
                1000,
            ));

            scs
        },
        |_| 1,
        |p| p.0 == *end,
    )
    .unwrap();

    let mut locations = HashSet::default();
    for path in paths.0.iter() {
        for (pos, _) in path.iter() {
            locations.insert(*pos);
        }
    }
    locations.len() as Num
}

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
    #[test]
    fn part2_example_1() {
        assert_eq!(
            part2(&parse(
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
            45
        );
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(
            part2(&parse(
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
            64
        );
    }
}
