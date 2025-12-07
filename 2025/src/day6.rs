use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<String>) -> u128 {
    let input = input
        .clone()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(String::from)
                .filter(|n| n.len() > 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let nums = input[0..(input.len() - 1)]
        .iter()
        .map(|row| {
            row.iter()
                .map(|d| d.parse::<u128>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let signs = input[input.len() - 1]
        .iter()
        .map(|op| match op.as_str() {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!("Invalid operator"),
        })
        .collect::<Vec<_>>();
    let mut results = signs
        .iter()
        .map(|op| match op {
            &Op::Add => 0,
            &Op::Mul => 1,
        })
        .collect::<Vec<_>>();
    for r in 0..nums.len() {
        let row = &nums[r];
        for c in 0..row.len() {
            match signs[c] {
                Op::Add => results[c] += row[c],
                Op::Mul => results[c] *= row[c],
            }
        }
    }
    results.iter().sum()
}

// |123 328  51 64
// | 45 64  387 23
// |  6 98  215 314
// *   +   *   +

// The rightmost problem is 4 + 431 + 623 = 1058
// The second problem from the right is 175 * 581 * 32 = 3253600
// The third problem from the right is 8 + 248 + 369 = 625
// Finally, the leftmost problem is 356 * 24 * 1 = 8544

#[aoc(day6, part2)]
fn part2(input: &[String]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +  "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), "<RESULT>");
    }
}
