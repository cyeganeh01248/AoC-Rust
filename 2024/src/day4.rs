use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{common::MyMatrix, parsers::v_grid_no_whitespace};

type Num = u16;

#[aoc_generator(day4)]
fn parse(input: &str) -> MyMatrix<char> {
    v_grid_no_whitespace(input)
}

#[aoc(day4, part1)]
fn part1(input: &MyMatrix<char>) -> Num {
    let mut count = 0;
    for row in 0..(input.len() as isize) {
        for col in 0..(input[row as usize].len() as isize) {
            for (rowi, coli) in [(0, 1), (1, 1), (1, 0), (1, -1)] {
                let mut word = String::new();
                word.push(input[row as usize][col as usize]);
                for i in 1..4 {
                    let rp = row + rowi * i;
                    let cp = col + coli * i;
                    if rp < 0
                        || rp >= input.len() as isize
                        || cp < 0
                        || cp >= input[row as usize].len() as isize
                    {
                        break;
                    }
                    word.push(input[rp as usize][cp as usize]);
                }
                if word.len() < 4 {
                    continue;
                }
                if word == "XMAS" || word == "SAMX" {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(input: &MyMatrix<char>) -> Num {
    let mut count = 0;
    for row in 1..(input.len() as isize - 1) {
        for col in 1..(input[row as usize].len() as isize - 1) {
            if input[row as usize][col as usize] != 'A' {
                continue;
            }
            let mut word = String::new();
            for (rowi, coli) in [(-1, -1), (0, 0), (1, 1), (-1, 1), (1, -1)] {
                let rp = row + rowi;
                let cp = col + coli;
                if rp < 0 || rp >= input.len() as isize {
                    break;
                }
                if cp < 0 || cp >= input[row as usize].len() as isize {
                    break;
                }
                word.push(input[rp as usize][cp as usize]);
            }
            if word.len() < 3 {
                continue;
            }
            if ["MASMS", "MASSM", "SAMSM", "SAMMS"].contains(&word.as_str()) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            18
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            9
        );
    }
}
