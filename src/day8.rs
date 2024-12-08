use std::{collections::HashMap, usize};

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut beacons: HashMap<char, Vec<(isize, isize)>> = HashMap::with_capacity(50);

    let grid = input.lines().collect::<Vec<&str>>();

    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;

    let mut locations: Vec<(isize, isize)> = Vec::with_capacity(250);

    for (y_i, line) in grid.iter().enumerate() {
        for (x_i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if !beacons.contains_key(&c) {
                beacons.insert(c, Vec::with_capacity(7));
            }

            let x = x_i as isize;
            let y = y_i as isize;

            for b in beacons.get(&c).unwrap() {
                let dx = b.0 - x;
                let dy = b.1 - y;

                let x3 = x - dx;
                let y3 = y - dy;

                let x4 = b.0 + dx;
                let y4 = b.1 + dy;

                if x3 >= 0 && x3 < x_max && y3 >= 0 && y3 < y_max {
                    locations.push((x3, y3));
                }

                if x4 >= 0 && x4 < x_max && y4 >= 0 && y4 < y_max {
                    locations.push((x4, y4));
                }
            }

            beacons.get_mut(&c).unwrap().push((x, y));
        }
    }

    locations.sort();
    locations.dedup();

    locations.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut beacons: HashMap<char, Vec<(isize, isize)>> = HashMap::with_capacity(50);

    let grid = input.lines().collect::<Vec<&str>>();

    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;

    let mut locations: Vec<(isize, isize)> = Vec::with_capacity(800);

    for (y_i, line) in grid.iter().enumerate() {
        for (x_i, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if !beacons.contains_key(&c) {
                beacons.insert(c, Vec::with_capacity(7));
            }
            let x = x_i as isize;
            let y = y_i as isize;

            for b in beacons.get(&c).unwrap() {
                let dx = x - b.0;
                let dy = y - b.1;

                let mut last_x = x;
                let mut last_y = y;
                while last_x - dx >= 0
                    && last_x - dx < x_max
                    && last_y - dy >= 0
                    && last_y - dy < y_max
                {
                    last_x -= dx;
                    last_y -= dy;
                    locations.push((last_x, last_y));
                }

                last_x = b.0;
                last_y = b.1;
                while last_x + dx >= 0
                    && last_x + dx < x_max
                    && last_y + dy >= 0
                    && last_y + dy < y_max
                {
                    last_x += dx;
                    last_y += dy;
                    locations.push((last_x, last_y));
                }
            }

            beacons.get_mut(&c).unwrap().push((x, y));
        }
    }

    locations.sort();
    locations.dedup();

    locations.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "............
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
............"
            ),
            14
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day8.txt").expect("")),
            228
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
            ),
            9
        );

        assert_eq!(
            part2(
                "............
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
............"
            ),
            34
        );

        assert!(part2(&fs::read_to_string("input/2024/day8.txt").expect("")) > 228);

        assert_eq!(
            part2(&fs::read_to_string("input/2024/day8.txt").expect("")),
            766
        );
    }
}
