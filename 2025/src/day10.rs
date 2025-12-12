use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>, Vec<u64>)> {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .iter()
        .map(|&line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            let mut diagram_n = 0u64;
            let diagram = parts[0]
                .replace("[", "")
                .replace("]", "")
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid character in diagram"),
                })
                .collect::<Vec<_>>();
            // Use same bit ordering as buttons - bit 0 is rightmost
            for (i, is_on) in diagram.iter().enumerate() {
                if *is_on {
                    diagram_n |= 1 << i;
                }
            }

            let joltage = parts[parts.len() - 1]
                .to_string()
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|&s| {
                    s.replace("(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|button| {
                    let mut button_n = 0u64;
                    for n in button {
                        button_n |= 1 << n;
                    }
                    button_n
                })
                .collect::<Vec<_>>();

            (diagram_n, buttons, joltage)
        })
        .collect()
}

use nalgebra::{DMatrix, DVector};
use pathfinding::{directed::bfs::bfs, prelude::astar};

//     [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
#[aoc(day10, part1)]
fn part1(input: &Vec<(u64, Vec<u64>, Vec<u64>)>) -> usize {
    input
        .iter()
        .map(|(diagram, buttons, _)| {
            let initial_machine = 0;
            let path = bfs(
                &initial_machine,
                |state| {
                    let mut next_states = Vec::new();
                    for button in buttons.iter() {
                        let new_state = *state ^ *button;
                        next_states.push(new_state);
                    }
                    next_states
                },
                |state| *state == *diagram,
            )
            .unwrap();
            path.len() - 1
        })
        .sum::<usize>()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<(u64, Vec<u64>, Vec<u64>)>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const EXAMPLE_INPUT: &str = indoc! {"
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 33);
    }
}
