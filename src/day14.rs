use aoc_runner_derive::aoc;
use regex::Regex;

use std::{thread, time::Duration};

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
    let width = 101;
    let heigth = 103;

    let sec = 100;

    let mut num_robots_quatrant = [0, 0, 0, 0];

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in input.lines() {
        let (_, [x_p, y_p, x_v, y_v]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();
        let mut pos: (i32, i32) = (x_p.parse().unwrap(), y_p.parse().unwrap());
        let vel: (i32, i32) = (x_v.parse().unwrap(), y_v.parse().unwrap());
        for _ in 0..sec {
            pos.0 = (pos.0 + vel.0).rem_euclid(width);
            pos.1 = (pos.1 + vel.1).rem_euclid(heigth);
        }
        if pos.0 > width / 2 && pos.1 > heigth / 2 {
            num_robots_quatrant[0] += 1;
        }

        if pos.0 < width / 2 && pos.1 > heigth / 2 {
            num_robots_quatrant[1] += 1;
        }

        if pos.0 > width / 2 && pos.1 < heigth / 2 {
            num_robots_quatrant[2] += 1;
        }

        if pos.0 < width / 2 && pos.1 < heigth / 2 {
            num_robots_quatrant[3] += 1;
        }
    }

    num_robots_quatrant[0]
        * num_robots_quatrant[1]
        * num_robots_quatrant[2]
        * num_robots_quatrant[3]
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
    let width = 101;
    let heigth = 103;

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = vec![];

    for line in input.lines() {
        let (_, [x_p, y_p, x_v, y_v]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();
        robots.push((
            (x_p.parse::<i32>().unwrap(), y_p.parse::<i32>().unwrap()),
            (x_v.parse::<i32>().unwrap(), y_v.parse::<i32>().unwrap()),
        ));
    }

    let mut sec = 0;

    loop {
        for robot in robots.iter_mut() {
            robot.0 .0 = (robot.0 .0 + robot.1 .0).rem_euclid(width);
            robot.0 .1 = (robot.0 .1 + robot.1 .1).rem_euclid(heigth);
        }
        sec += 1;
        if easter_egg_heuristic(&robots) {
            return sec;
        }
    }
}

pub fn easter_egg_heuristic(robots: &Vec<((i32, i32), (i32, i32))>) -> bool {
    /*I assumed that a lot of robots would be close to gether */
    let mut robots_with_neigbors = 0;
    for robot1 in robots {
        for robot2 in robots {
            if robot1 == robot2 {
                continue;
            }

            if robot1.0 .0.abs_diff(robot2.0 .0) <= 1 && robot1.0 .1.abs_diff(robot2.0 .1) <= 1 {
                robots_with_neigbors += 1;
            }
        }
    }

    robots_with_neigbors > robots.len() * 2
}

pub fn print_robots(robots: &Vec<((i32, i32), (i32, i32))>) {
    for y in 0..103 {
        'x_loop: for x in 0..101 {
            for robot in robots {
                if robot.0 .0 == x && robot.0 .1 == y {
                    print!("#");
                    continue 'x_loop;
                }
            }
            print!(".");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert!(part1(&fs::read_to_string("input/2024/day14.txt").expect("")) > 84134700);
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day14.txt").expect("")),
            230172768
        );
    }

    #[test]
    fn part2_test() {assert_eq!(
        part2(&fs::read_to_string("input/2024/day14.txt").expect("")),
        8087
    );}
}
