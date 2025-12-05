use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::parsers::v_grid_no_whitespace;
use fxhash::FxHashSet;
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    v_grid_no_whitespace(input)
}

fn first_find_removable(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut search = Vec::with_capacity(input.len() * input[0].len());

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] == '.' {
                continue;
            }
            search.push((r, c));
        }
    }
    find_removeable(input, search)
}

fn find_removeable(input: &Vec<Vec<char>>, check: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut removeable = vec![];
    for (r, c) in check.into_iter() {
        let mut found_count = 0;

        for (nr, nc) in get_neighbors(r, c, input.len(), input[r].len()) {
            if input[nr][nc] == '@' {
                found_count += 1;
            }
            if found_count >= 4 {
                break;
            }
        }
        if found_count < 4 {
            removeable.push((r, c))
        }
    }
    removeable
}

fn get_neighbors(r: usize, c: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(8);

    let r = r as isize;
    let c = c as isize;
    let w = w as isize;
    let h = h as isize;

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r + dr;
            let nc = c + dc;

            if nr >= 0 && nr < h && nc >= 0 && nc < w {
                neighbors.push((nr as usize, nc as usize));
            }
        }
    }

    neighbors
}

#[aoc(day4, part1)]
fn part1(input: &Vec<Vec<char>>) -> usize {
    first_find_removable(input).len()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut input = input.clone();
    let mut count = 0;
    let mut removeable = first_find_removable(&input);
    loop {
        if removeable.len() == 0 {
            break;
        }
        count += removeable.len();
        for &(r, c) in removeable.iter() {
            input[r][c] = '.';
        }
        let mut search = FxHashSet::default();

        for &(r, c) in removeable.iter() {
            for (nr, nc) in get_neighbors(r, c, input.len(), input[r].len()) {
                if input[nr][nc] == '@' {
                    search.insert((nr, nc));
                }
            }
        }
        removeable = find_removeable(&input, search.into_iter().collect());
    }
    count
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
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 43);
    }
}
