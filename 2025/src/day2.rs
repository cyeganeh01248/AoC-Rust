use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::common::ilen;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .replace("\n", "")
        .split(",")
        .map(|range| {
            let t = range
                .split("-")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();
            (t[0], t[1])
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for &(start, end) in input.iter() {
        for i in start..=end {
            let l = ilen(i);
            if l % 2 != 0 {
                continue;
            }
            if i / 10i64.pow(l / 2) == i % 10i64.pow(l / 2) {
                sum += i;
            }
        }
    }
    sum
}

#[aoc(day2, part2)]
fn part2(input: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for &(start, end) in input.iter() {
        for n in start..=end {
            let n_s = n.to_string();
            let len = n_s.len();

            for partion_width in 1..len {
                if len % partion_width != 0 {
                    continue;
                }
                if is_repeated_sequence(&n_s, partion_width) {
                    // println!("{n}");
                    sum += n;
                    break;
                }
            }
        }
    }
    sum
}

fn is_repeated_sequence(n_str: &str, partion_width: usize) -> bool {
    let len = n_str.len();
    if !len.is_multiple_of(partion_width) {
        return false;
    }

    let first_partition = &n_str[0..partion_width];

    for i in 1..(len / partion_width) {
        let start = i * partion_width;
        let end = (i + 1) * partion_width;
        let current_partition = &n_str[start..end];
        if current_partition != first_partition {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_STR: &str = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
        "
    };

    #[test]
    fn part1_example() {
        println!("{EXAMPLE_STR:?}");
        assert_eq!(part1(&parse(EXAMPLE_STR)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR)), 4174379265);
    }
}
