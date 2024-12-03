use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.replace("\n", "")
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    let mut mode_enabled = true;
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let t = re_mul
        .captures_iter(input)
        .map(|cap| match cap.get(0).unwrap().as_str() {
            "do()" => {
                mode_enabled = true;
                0
            }
            "don't()" => {
                mode_enabled = false;
                0
            }
            _ => {
                let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                if mode_enabled {
                    a * b
                } else {
                    0
                }
            }
        })
        .sum();
    t
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
