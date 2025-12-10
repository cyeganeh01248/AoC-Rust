use std::i64;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|points| (points[0], points[1]))
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;

    for p1_i in 0..input.len() {
        for p2_i in p1_i + 1..input.len() {
            let p1 = input[p1_i];
            let p2 = input[p2_i];

            let area = ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

#[aoc(day9, part2)]
fn part2(input: &[(i64, i64)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), "<RESULT>");
    }
}
