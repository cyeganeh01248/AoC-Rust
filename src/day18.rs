use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::dijkstra;

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let t = line
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            (t[1], t[0])
        })
        .collect::<Vec<_>>()
}

fn list_to_grid(input: &[(u8, u8)]) -> Vec<Vec<char>> {
    let (num_r, num_c) = {
        let (mut max_r, mut max_c) = (0, 0);
        for (r, c) in input.iter() {
            if *r > max_r {
                max_r = *r;
            }
            if *c > max_c {
                max_c = *c;
            }
        }
        (max_r, max_c)
    };
    let mut grid = vec![vec!['.'; (num_c + 3) as usize]; (num_r + 3) as usize];
    for (r, c) in input.iter() {
        grid[(*r + 1) as usize][(*c + 1) as usize] = '#';
    }
    for r in 0..(num_r + 3) {
        for c in 0..(num_c + 3) {
            if r == 0 || r == (num_r + 2) || c == 0 || c == (num_c + 2) {
                grid[r as usize][c as usize] = '#';
            }
        }
    }
    grid
}

#[aoc(day18, part1)]
fn part1(input: &[(u8, u8)]) -> u16 {
    let grid = list_to_grid(&input[..1024]);
    // print_matrix(&grid);
    let _t = dijkstra(&grid, (1, 1));
    // println!("{:?}", t);
    todo!()
}

#[aoc(day18, part2)]
fn part2(_input: &[(u8, u8)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let grid = list_to_grid(&parse(
            "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        ));
        let _t = dijkstra(&grid, (1, 1));
        // println!("{:?}", t) ;
        // assert!(false);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
