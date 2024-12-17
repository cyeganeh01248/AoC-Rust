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
    while machine.step() {}
    machine.output()
}

#[aoc(day17, part2)]
fn part2((input, program): &(Machine, String)) -> BigInt {
    let program = program.to_owned().chars().collect::<Vec<_>>();
    let program_str = program.iter().collect::<String>();
    let mut a_raw = BigInt::from(0);
    let mut i = 1;
    'outer: loop {
        println!("{a_raw}");
        let a = a_raw.clone();
        let mut machine = input.to_owned();
        machine.A = a;
        while machine.step() {}
        let output = machine.output().chars().collect::<Vec<_>>();
        if output == program {
            return a_raw;
        }

        let mut matc = true;
        for j in 0..i {
            if j > output.len() {
                break;
            }
            println!(
                "{} {}",
                output[output.len() - j - 1],
                program[program.len() - j - 1]
            );
            if output[output.len() - j - 1] == program[program.len() - j - 1] {
                matc = false;
            }
        }
        if matc {
            a_raw *= 8;
            i += 1;
        } else {
            a_raw += 1;
        }
        // if output.chars().nth(output.len() - i * 2).unwrap() == program[program.len() - i * 2] {
        // a_raw *= 8;
        // i += 1;
        // } else {
        // a_raw += 1;
        // }
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
