use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::HashSet;

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut towels = split[0]
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // towels.sort_by(|a, b| b.len().cmp(&a.len()));

    let patterns = split[1]
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (towels, patterns)
}

#[aoc(day19, part1)]
fn part1((towels, patterns): &(Vec<String>, Vec<String>)) -> usize {
    let pattern = String::from("^(") + &towels.join("|") + ")+$";
    let re = Regex::new(&pattern).unwrap();

    patterns
        .into_iter()
        .map(|haystack| {
            let occurances = re.find_iter(haystack).count();
            if occurances > 0 {
                1
            } else {
                0
            }
        })
        .sum()
}

use regex::{bytes::RegexSet, Regex};
#[aoc(day19, part2)]
fn part2((full_towels, patterns): &(Vec<String>, Vec<String>)) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            )),
            6
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            )),
            16
        );
    }
}
