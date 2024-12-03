use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string().replace("\n", "")
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    let caps = re.captures_iter(input);
    for cap in caps {
        let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let c = a * b;
        sum += c;
    }
    sum
}

#[aoc(day3, part2)]
fn part2(mut input: &str) -> i64 {
    let mut mode_enabled = true;
    let re_mul = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    while !input.is_empty() {
        if mode_enabled {
            if input.starts_with("don't()") {
                mode_enabled = false;
                input = &input[7..];
            } else if let Some(cap) = re_mul.captures(input) {
                let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let c = a * b;
                sum += c;
                let l = cap.len();
                input = &input[l..];
            } else {
                input = &input[1..];
            }
        } else if input.starts_with("do()") {
            mode_enabled = true;
            input = &input[4..];
        } else {
            input = &input[1..];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
