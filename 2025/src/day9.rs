use std::i64;

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|points| (points[0], points[1]))
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(i64, i64)]) -> i64 {
    let mut max_area = 0;

    for p1_i in 0..input.len() {
        for p2_i in p1_i + 1..input.len() {
            let p1 = input[p1_i];
            let p2 = input[p2_i];

            let area = ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

// UGLY UNOPTIMAL CODE, takes ~5 min on my computer
#[aoc(day9, part2)]
fn part2(input: &[(i64, i64)]) -> i64 {
    let reds = FxHashSet::from_iter(input.iter().copied());
    let mut cache = FxHashMap::default();
    let mut max_area = 0;

    for p1_i in 0..input.len() {
        for p2_i in p1_i + 1..input.len() {
            let p1 = input[p1_i];
            let p2 = input[p2_i];

            let min_x = p1.0.min(p2.0);
            let max_x = p1.0.max(p2.0);
            let min_y = p1.1.min(p2.1);
            let max_y = p1.1.max(p2.1);
            let mut valid = true;

            for test_x in min_x..=max_x {
                let point1 = (test_x, min_y);
                let point2 = (test_x, max_y);
                if (!reds.contains(&point1) && !is_point_valid_cached(point1, input, &mut cache))
                    || (!reds.contains(&point2)
                        && !is_point_valid_cached(point2, input, &mut cache))
                {
                    valid = false;
                    break;
                }
            }
            for test_y in min_y..=max_y {
                let point1 = (min_x, test_y);
                let point2 = (max_x, test_y);
                if (!reds.contains(&point1) && !is_point_valid_cached(point1, input, &mut cache))
                    || (!reds.contains(&point2)
                        && !is_point_valid_cached(point2, input, &mut cache))
                {
                    valid = false;
                    break;
                }
            }

            if valid {
                let area = ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);
                max_area = max_area.max(area);
            }
        }
    }
    max_area
}

fn is_point_valid_cached(
    point: (i64, i64),
    polygon: &[(i64, i64)],
    cache: &mut FxHashMap<(i64, i64), bool>,
) -> bool {
    if let Some(&result) = cache.get(&point) {
        return result;
    }

    let result = _is_point_inside_polygon(point, polygon) || _is_on_polygon_edge(point, polygon);
    cache.insert(point, result);
    result
}
fn _is_point_inside_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

fn _is_on_polygon_edge(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];

        if _is_point_on_line_segment(point, a, b) {
            return true;
        }
    }
    false
}

fn _is_point_on_line_segment(point: (i64, i64), a: (i64, i64), b: (i64, i64)) -> bool {
    let (px, py) = point;
    let (ax, ay) = a;
    let (bx, by) = b;

    if ax == bx {
        px == ax && py >= ay.min(by) && py <= ay.max(by)
    } else if ay == by {
        py == ay && px >= ax.min(bx) && px <= ax.max(bx)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 24);
    }
}
