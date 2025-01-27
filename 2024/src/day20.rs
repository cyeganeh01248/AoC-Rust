use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::{
    common::{HashMap, HashSet, MyMatrix, DIRS},
    parsers::v_grid_no_whitespace,
};
#[aoc_generator(day20)]
fn parse(input: &str) -> (MyMatrix<char>, (isize, isize), (isize, isize)) {
    let mut grid = v_grid_no_whitespace(input);
    let (mut start_r, mut start_c) = (0, 0);
    let (mut end_r, mut end_c) = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start_r = r as isize;
                start_c = c as isize;
            }
            if *cell == 'E' {
                end_r = r as isize;
                end_c = c as isize;
            }
        }
    }
    grid[start_r as usize][start_c as usize] = '.';
    grid[end_r as usize][end_c as usize] = '.';
    (grid, (start_r, start_c), (end_r, end_c))
}

#[aoc(day20, part1)]
fn part1(
    (grid, (start_r, start_c), (end_r, end_c)): &(MyMatrix<char>, (isize, isize), (isize, isize)),
) -> usize {
    let normal_time =
        get_cost_graph(grid, (*start_r, *start_c), (*end_r, *end_c))[&(*start_r, *start_c)];
    let t = get_path_lengths_with_cheating(grid, (*start_r, *start_c), (*end_r, *end_c), 2);
    let mut count = 0;
    for (time, cnt) in t {
        if normal_time - time >= 100 {
            count += cnt;
        }
    }
    count as usize
}
#[aoc(day20, part2)]
fn part2(
    (grid, (start_r, start_c), (end_r, end_c)): &(MyMatrix<char>, (isize, isize), (isize, isize)),
) -> usize {
    let normal_time =
        get_cost_graph(grid, (*start_r, *start_c), (*end_r, *end_c))[&(*start_r, *start_c)];
    let t = get_path_lengths_with_cheating(grid, (*start_r, *start_c), (*end_r, *end_c), 20);
    let mut _count = 0;
    for (time, cnt) in t {
        if normal_time - time >= 100 {
            _count += cnt;
        }
        if normal_time - time >= 50 {
            // println!("{}: {}", 84isize - (time as isize), cnt);
        }
    }
    todo!();
    // _count as usize
}

fn get_path_lengths_with_cheating(
    grid: &MyMatrix<char>,
    (start_r, start_c): (isize, isize),
    (end_r, end_c): (isize, isize),
    cheat_cnt: usize,
) -> HashMap<usize, i32> {
    let cost_graph = get_cost_graph(grid, (start_r, start_c), (end_r, end_c));
    let mut finish_times = HashMap::default();
    finish_times.insert(cost_graph[&(start_r, start_c)], 1);
    let (mut r, mut c) = (start_r, start_c);
    let mut visited = HashSet::default();
    let mut step_count = 0;
    while r != end_r || c != end_c {
        let (mut final_r, mut final_c) = (0, 0);
        for (dr, dc) in DIRS {
            let (nr, nc) = (r + dr, c + dc);
            if grid[nr as usize][nc as usize] == '#' {
                let mut found_ends = vec![];

                for i in 1..=((cheat_cnt - 1) as isize) {
                    for (ddr, ddc) in DIRS {
                        if ddr == -dr && ddc == -dc {
                            continue;
                        }
                        let (nnr, nnc) = (nr + ddr * i, nc + ddc * i);
                        if nnr < 0
                            || nnr >= grid.len() as isize
                            || nnc < 0
                            || nnc >= grid[0].len() as isize
                        {
                            break;
                        }
                        if grid[nnr as usize][nnc as usize] == '.' && !visited.contains(&(nnr, nnc))
                        {
                            found_ends.push((nnr, nnc));
                        }
                    }
                }
                for (nnr, nnc) in found_ends {
                    let t = step_count + cost_graph[&(nnr, nnc)] + 2;
                    finish_times.entry(t).and_modify(|e| *e += 1).or_insert(1);
                }
            } else if grid[nr as usize][nc as usize] == '.' && !visited.contains(&(nr, nc)) {
                (final_r, final_c) = (nr, nc);
            }
        }
        visited.insert((r, c));
        r = final_r;
        c = final_c;
        step_count += 1;
    }
    finish_times
}
fn get_cost_graph(
    grid: &MyMatrix<char>,
    (start_r, start_c): (isize, isize),
    (end_r, end_c): (isize, isize),
) -> HashMap<(isize, isize), usize> {
    let mut cost_graph = HashMap::default();
    cost_graph.insert((end_r, end_c), 0);
    let (mut r, mut c) = (end_r, end_c);
    let mut visited = HashSet::default();
    visited.insert((r, c));
    while r != start_r || c != start_c {
        for (dr, dc) in DIRS {
            let (nr, nc) = (r + dr, c + dc);
            if grid[nr as usize][nc as usize] != '#' && !visited.contains(&(nr, nc)) {
                visited.insert((nr, nc));
                cost_graph.insert((nr, nc), cost_graph[&(r, c)] + 1);
                r = nr;
                c = nc;
                break;
            }
        }
    }
    cost_graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############\
                "
            )),
            0
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############\
                    "
            )),
            1
        );
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
