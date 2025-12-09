use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[aoc_generator(day7)]
fn parse(input: &str) -> (usize, HashMap<usize, Vec<usize>>, usize) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut out = HashMap::default();
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '^' {
                let v: &mut Vec<usize> = out.entry(row).or_default();
                v.push(col);
            }
        }
    }
    (lines[0].find('S').unwrap(), out, lines.len())
}

#[aoc(day7, part1)]
fn part1((start, input, height): &(usize, HashMap<usize, Vec<usize>>, usize)) -> u64 {
    let mut cols_to_check = HashSet::default();
    cols_to_check.insert(*start);
    let mut r = 0;
    let mut count = 0;
    while r < *height {
        let row = input.get(&r);
        if row.is_none() {
            r += 1;
            continue;
        }
        let row = row.unwrap();
        let mut new_cols = cols_to_check.clone();
        for col in cols_to_check.iter() {
            if row.contains(col) {
                println!("Colision at {r} {col}");
                count += 1;
                new_cols.remove(&col);
                new_cols.insert(col + 1);
                new_cols.insert(col - 1);
            }
        }
        cols_to_check = new_cols;
        r += 1;
    }
    count
}

#[aoc(day7, part2)]
fn part2((start, input, height): &(usize, HashMap<usize, Vec<usize>>, usize)) -> u64 {
    let mut cols_to_check = HashMap::default();
    cols_to_check.insert(*start, 1);
    let mut r = 0;
    while r < *height {
        let row = input.get(&r);
        if row.is_none() {
            r += 1;
            continue;
        }
        let row = row.unwrap();
        let mut new_cols = cols_to_check.clone();
        for col in cols_to_check.keys() {
            if row.contains(col) {
                let cur = *new_cols.get(&col).unwrap();
                *new_cols.entry(col + 1).or_insert(0) += cur;
                *new_cols.entry(col - 1).or_insert(0) += cur;
                new_cols.remove(&col);
            }
        }
        cols_to_check = new_cols;
        r += 1;
    }
    cols_to_check.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ..............."};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 21);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(
            part1(&parse(indoc! {"
            ....S....
            .........
            ....^....
            ...^.^...
            .........
            ........."
            })),
            3
        );
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(
            part2(&parse(indoc! {"
            ....S....
            .........
            ....^....
            ...^.^...
            .........
            ........."
            })),
            4
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 40);
    }
}
