use std::collections::HashSet;

use aoc_runner_derive::aoc;

use pathfinding::prelude::{astar, astar_bag_collect};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Reindeer {
    y: usize,
    x: usize,
    direction: Direction,
}

impl Reindeer {
    fn new(y: usize, x: usize, direction: Direction) -> Self {
        Self { y, x, direction }
    }

    fn heuristic(&self, goal: (usize, usize)) -> usize {
        self.x.abs_diff(goal.0)
            + self.y.abs_diff(goal.1)
            + match self.x != goal.1 && self.y != goal.0 {
                true => 1000,
                false => 0,
            }
    }

    fn finished(&self, finish: (usize, usize)) -> bool {
        self.y == finish.0 && self.x == finish.1
    }

    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Reindeer, usize)> {
        let mut succ = vec![];
        match self.direction {
            Direction::East | Direction::West => {
                if grid[self.y + 1][self.x] != '#' {
                    succ.push((Reindeer::new(self.y + 1, self.x, Direction::South), 1001));
                }

                if grid[self.y - 1][self.x] != '#' {
                    succ.push((Reindeer::new(self.y - 1, self.x, Direction::North), 1001));
                }

                if self.direction == Direction::East && grid[self.y][self.x + 1] != '#' {
                    succ.push((Reindeer::new(self.y, self.x + 1, Direction::East), 1));
                }

                if self.direction == Direction::West && grid[self.y][self.x - 1] != '#' {
                    succ.push((Reindeer::new(self.y, self.x - 1, Direction::West), 1));
                }
            }

            Direction::North | Direction::South => {
                if grid[self.y][self.x + 1] != '#' {
                    succ.push((Reindeer::new(self.y, self.x + 1, Direction::East), 1001));
                }

                if grid[self.y][self.x - 1] != '#' {
                    succ.push((Reindeer::new(self.y, self.x - 1, Direction::West), 1001));
                }

                if self.direction == Direction::South && grid[self.y + 1][self.x] != '#' {
                    succ.push((Reindeer::new(self.y + 1, self.x, Direction::South), 1));
                }

                if self.direction == Direction::North && grid[self.y - 1][self.x] != '#' {
                    succ.push((Reindeer::new(self.y - 1, self.x, Direction::North), 1));
                }
            }
        };

        succ
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut reindeer = (0, 0);
    let mut end = (0, 0);

    // ToDo: integrate in grid parsing
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                reindeer = (y, x);
            }

            if grid[y][x] == 'E' {
                end = (y, x);
            }
        }
    }

    astar(
        &Reindeer::new(reindeer.0, reindeer.1, Direction::East),
        |r| r.successors(&grid),
        |r| r.heuristic((end.0, end.1)),
        |r| r.finished((end.0, end.1)),
    )
    .unwrap()
    .1
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut reindeer = (0, 0);
    let mut end = (0, 0);

    // ToDo: integrate in grid parsing
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                reindeer = (y, x);
            }

            if grid[y][x] == 'E' {
                end = (y, x);
            }
        }
    }

    let mut uniq_tiles = HashSet::with_capacity(550);
    for solution in astar_bag_collect(
        &Reindeer::new(reindeer.0, reindeer.1, Direction::East),
        |r| r.successors(&grid),
        |r| r.heuristic((end.0, end.1)),
        |r| r.finished((end.0, end.1)),
    )
    .unwrap()
    .0
    {
        for r in solution {
            uniq_tiles.insert((r.y, r.x));
        }
    }

    uniq_tiles.len()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            7036
        );

        assert_eq!(
            part1(
                "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            11048
        );
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day16.txt").expect("")),
            107468
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            45
        );

        assert_eq!(
            part2(
                "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            64
        );
        assert_eq!(
            part2(&fs::read_to_string("input/2024/day16.txt").expect("")),
            533
        );
    }
}
