use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i64]) -> bool {
    let mut all_ascending = true;
    let mut all_descending = true;
    let mut all_within_1_3 = true;

    for i in 1..report.len() {
        if report[i] < report[i - 1] {
            all_ascending = false;
        }
        if report[i] > report[i - 1] {
            all_descending = false;
        }
        if (report[i] - report[i - 1]).abs() > 3 || (report[i] - report[i - 1]).abs() < 1 {
            all_within_1_3 = false;
        }
    }
    (all_ascending || all_descending) && all_within_1_3
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|report| if is_safe(report) { 1 } else { 0 })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|report| {
            for skip in 0..=report.len() {
                let mut new_report = report.clone();
                if skip != report.len() {
                    new_report.remove(skip);
                }
                if is_safe(&new_report) {
                    return 1;
                }
            }
            0
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
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            4
        );
    }
}
