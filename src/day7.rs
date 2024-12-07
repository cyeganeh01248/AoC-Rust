use aoc_runner_derive::{aoc, aoc_generator};
use num_bigint::BigInt as Integer;

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(isize, Vec<isize>)> {
    let grid = input
        .lines()
        .map(|line| {
            let mut vals = vec![];
            let mut split = line.split(": ");
            let test = split.next().unwrap().parse().unwrap();
            let opers = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<isize>>();
            vals.extend(opers);
            (test, vals)
            // let v =
        })
        .collect();
    grid
}

#[aoc(day7, part1)]
fn part1(input: &Vec<(isize, Vec<isize>)>) -> isize {
    input
        .iter()
        .map(|(test, opers)| {
            if can_make_test_value(*test, 0, opers) {
                return *test;
            } else {
                return 0;
            }
        })
        .sum()
}

fn can_make_test_value(test_value: isize, input: isize, opers: &[isize]) -> bool {
    if opers.len() == 0 {
        panic!("How did you get here?")
    } else if opers.len() == 1 {
        return opers[0] + input == test_value || opers[0] * input == test_value;
    } else {
        return can_make_test_value(test_value, opers[0] + input, &opers[1..])
            || can_make_test_value(test_value, opers[0] * input, &opers[1..]);
    }
}

#[aoc(day7, part2)]
fn part2(input: &Vec<(isize, Vec<isize>)>) -> Integer {
    input
        .iter()
        .map(|(test, opers)| {
            if can_make_test_value_with_cat(
                Integer::from(*test),
                Integer::from(0),
                &opers
                    .iter()
                    .map(|x| Integer::from(*x))
                    .collect::<Vec<Integer>>(),
            ) {
                return Integer::from(*test);
            } else {
                return Integer::from(0);
            }
        })
        .sum()
}

fn can_make_test_value_with_cat(test_value: Integer, input: Integer, opers: &[Integer]) -> bool {
    if opers.len() == 0 {
        panic!("How did you get here?")
    } else if opers.len() == 1 {
        let add_oper = opers[0].clone() + input.clone();
        let mul_oper = opers[0].clone() * input.clone();
        let cat_oper = (input.clone().to_string() + &opers[0].clone().to_string())
            .parse::<Integer>()
            .unwrap();
        return add_oper == test_value.clone()
            || mul_oper == test_value.clone()
            || cat_oper == test_value.clone();
    } else {
        let add_oper = opers[0].clone() + input.clone();
        let mul_oper = opers[0].clone() * input.clone();
        let cat_oper = (input.to_string() + &opers[0].to_string())
            .parse::<Integer>()
            .unwrap();
        return can_make_test_value_with_cat(test_value.clone(), add_oper, &opers[1..])
            || can_make_test_value_with_cat(test_value.clone(), mul_oper, &opers[1..])
            || can_make_test_value_with_cat(test_value.clone(), cat_oper, &opers[1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\
            "
            )),
            3749
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20\
            "
            )),
            Integer::from(11387)
        );
    }
}
