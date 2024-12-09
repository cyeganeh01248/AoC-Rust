use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::parsers::v_grid_no_whitespace;

type Num = u16;

#[aoc_generator(day8)]
fn parse(input: &str) -> (HashMap<char, Vec<(isize, isize)>>, usize, usize) {
    let grid: Vec<Vec<char>> = v_grid_no_whitespace(input);
    let mut antennas = HashMap::new();
    for (row, row_chars) in grid.iter().enumerate() {
        for (col, code) in row_chars.iter().enumerate() {
            if *code != '.' {
                antennas
                    .entry(*code)
                    .and_modify(|li: &mut Vec<(isize, isize)>| {
                        li.push((row as isize, col as isize));
                    })
                    .or_insert(vec![(row as isize, col as isize)]);
            }
        }
    }
    (antennas, grid.len(), grid[0].len())
}

#[aoc(day8, part1)]
fn part1((input, width, height): &(HashMap<char, Vec<(isize, isize)>>, usize, usize)) -> Num {
    let mut global_pts = HashSet::with_capacity(input.len() * input.len());

    for code in input.keys() {
        let ants = input.get(code).unwrap();
        for ant_a_i in 0..(ants.len() - 1) {
            for ant_b_i in (ant_a_i + 1)..ants.len() {
                let ant_a = ants[ant_a_i];
                let ant_b = ants[ant_b_i];
                let slope = ((ant_b.0 - ant_a.0), (ant_b.1 - ant_a.1));

                let mut pts = HashSet::new();
                pts.insert((ant_a.0 - slope.0, ant_a.1 - slope.1));
                pts.insert((ant_b.0 + slope.0, ant_b.1 + slope.1));
                for (pt_r, pt_c) in pts {
                    if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                        continue;
                    }
                    // println!("({}, {})", pt_r, pt_c);
                    global_pts.insert((pt_r as usize, pt_c as usize));
                }
            }
        }
    }
    global_pts.len() as Num
}

#[aoc(day8, part2)]
fn part2((input, width, height): &(HashMap<char, Vec<(isize, isize)>>, usize, usize)) -> Num {
    let mut global_pts = HashSet::with_capacity(input.len() * input.len());
    for code in input.keys() {
        let ants = input.get(code).unwrap();
        for ant_a_i in 0..(ants.len() - 1) {
            for ant_b_i in (ant_a_i + 1)..ants.len() {
                let ant_a = ants[ant_a_i];
                let ant_b = ants[ant_b_i];
                let slope = ((ant_b.0 - ant_a.0), (ant_b.1 - ant_a.1));

                let (mut pt_r, mut pt_c) = (ant_a.0, ant_a.1);
                loop {
                    if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                        break;
                    }
                    global_pts.insert((pt_r, pt_c));
                    pt_r -= slope.0;
                    pt_c -= slope.1;
                }
                let (mut pt_r, mut pt_c) = (ant_b.0, ant_b.1);
                loop {
                    if pt_r < 0 || pt_r >= *height as isize || pt_c < 0 || pt_c >= *width as isize {
                        break;
                    }
                    global_pts.insert((pt_r, pt_c));
                    pt_r += slope.0;
                    pt_c += slope.1;
                }
            }
        }
    }
    let mut t = global_pts.iter().collect::<Vec<_>>();
    t.sort();

    global_pts.len() as Num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\
"
            )),
            14
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............\
"
            )),
            34
        );
    }
}
