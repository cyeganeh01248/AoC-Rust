use aoc_runner_derive::{aoc, aoc_generator};

use num_bigint::BigInt;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
struct Machine {
    A: BigInt,
    B: BigInt,
    C: BigInt,

    P: Vec<u8>,
    I: isize,
    O: Vec<String>,
}

impl Machine {
    fn calc_combo(&self, combo: u8) -> BigInt {
        match combo {
            4 => self.A.clone(),
            5 => self.B.clone(),
            6 => self.C.clone(),
            7 => unreachable!("Reserved combo"),
            n if n < 4 => BigInt::from(n),
            _ => unreachable!(),
        }
    }
    fn step(&mut self) -> bool {
        if self.I >= self.P.len() as isize {
            return false;
        }
        let opcode = self.P[self.I as usize];
        let operand = self.P[self.I as usize + 1];

        match opcode {
            0 => {
                // ADV combo
                self.A = self.A.clone()
                    / BigInt::from(2).pow(self.calc_combo(operand).try_into().unwrap());
            }
            1 => {
                // BXL literal
                self.B = self.B.clone() ^ BigInt::from(operand);
            }
            2 => {
                // BST combo
                self.B = self.calc_combo(operand) & BigInt::from(0b111u8);
            }
            3 => {
                // JNZ literal
                if self.A != BigInt::ZERO {
                    self.I = operand as isize - 2;
                }
            }
            4 => {
                // BXC none
                self.B = self.B.clone() ^ self.C.clone();
            }
            5 => {
                // OUT combo
                self.O.push(format!(
                    "{}",
                    self.calc_combo(operand) & BigInt::from(0b111u8)
                ));
            }
            6 => {
                // BDV combo
                self.B = self.A.clone()
                    / BigInt::from(2).pow(self.calc_combo(operand).try_into().unwrap());
            }
            7 => {
                // CDV combo
                self.C = self.A.clone()
                    / BigInt::from(2).pow(self.calc_combo(operand).try_into().unwrap());
            }
            _ => unreachable!(),
        }
        self.I += 2;
        true
    }
    fn step_till_end(&mut self) -> String {
        while self.step() {}
        self.output()
    }
    fn output(&self) -> String {
        self.O.join(",")
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> (Machine, String) {
    let lines = input.lines().collect::<Vec<_>>();

    let a = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<BigInt>()
        .unwrap();

    let b = lines[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<BigInt>()
        .unwrap();

    let c = lines[2]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<BigInt>()
        .unwrap();

    let program = lines[4]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    (
        Machine {
            A: a,
            B: b,
            C: c,
            P: program,
            I: 0,
            O: vec![],
        },
        lines[4].split(": ").nth(1).unwrap().to_string(),
    )
}

#[aoc(day17, part1)]
fn part1((input, _): &(Machine, String)) -> String {
    let mut machine = input.to_owned();
    machine.step_till_end()
}

#[aoc(day17, part2)]
fn part2((input, program): &(Machine, String)) -> BigInt {
    let expected = program.split(",").map(|x| x.parse().unwrap()).collect();
    find_solution(input, &expected, BigInt::from(0), 1).unwrap()
}

fn find_solution(base_machine: &Machine, program: &Vec<u8>, a: BigInt, n: usize) -> Option<BigInt> {
    if n > program.len() {
        return Some(a);
    }
    for i in 0..8 {
        let a2 = a.clone() * BigInt::from(8) + BigInt::from(i);
        let mut machine = base_machine.clone();
        machine.A = a2.clone();
        let out = machine
            .step_till_end()
            .split(",")
            .map(|i| i.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let target = program.clone()[(program.len() - n)..].to_vec();
        if out == target {
            let res = find_solution(base_machine, program, a2, n + 1);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0\
            "
            )),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0\
            "
            )),
            BigInt::from(117440)
        );
    }
}
