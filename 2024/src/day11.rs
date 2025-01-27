use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::common::int_len;
use aoc_utils::common::HashMap;

type Num = u128;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Num> {
    input
        .split(" ")
        .map(|n| n.parse::<Num>().unwrap())
        .collect()
}

#[aoc(day11, part1)]
fn part1(stones: &[Num]) -> Num {
    blink(stones, 25)
}
fn blink(nums: &[Num], depth: u8) -> Num {
    let mut cache = HashMap::default();
    nums.iter()
        .map(|n| blink_helper(*n, 0, depth, &mut cache))
        .sum()
}

fn blink_helper(num: Num, depth: u8, target_depth: u8, cache: &mut HashMap<(Num, u8), Num>) -> Num {
    if depth == target_depth {
        return 1;
    }
    if cache.contains_key(&(num, depth)) {
        return cache[&(num, depth)];
    }
    let num_len = int_len(num as f64);
    let result = if num == 0 {
        blink_helper(1, depth + 1, target_depth, cache)
    } else if num_len % 2 == 0 {
        let half_val = 10u128.pow(num_len / 2) as Num;
        blink_helper(num / half_val, depth + 1, target_depth, cache)
            + blink_helper(num % half_val, depth + 1, target_depth, cache)
    } else {
        blink_helper(num * 2024, depth + 1, target_depth, cache)
    };

    cache.insert((num, depth), result);
    result
}

#[aoc(day11, part2)]
fn part2(stones: &[Num]) -> Num {
    blink(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_example() {
        assert_eq!(part1(&parse("125 17")), 55312);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("125 17")), 65601038650482);
    }
}
