use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut g1s = vec![];
    let mut g2s = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let g1 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let g2 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        g1s.push(g1);
        g2s.push(g2);
    }
    (g1s, g2s)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u64>, Vec<u64>)) -> String {
    let mut g1s = input.0.clone();
    let mut g2s = input.1.clone();
    g1s.sort();
    g2s.sort();

    let mut dist = 0;

    for (g1, g2) in g1s.iter().zip(g2s.iter()) {
        println!("{g1} {g2}");
        let ma = g1.max(g2);
        let mi = g1.min(g2);
        dist += ma - mi;
    }
    println!("{dist}");
    dist.to_string()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u64>, Vec<u64>)) -> String {
    let mut score = 0;
    for n1 in input.0.iter() {
        score += n1 * input.1.iter().filter(|n2| n1 == *n2).count() as u64;
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            "11"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            "31"
        );
    }
}
