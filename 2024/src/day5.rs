use crate::common::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

use crate::common::MyMatrix;

type Num = u16;

#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<Num, Vec<Num>>, MyMatrix<Num>) {
    let mut keys = HashMap::default();
    let lines = input.lines().collect::<Vec<_>>();
    let mut i = 0;
    loop {
        let line = lines[i];
        if line.is_empty() {
            break;
        }
        let mut parts = line.split("|");
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();

        keys.entry(a)
            .and_modify(|li: &mut Vec<Num>| {
                li.push(b);
                li.sort()
            })
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
        let page: Vec<Num> = line.split(",").map(|s| s.parse().unwrap()).collect();
        pages.push(page);
        i += 1;
    }
    (keys, pages)
}

#[aoc(day5, part1)]
fn part1((keys, pages): &(HashMap<Num, Vec<Num>>, MyMatrix<Num>)) -> Num {
    pages
        .iter()
        .map(|page| {
            // let mut page = page.clone();
            // let page_og = page.clone();
            for i in 0..(page.len() - 1) {
                if match sort_fn(&page[i], &page[i + 1], keys) {
                    Ordering::Less => false,
                    Ordering::Equal => false,
                    Ordering::Greater => true,
                } {
                    return 0;
                }
            }
            page[page.len() / 2]
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2((keys, pages): &(HashMap<Num, Vec<Num>>, MyMatrix<Num>)) -> Num {
    pages
        .iter()
        .map(|page| {
            let mut sorted = true;
            for i in 0..(page.len() - 1) {
                sorted = sorted
                    && match sort_fn(&page[i], &page[i + 1], keys) {
                        Ordering::Less => true,
                        Ordering::Equal => true,
                        Ordering::Greater => false,
                    };
            }
            if sorted {
                return 0;
            }
            let mut page = page.clone();
            page.sort_by(|a, b| sort_fn(a, b, keys));
            page[page.len() / 2]
        })
        .sum()
}

fn sort_fn(a: &Num, b: &Num, keys: &HashMap<Num, Vec<Num>>) -> Ordering {
    if let Some(a_list) = keys.get(a) {
        if a_list.contains(b) {
            return Ordering::Less;
        }
    }
    if let Some(b_list) = keys.get(b) {
        if b_list.contains(a) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
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
        assert_eq!(
            part2(&parse(
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
            123
        );
    }
}
