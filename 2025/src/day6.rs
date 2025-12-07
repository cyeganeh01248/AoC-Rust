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
    let nums = input[0..(input.len() - 1)]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<u128>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let signs = input[input.len() - 1]
        .split_whitespace()
        .map(|op| match op {
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

pub fn rotate_90_ccw<T: Clone>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if grid.is_empty() {
        return grid;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut rotated = vec![vec![]; cols];

    for col in (0..cols).rev() {
        let mut new_row = Vec::with_capacity(rows);
        for row in 0..rows {
            new_row.push(grid[row][col].clone());
        }
        rotated[cols - 1 - col] = new_row;
    }

    rotated
}

#[aoc(day6, part2)]
fn part2(input: &Vec<String>) -> u128 {
    let input = input
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let input = rotate_90_ccw(input);

    let rows = input
        .iter()
        .map(|n| n.iter().filter(|&&c| c != ' ').collect::<String>())
        .collect::<Vec<_>>();

    let num_problems = rows.iter().filter(|n| n.is_empty()).count() + 1;

    let mut signs = Vec::with_capacity(num_problems);
    let mut results = Vec::with_capacity(num_problems);
    let mut nums = Vec::with_capacity(num_problems);
    nums.push(Vec::new());

    let mut p = 0;
    for row in &rows {
        if row.is_empty() {
            p += 1;
            nums.push(Vec::new());
            continue;
        }
        if row.contains("+") {
            signs.push(Op::Add);
            results.push(0);
            nums[p].push(row[0..(row.len() - 1)].parse::<u128>().unwrap());
        } else if row.contains("*") {
            signs.push(Op::Mul);
            results.push(1);
            nums[p].push(row[0..(row.len() - 1)].parse::<u128>().unwrap());
        } else {
            nums[p].push(row.parse().unwrap());
        }
    }

    nums.iter()
        .enumerate()
        .map(|(r, row)| match signs[r] {
            Op::Add => row.iter().sum::<u128>(),
            Op::Mul => row.iter().product::<u128>(),
        })
        .sum()
}

#[cfg(test)]
    #[rustfmt::skip]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 3263827);
    }
}
