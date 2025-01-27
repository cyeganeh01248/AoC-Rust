// #![allow(clippy::ptr_arg)]
use aoc_runner_derive::{aoc, aoc_generator};

type Num = i64;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Num> {
    let mut file_system = vec![];
    let mut file = true;
    let mut id = 0;
    for n in input.chars() {
        if let Some(n_int) = n.to_digit(10) {
            if file {
                for _i in 0..n_int {
                    file_system.push(id);
                }
                id += 1;
            } else {
                for _i in 0..n_int {
                    file_system.push(-1);
                }
            }
            file = !file;
        };
    }
    file_system
}

fn calculate_checksum(disk: &[Num]) -> Num {
    disk.iter()
        .enumerate()
        .map(|(i, c)| if *c == -1 { 0 } else { c * i as Num })
        .sum()
}

#[aoc(day9, part1)]
fn part1(disk: &[Num]) -> Num {
    let mut disk = disk.to_owned();
    let mut empty_pos = 0;
    for (i, c) in disk.clone().iter().rev().enumerate() {
        let pos = disk.len() - i - 1;

        if *c == -1 {
            continue;
        }
        loop {
            if disk[empty_pos] != -1 {
                empty_pos += 1;
            } else {
                break;
            }
        }
        if pos <= empty_pos {
            break;
        }
        disk.swap(pos, empty_pos);
    }
    calculate_checksum(&disk)
}

#[aoc(day9, part2)]
fn part2(disk: &[Num]) -> Num {
    let mut disk = disk.to_owned();
    let mut end_pos = disk.len() as isize;
    let mut file_blocks = Vec::with_capacity(disk.len());
    let mut empty_blocks = Vec::with_capacity(disk.len());
    loop {
        if end_pos - 1 < 0 {
            break;
        }
        let mut start_pos = end_pos - 1;
        while start_pos > 0 && disk[(start_pos - 1) as usize] == disk[(end_pos - 1) as usize] {
            start_pos -= 1;
        }
        if disk[start_pos as usize] == -1 {
            empty_blocks.push(((end_pos - start_pos) as usize, start_pos as usize));
        } else {
            file_blocks.push((
                disk[start_pos as usize],
                (end_pos - start_pos) as usize,
                start_pos as usize,
            ));
        }
        end_pos = start_pos;
    }
    file_blocks.sort_by(|(id1, _, _), (id2, _, _)| id1.cmp(id2));
    file_blocks.reverse();
    empty_blocks.reverse();

    for (_, file_size, file_start) in file_blocks.iter() {
        for e_i in 0..empty_blocks.len() {
            let (empty_size, empty_start) = &empty_blocks[e_i];

            if empty_start > file_start {
                continue;
            }
            if empty_size >= file_size {
                for i in 0..*file_size {
                    disk.swap(*file_start + i, *empty_start + i);
                }
                if empty_size == file_size {
                    empty_blocks.remove(e_i);
                } else {
                    empty_blocks[e_i] = (*empty_size - file_size, *empty_start + file_size);
                }
                break;
            }
        }
    }
    calculate_checksum(&disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("2333133121414131402")), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("2333133121414131402")), 2858);
    }
}
