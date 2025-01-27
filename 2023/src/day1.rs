use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::common::HashMap;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|n| n.to_owned()).collect()
}

#[aoc(day1, part1)]
fn part1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let mut num = String::new();
            for c in line.chars() {
                if c.is_numeric() {
                    num.push(c);
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_numeric() {
                    num.push(c);
                    break;
                }
            }
            num.parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(lines: &[String]) -> u32 {
    let mut map_forward = HashMap::default();
    let mut map_backward = HashMap::default();

    for (key, val) in [
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ] {
        map_forward.insert(key.to_string(), val);
        map_backward.insert(key.chars().rev().collect::<String>(), val);
    }

    lines
        .iter()
        .map(|line| {
            let mut poses_forw = HashMap::default();
            let mut poses_back = HashMap::default();
            let line_back = line.chars().rev().collect::<String>();
            for key in map_forward.keys() {
                poses_forw.insert(key, line.find(key));
            }
            for key in map_backward.keys() {
                poses_back.insert(key, line_back.find(key));
            }

            let mut min_forw_key = "".to_owned();
            let mut min_forw_val = line.len() + 100;
            let mut min_back_key = "".to_owned();
            let mut min_back_val = line.len() + 100;

            for (key, val) in &poses_forw {
                if val.is_some() && val.unwrap() < min_forw_val {
                    min_forw_key = (*key).clone();
                    min_forw_val = val.unwrap();
                }
            }
            for (key, val) in &poses_back {
                if val.is_some() && val.unwrap() < min_back_val {
                    min_back_key = (*key).clone();
                    min_back_val = val.unwrap();
                }
            }
            let num1 = map_forward.get(&min_forw_key).unwrap();
            let num2 = map_backward.get(&min_back_key).unwrap();

            //
            num1 * 10 + num2
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
                "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )),
            281
        );
    }
}
