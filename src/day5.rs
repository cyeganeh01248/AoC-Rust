use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut keys = HashMap::new();
    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;
    loop {
        let line = lines[i];
        if line == "" {
            break;
        }
        let mut parts = line.split("|");
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();

        keys.entry(a)
            .and_modify(|li: &mut Vec<i64>| li.push(b))
            .or_insert(vec![b]);
        i += 1;
    }
    i += 1;

    let mut pages = vec![];

    loop {
        if i >= lines.len() {
            break;
        }
        let line = lines[i];
        let mut page: Vec<i64> = line.split(",").map(|s| s.parse().unwrap()).collect();
        page.sort_unstable();
        pages.push(page);
        i += 1;
    }
    (keys, pages)
}

#[aoc(day5, part1)]
fn part1((keys, pages): &(HashMap<i64, Vec<i64>>, Vec<Vec<i64>>)) -> i64 {
    println!("{keys:?}");
    println!("{pages:?}");
    let mut count = 0;
    for page in pages {
        let mut add = true;
        println!("{page:?}");
        for i in 0..(page.len() - 1) {
            for j in (i + 1)..(page.len()) {
                let a = page[i];
                let b = page[j];

                if let Some(li) = keys.get(&b) {
                    if li.contains(&a) {
                        add = false;
                        // break;
                    }
                }
                println!("{add} {a} {b} {:?} {:?}", keys.get(&a), keys.get(&b))
            }
            if !add {
                break;
            }
        }
        if add {
            count += 1;
        }
    }
    println!("{count}");
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &(HashMap<i64, Vec<i64>>, Vec<Vec<i64>>)) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            )),
            143
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
