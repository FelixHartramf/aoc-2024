use std::{collections::{HashMap, HashSet}, usize};

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut beacons: HashMap<char, Vec<(isize, isize)>> = HashMap::with_capacity(50);

    let grid = input.lines().collect::<Vec<&str>>();

    let x_max = grid[0].len() as isize;
    let y_max = grid.len() as isize;

    let mut locations: Vec<(isize, isize)> = Vec::with_capacity(250);

    for (y, line) in grid.iter().enumerate() {
        
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if !beacons.contains_key(&c) {
                beacons.insert(c, Vec::with_capacity(7));
            }

            for b in beacons.get(&c).unwrap(){
                let dx =  b.0 - x as isize;
                let dy = b.1 - y as isize;

                let x3 = x as isize- dx;
                let y3 = y as isize - dy ;

                let x4 =  b.0 + dx;
                let y4 = b.1 + dy;

                if x3 >= 0 && x3 < x_max && y3 >= 0 && y3 < y_max  {
                    locations.push((x3, y3));
                }

                if x4 >= 0 && x4 < x_max && y4 >= 0 && y4 < y_max {
                    locations.push((x4, y4));
                }
            }

            beacons.get_mut(&c).unwrap().push((x as isize, y as isize));
        }
    }

    locations.sort();
    locations.dedup();

    locations.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut beacons: HashMap<char, Vec<(isize, isize)>> = HashMap::with_capacity(50);

    let mut x_max = 0;
    let mut y_max = 0;
    for (y, line) in input.lines().enumerate() {
        y_max += 1;
        if x_max == 0 {
            x_max = line.len() as isize;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            if !beacons.contains_key(&c) {
                beacons.insert(c, vec![]);
            }

            beacons.get_mut(&c).unwrap().push((x as isize, y as isize));
        }
    }

    let mut locations: HashSet<(isize, isize)> = HashSet::with_capacity(50);
    for (_, beacons) in beacons.iter() {
        for b1 in beacons {
            for b2 in beacons {
                if b1 == b2 {
                    continue;
                }

                let dx = b2.0 - b1.0;
                let dy = b2.1 - b1.1;

                let mut i = 0;
                while b1.0 - (i * dx) >= 0
                    && b1.0 - (i * dx) < x_max
                    && b1.1 - (i * dy) >= 0
                    && b1.1 - (i * dy) < y_max
                {
                    locations.insert((b1.0 - (i * dx), b1.1 - (i * dy)));
                    i += 1;
                }

                i = 0;
                while b2.0 + (i * dx) >= 0
                    && b2.0 + (i * dx) < x_max
                    && b2.1 + (i * dy) >= 0
                    && b2.1 + (i * dy) < y_max
                {
                    locations.insert((b2.0 + (i * dx), b2.1 + (i * dy)));
                    i += 1;
                }
            }
        }
    }

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
............
"
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
..........
"
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
............
"
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
