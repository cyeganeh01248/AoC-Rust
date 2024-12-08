use aoc_runner_derive::{aoc, aoc_generator};

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
fn part1(input: &[(isize, Vec<isize>)]) -> isize {
    input
        .iter()
        .map(|(test, opers)| {
            if can_make_test_value(*test, 0, opers) {
                *test
            } else {
                0
            }
        })
        .sum()
}

fn can_make_test_value(test_value: isize, input: isize, opers: &[isize]) -> bool {
    if opers.is_empty() {
        panic!("How did you get here?")
    } else if opers.len() == 1 {
        return opers[0] + input == test_value || opers[0] * input == test_value;
    } else {
        return can_make_test_value(test_value, opers[0] + input, &opers[1..])
            || can_make_test_value(test_value, opers[0] * input, &opers[1..]);
    }
}

#[aoc(day7, part2)]
fn part2(input: &[(isize, Vec<isize>)]) -> isize {
    input
        .iter()
        .map(|(test, opers)| {
            if can_make_test_value_with_cat(*test, 0, opers) {
                *test
            } else {
                0
            }
        })
        .sum()
}

fn can_make_test_value_with_cat(test_value: isize, input: isize, opers: &[isize]) -> bool {
    if input > test_value {
        return false;
    }
    if opers.is_empty() {
        panic!("How did you get here?")
    } else if opers.len() == 1 {
        let add_oper = opers[0] + input;
        let mul_oper = opers[0] * input;
        let cat_oper = (input.to_string() + &opers[0].to_string())
            .parse::<isize>()
            .unwrap();
        return add_oper == test_value || mul_oper == test_value || cat_oper == test_value;
    } else {
        let add_oper = opers[0] + input;
        let mul_oper = opers[0] * input;
        let cat_oper = (input.to_string() + &opers[0].to_string())
            .parse::<isize>()
            .unwrap();
        return can_make_test_value_with_cat(test_value, add_oper, &opers[1..])
            || can_make_test_value_with_cat(test_value, mul_oper, &opers[1..])
            || can_make_test_value_with_cat(test_value, cat_oper, &opers[1..]);
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
            11387
        );
    }
}
