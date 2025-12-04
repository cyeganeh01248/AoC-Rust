use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::parsers::v_grid_no_whitespace;
use fxhash::FxHashMap as HashMap;
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<u128>> {
    v_grid_no_whitespace(input)
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Vec<u128>>) -> u128 {
    input
        .iter()
        .map(|bank| {
            let mut max_1 = 0;
            let mut max_2 = 0;
            for i in 0..(bank.len() - 1) {
                if bank[i] > max_1 {
                    max_1 = bank[i];
                    max_2 = bank[i + 1];
                } else {
                    max_2 = max_2.max(bank[i]);
                }
            }
            if bank[bank.len() - 1] > max_2 {
                max_2 = bank[bank.len() - 1];
            }
            max_1 * 10 + max_2
        })
        .sum()
}

const NUM_BATTERIES_TO_SELECT: usize = 12;

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<u128>>) -> u128 {
    input
        .iter()
        .map(|bank| calculate_max_joltage_for_bank(bank))
        .sum()
}

fn calculate_max_joltage_for_bank(bank_digits: &Vec<u128>) -> u128 {
    let mut memo = HashMap::default();

    fn find_max_recursive(
        current_idx: usize,
        digits_remaining_to_select: usize,
        bank: &[u128],
        memo: &mut HashMap<(usize, usize), Option<String>>,
    ) -> Option<String> {
        if let Some(cached_result) = memo.get(&(current_idx, digits_remaining_to_select)) {
            return cached_result.clone();
        }

        if digits_remaining_to_select == 0 {
            return Some("".to_string());
        }

        if current_idx == bank.len() {
            return None;
        }

        let remaining_chars_count = bank.len() - current_idx;
        if remaining_chars_count < digits_remaining_to_select {
            return None;
        }

        let mut best_result = None;

        let current_digit_val = bank[current_idx];
        let suffix_if_current_included =
            find_max_recursive(current_idx + 1, digits_remaining_to_select - 1, bank, memo);

        if let Some(suffix) = suffix_if_current_included {
            let current_joltage_str = format!("{}{}", current_digit_val, suffix);
            best_result = Some(current_joltage_str);
        }

        if remaining_chars_count > digits_remaining_to_select {
            let result_if_current_excluded =
                find_max_recursive(current_idx + 1, digits_remaining_to_select, bank, memo);

            if let Some(excluded_joltage_str) = result_if_current_excluded {
                if let Some(current_best_joltage_str) = &best_result {
                    if excluded_joltage_str > *current_best_joltage_str {
                        best_result = Some(excluded_joltage_str);
                    }
                } else {
                    best_result = Some(excluded_joltage_str);
                }
            }
        }

        memo.insert(
            (current_idx, digits_remaining_to_select),
            best_result.clone(),
        );
        best_result
    }

    let max_joltage_str = find_max_recursive(0, NUM_BATTERIES_TO_SELECT, bank_digits, &mut memo)
        .expect("Should always find a valid 12-digit number for a given bank");

    max_joltage_str
        .parse::<u128>()
        .expect("Failed to parse joltage string to u128")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! { "
        987654321111111
        811111111111119
        234234234234278
        818181911112111"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 3121910778619);
    }
}
