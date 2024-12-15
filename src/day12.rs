use aoc_runner_derive::{aoc, aoc_generator};

use crate::{
    common::{HashMap, HashSet, Matrix},
    parsers::v_grid_no_whitespace,
};

type Num = u64;

#[aoc_generator(day12)]
fn parse(input: &str) -> Matrix<char> {
    let grid = v_grid_no_whitespace(input);
    assert!(grid.len() == grid[0].len());
    grid
}

#[aoc(day12, part1)]
fn part1(garden: &Matrix<char>) -> Num {
    let width = garden.len();

    let mut perimeters = HashMap::default();
    let mut areas = HashMap::default();

    let mut not_checked = HashSet::default();
    not_checked.reserve(width * width);
    for r in 0..width {
        for c in 0..width {
            not_checked.insert((r as isize, c as isize));
        }
    }

    let mut id = 1;

    while not_checked.len() > 0 {
        // pop off an element
        let (r, c) = not_checked.iter().next().unwrap().clone();
        not_checked.remove(&(r, c));
        let cell_code = garden[r as usize][c as usize];
        let area_code = (id, cell_code);

        let points = find_points_in_region(garden, r, c);
        areas.insert(area_code, points.len() as Num);
        perimeters.insert(area_code, 0 as Num);
        for p in points.iter() {
            not_checked.remove(&p);
            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nr, nc) = (p.0 + dr, p.1 + dc);
                if nr < 0
                    || nr >= width as isize
                    || nc < 0
                    || nc >= width as isize
                    || cell_code != garden[nr as usize][nc as usize]
                {
                    perimeters.entry(area_code).and_modify(|e| *e += 1);
                    if area_code.1 == 'R' {};
                    continue;
                }
            }
        }
        id += 1;
    }

    areas
        .keys()
        .map(|area_code| areas.get(area_code).unwrap() * perimeters.get(area_code).unwrap())
        .sum()
}

fn find_points_in_region(garden: &Matrix<char>, r: isize, c: isize) -> HashSet<(isize, isize)> {
    let mut points = HashSet::default();
    points.insert((r, c));
    find_points_in_region_helper(garden, r, c, &mut points);
    points
}
fn find_points_in_region_helper(
    garden: &Matrix<char>,
    r: isize,
    c: isize,
    points: &mut HashSet<(isize, isize)>,
) {
    let width = garden.len();
    if r < 0 || r >= width as isize || c < 0 || c >= width as isize {
        return;
    }
    let cell = garden[r as usize][c as usize];
    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (nr, nc) = (r + dr, c + dc);
        if nr < 0
            || nr >= width as isize
            || nc < 0
            || nc >= width as isize
            || points.contains(&(nr, nc))
        {
            continue;
        }
        if garden[nr as usize][nc as usize] == cell {
            points.insert((nr, nc));
            find_points_in_region_helper(garden, nr, nc, points);
        }
    }
}

#[aoc(day12, part2)]
fn part2(garden: &Matrix<char>) -> Num {
    // print_matrix(garden);
    let width = garden.len();

    let mut perimeters = HashMap::default();
    let mut areas = HashMap::default();

    let mut not_checked = HashSet::default();
    not_checked.reserve(width * width);
    for r in 0..width {
        for c in 0..width {
            not_checked.insert((r as isize, c as isize));
        }
    }

    let mut id = 1;

    while not_checked.len() > 0 {
        // pop off an element
        let (r, c) = not_checked.iter().next().unwrap().clone();
        not_checked.remove(&(r, c));
        let cell_code = garden[r as usize][c as usize];
        let area_code = (id, cell_code);

        let mut points = find_points_in_region(garden, r, c);
        areas.insert(area_code, points.len() as Num);
        perimeters.insert(area_code, 0 as Num);
        for p in points.clone().iter() {
            points.insert((p.0 + 1, p.1 + 1));
            not_checked.remove(&p);
        }

        for p in points.iter() {
            let _cells = 0u8;
            for (dr, dc) in [(0, 0), (0, -1), (-1, -1), (-1, 0)] {
                let (nr, nc) = (p.0 + dr, p.1 + dc);

                if dr != 0 && nr < 0 {}
                if dc != 0 && nc < 0 {}
            }
        }
        id += 1;
    }

    for _area_code in areas.keys() {
        // println!(
        //     "{:?} {} * {} = {}",
        //     area_code,
        //     areas.get(area_code).unwrap(),
        //     perimeters.get(area_code).unwrap(),
        //     areas.get(area_code).unwrap() * perimeters.get(area_code).unwrap()
        // );
    }
    areas
        .keys()
        .map(|area_code| areas.get(area_code).unwrap() * perimeters.get(area_code).unwrap())
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(
            part1(&parse(
                "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE\
"
            )),
            1930
        );
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(
            part1(&parse(
                "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO\
"
            )),
            772
        );
    }

    // #[test]
    fn part2_example_1() {
        assert_eq!(
            part2(&parse(
                "\
AAAA
BBCD
BBCC
EEEC\
"
            )),
            80
        );
    }
    // #[test]
    fn part2_example_2() {
        assert_eq!(
            part2(&parse(
                "\
OOOOO
OOOOO
OOOOO
OXOXO
OOOOO\
"
            )),
            436
        );
    }
    // #[test]
    fn part2_example_3() {
        assert_eq!(
            part2(&parse(
                "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE\
"
            )),
            236
        );
    }
    // #[test]
    fn part2_example_4() {
        assert_eq!(
            part2(&parse(
                "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA\
"
            )),
            368
        );
    }
    // #[test]
    fn part2_example_5() {
        assert_eq!(
            part2(&parse(
                "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE\
"
            )),
            1206
        );
    }
}
