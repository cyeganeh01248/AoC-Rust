use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut g1s = vec![];
    let mut g2s = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let g1 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let g2 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        g1s.push(g1);
        g2s.push(g2);
    }
    (g1s, g2s)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut g1s = input.0.clone();
    let mut g2s = input.1.clone();
    g1s.sort();
    g2s.sort();

    let mut dist = 0;

    for (g1, g2) in g1s.iter().zip(g2s.iter()) {
        dist += g1.max(g2) - g1.min(g2);
    }
    dist
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut scores = HashMap::new();
    for n2 in input.1.iter() {
        scores.entry(n2).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut score = 0;

    for n1 in input.0.iter() {
        let cnt = match scores.get(n1) {
            Some(cnt) => *cnt,
            None => 0,
        };
        score += n1 * cnt;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            11
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            31
        );
    }
}
