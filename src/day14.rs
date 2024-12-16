use aoc_runner_derive::{aoc, aoc_generator};

static WIDTH: i16 = 101;
static HEIGHT: i16 = 103;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (i16, i16),
    vel: (i16, i16),
}
#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let pos = split.next().unwrap()[2..].split(',').collect::<Vec<_>>();
        let vel = split.next().unwrap()[2..].split(',').collect::<Vec<_>>();
        robots.push(Robot {
            pos: (pos[0].parse().unwrap(), pos[1].parse().unwrap()),
            vel: (vel[0].parse().unwrap(), vel[1].parse().unwrap()),
        });
    }
    robots
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> u32 {
    let mut robots = input.to_owned();
    simulate(&mut robots, 100, WIDTH, HEIGHT);
    count_quadrants(&robots, WIDTH, HEIGHT)
}

fn simulate(robots: &mut [Robot], steps: i128, width: i16, height: i16) {
    for robot in robots.iter_mut() {
        let rx = robot.pos.0 as i128;
        let ry = robot.pos.1 as i128;
        let vx = robot.vel.0 as i128;
        let vy = robot.vel.1 as i128;
        let x = (rx + vx * steps).rem_euclid(width as i128);
        let y = (ry + vy * steps).rem_euclid(height as i128);
        robot.pos.0 = x as i16;
        robot.pos.1 = y as i16;
    }
}
fn count_quadrants(robots: &[Robot], width: i16, height: i16) -> u32 {
    let mut quadrants = [0u32; 4];
    #[allow(non_snake_case)]
    let (mid_x, mid_y) = (width / 2, height / 2);
    for robot in robots.iter() {
        if robot.pos.0 == mid_x || robot.pos.1 == mid_y {
            continue;
        }
        if robot.pos.0 < mid_x && robot.pos.1 < mid_y {
            quadrants[0] += 1;
        } else if robot.pos.0 > mid_x && robot.pos.1 < mid_y {
            quadrants[1] += 1;
        } else if robot.pos.0 < mid_x && robot.pos.1 > mid_y {
            quadrants[2] += 1;
        } else if robot.pos.0 > mid_x && robot.pos.1 > mid_y {
            quadrants[3] += 1;
        }
    }
    let mut i = 1;
    for q in quadrants.iter() {
        i *= q;
    }
    i
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> u32 {
    let mut robots = input.to_owned();
    for i in 0..(WIDTH as u32 * HEIGHT as u32) {
        let mut matrix = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
        simulate(&mut robots, 1, WIDTH, HEIGHT);
        for robot in robots.iter() {
            let (x, y) = robot.pos;
            matrix[y as usize][x as usize] = '#';
        }
        let mut max_row_count = 0;
        for row in matrix.iter() {
            let mut row_count = 0;
            for i in 0..(row.len() - 1) {
                if row[i] != ' ' && row[i + 1] != ' ' {
                    row_count += 1;
                }
            }
            if max_row_count < row_count {
                max_row_count = row_count;
            }
        }
        if max_row_count > 22 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let mut robots = parse(
            "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3\
            ",
        );
        simulate(&mut robots, 100, 11, 7);
        assert_eq!(count_quadrants(&robots, 11, 7), 12);
    }
}
