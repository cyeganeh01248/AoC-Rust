use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::parsers::v_grid_no_whitespace;
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<String>> {
    v_grid_no_whitespace(input)
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<String>>) -> u32 {
    let mut count = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] == "." {
                continue;
            }
            let mut found_count = 0;
            for i in [-1isize, 0, 1] {
                for j in [-1isize, 0, 1] {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let nr = r as isize + i;
                    let nc = c as isize + j;

                    if nr < input.len() as isize
                        && nr >= 0
                        && nc < input[nr as usize].len() as isize
                        && nc >= 0
                    {
                        if input[nr as usize][nc as usize] == "@" {
                            found_count += 1;
                        }
                    }
                    if found_count >= 4 {
                        break;
                    }
                }
                if found_count >= 4 {
                    break;
                }
            }

            if found_count < 4 {
                println!("{} {}", r, c);
                count += 1;
            }
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Vec<String>>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 0);
    }
}
