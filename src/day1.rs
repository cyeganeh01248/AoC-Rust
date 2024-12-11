use crate::common::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Num = i32;

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<Num>, Vec<Num>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut g1s = vec![];
    let mut g2s = vec![];

    input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let g1 = caps.get(1).unwrap().as_str().parse::<Num>().unwrap();
        let g2 = caps.get(2).unwrap().as_str().parse::<Num>().unwrap();
        g1s.push(g1);
        g2s.push(g2);
    });
    (g1s, g2s)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<Num>, Vec<Num>)) -> Num {
    let mut g1s = input.0.clone();
    let mut g2s = input.1.clone();
    g1s.sort_unstable();
    g2s.sort_unstable();

    g1s.iter()
        .zip(g2s.iter())
        .map(|(g1, g2)| g1.max(g2) - g1.min(g2))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<Num>, Vec<Num>)) -> Num {
    let mut scores = HashMap::default();
    for n2 in input.1.iter() {
        scores.entry(n2).and_modify(|n| *n += 1).or_insert(1);
    }

    input
        .0
        .iter()
        .map(|n1| {
            n1 * match scores.get(n1) {
                Some(cnt) => *cnt,
                None => 0,
            }
        })
        .sum()
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
