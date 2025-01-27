use aoc_runner_derive::{aoc, aoc_generator};
use realistic::Real;
type Num = Real;

#[derive(Debug, Clone)]
struct Machine {
    a_button: (Num, Num),
    b_button: (Num, Num),
    prize: (Num, Num),
}
#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Machine> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut machines = vec![];
    let button_a = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut i = 0;
    while i < lines.len() {
        let l1 = lines[i].trim();
        let l2 = lines[i + 1].trim();
        let l3 = lines[i + 2].trim();

        let c1 = button_a.captures(l1).unwrap();
        let c2 = button_b.captures(l2).unwrap();
        let c3 = prize.captures(l3).unwrap();

        for c in [&c1, &c2, &c3] {
            assert!(c.len() == 3);
        }

        let machine = Machine {
            a_button: (
                c1.get(1).unwrap().as_str().parse().unwrap(),
                c1.get(2).unwrap().as_str().parse().unwrap(),
            ),
            b_button: (
                c2.get(1).unwrap().as_str().parse().unwrap(),
                c2.get(2).unwrap().as_str().parse().unwrap(),
            ),
            prize: (
                c3.get(1).unwrap().as_str().parse().unwrap(),
                c3.get(2).unwrap().as_str().parse().unwrap(),
            ),
        };
        machines.push(machine);
        i += 4;
    }
    machines
}

#[allow(non_snake_case)]
#[aoc(day13, part1)]
fn part1(machines: &[Machine]) -> Num {
    let mut sum = Num::from(0);
    for machine in machines.iter() {
        let AXinc = machine.a_button.0.clone();
        let AYinc = machine.a_button.1.clone();
        let BXinc = machine.b_button.0.clone();
        let BYinc = machine.b_button.1.clone();
        let PX = machine.prize.0.clone();
        let PY = machine.prize.1.clone();

        let b = ((PY.clone() - (PX.clone() * AYinc.clone() / AXinc.clone()).unwrap())
            / (BYinc.clone() - (BXinc.clone() * AYinc.clone() / AXinc.clone()).unwrap()))
        .unwrap();
        let a = ((PX.clone() - (b.clone() * BXinc.clone())) / AXinc.clone()).unwrap();

        if a.is_whole() && b.is_whole() {
            sum = sum + a * Num::from(3) + b;
        }
    }
    sum
}

#[allow(non_snake_case)]
#[aoc(day13, part2)]
fn part2(machines: &[Machine]) -> Num {
    let mut sum = Num::from(0);
    for machine in machines.iter() {
        let AXinc = machine.a_button.0.clone();
        let AYinc = machine.a_button.1.clone();
        let BXinc = machine.b_button.0.clone();
        let BYinc = machine.b_button.1.clone();
        let PX = machine.prize.0.clone() + Num::from(10000000000000i64);
        let PY = machine.prize.1.clone() + Num::from(10000000000000i64);

        let b = ((PY.clone() - (PX.clone() * AYinc.clone() / AXinc.clone()).unwrap())
            / (BYinc.clone() - (BXinc.clone() * AYinc.clone() / AXinc.clone()).unwrap()))
        .unwrap();
        let a = ((PX.clone() - (b.clone() * BXinc.clone())) / AXinc.clone()).unwrap();

        if a.is_whole() && b.is_whole() {
            sum = sum + a * Num::from(3) + b;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
                  "
            )),
            Num::from(480)
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
                  "
            )),
            Num::from(875318608908i64)
        );
    }
}
