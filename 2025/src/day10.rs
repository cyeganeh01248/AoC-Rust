use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    const EXAMPLE_INPUT: &str = indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), "<RESULT>");
    }
}
