use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ids = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && line != ""
    {
        let mut vals = line.split("-").map(|n| n.parse().unwrap());
        ranges.push((vals.next().unwrap(), vals.next().unwrap()));
    }

    while let Some(id) = lines.next()
        && id != ""
    {
        ids.push(id.parse().unwrap());
    }

    (ranges, ids)
}

#[aoc(day5, part1)]
fn part1((ranges, ids): &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    ids.into_iter()
        .filter(|&id| {
            ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(id))
        })
        .count()
}

#[aoc(day5, part2)]
fn part2((ranges, _): &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let mut ranges = ranges.clone();
    loop {
        let mut found = false;

        let mut i = 0;
        let mut j = 1;
        while i < ranges.len() - 1 {
            let r1_s = ranges[i].0;
            let r1_e = ranges[i].1;
            let r2_s = ranges[j].0;
            let r2_e = ranges[j].1;

            if (r1_s >= r2_s && r1_s <= r2_e)
                || (r1_e >= r2_s && r1_e <= r2_e)
                || (r2_s >= r1_s && r2_s <= r1_e)
                || (r2_e >= r1_s && r2_e <= r1_e)
            {
                found = true;
                ranges[i] = (r1_s.min(r2_s), r1_e.max(r2_e));
                ranges.remove(j);
                j -= 1;
            }
            j += 1;
            if j >= ranges.len() {
                i += 1;
                j = i + 1;
            }
        }

        if !found {
            break;
        }
    }
    ranges.into_iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 14);
    }
}
