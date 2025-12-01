use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum DIR {
    L,
    R,
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(DIR, i32)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let d_raw = chars.next().unwrap();
            let num_raw = chars.collect::<String>();
            (
                match d_raw {
                    'L' => DIR::L,
                    'R' => DIR::R,
                    _ => panic!("Invalid direction"),
                },
                num_raw.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(DIR, i32)]) -> u32 {
    let mut pos = 50;
    let mut count = 0;
    for (d, n) in input {
        match &d {
            DIR::L => pos -= n,
            DIR::R => pos += n,
        }
        while pos < 0 {
            pos += 100;
        }
        pos %= 100;

        if pos == 0 {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part2)]
fn part2(input: &[(DIR, i32)]) -> i32 {
    let mut pos = 50;
    let mut count = 0;

    for (d, n) in input {
        let pp = pos;
        count += match &d {
            DIR::R => {
                pos += n;
                let c = pos / 100;
                pos = pos.rem_euclid(100);
                c
            }
            DIR::L => {
                pos = pos - n;
                let mut c = (pos / 100).abs();
                if pos <= 0 && pp != 0 {
                    c += 1;
                }
                pos = pos.rem_euclid(100);
                c
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_STR: &str = indoc! { "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"
    };
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_STR)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_STR)), 6);
    }
}
