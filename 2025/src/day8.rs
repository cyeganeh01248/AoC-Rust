use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

type Point3d = (i64, i64, i64);

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<(Point3d, Point3d)> {
    let points = input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();
            (x, y, z)
        })
        .collect::<Vec<_>>();
    let mut points_distance = Vec::new();
    for p1_i in 0..(points.len() - 1) {
        for p2_i in (p1_i + 1)..points.len() {
            let p1 = points[p1_i];
            let p2 = points[p2_i];
            let distance = ((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2));
            points_distance.push((p1, p2, distance));
        }
    }
    points_distance.sort_by(|a, b| a.2.cmp(&b.2));
    points_distance
        .iter()
        .map(|a| (a.0, a.1))
        .collect::<Vec<_>>()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<(Point3d, Point3d)>) -> u32 {
    let input = input.iter().take(1000).collect::<Vec<_>>();
    let mut point_groups = FxHashMap::default();

    let mut min_group = 1;

    for (a, b) in input {
        let a_group = *point_groups.get(a).unwrap_or(&0);
        let b_group = *point_groups.get(b).unwrap_or(&0);
        if a_group != 0 && b_group != 0 {
            for (_, v) in point_groups.iter_mut() {
                if *v == b_group {
                    *v = a_group;
                }
            }
        } else if a_group != 0 {
            point_groups.insert(*b, a_group);
        } else if b_group != 0 {
            point_groups.insert(*a, b_group);
        } else {
            point_groups.insert(*a, min_group);
            point_groups.insert(*b, min_group);
            min_group += 1;
        }
    }
    let mut group_sizes = FxHashMap::default();
    for (_, group_id) in point_groups.iter() {
        *group_sizes.entry(*group_id).or_insert(0) += 1;
    }
    let mut t = group_sizes.into_iter().collect::<Vec<_>>();
    t.sort_by(|a, b| b.1.cmp(&a.1));
    t.iter().take(3).map(|a| a.1).product()
}

#[aoc(day8, part2)]
fn part2(input: &Vec<(Point3d, Point3d)>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689"
    };

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(EXAMPLE_INPUT).into_iter().take(10).collect()),
            40
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), "<RESULT>");
    }
}
