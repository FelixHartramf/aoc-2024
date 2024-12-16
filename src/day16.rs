use std::collections::HashSet;

use aoc_runner_derive::aoc;

use pathfinding::prelude::astar;

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
    fn new(y: usize, x: usize, dir: Direction) -> Self {
        Self {
            y: y,
            x: x,
            direction: dir,
        }
    }

    fn heuristic(&self, goal: (usize, usize)) -> usize {
        self.x.abs_diff(goal.0) + self.y.abs_diff(goal.1)
    }

    fn heuristic_part2(&self, goal: (usize, usize)) -> usize {
        self.x.abs_diff(goal.0) + self.y.abs_diff(goal.1)
    }

    fn finished(&self, finish: (usize, usize)) -> bool {
        self.y == finish.0 && self.x == finish.1
    }

    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Reindeer, usize)> {
        
        let mut succ = match self.direction {
            Direction::East | Direction::West => {
                vec![
                    (Reindeer::new(self.y, self.x, Direction::South), 1000),
                    (Reindeer::new(self.y, self.x, Direction::North), 1000),
                ]
            }

            Direction::North | Direction::South => {
                vec![
                    (Reindeer::new(self.y, self.x, Direction::East), 1000),
                    (Reindeer::new(self.y, self.x, Direction::West), 1000),
                ]
            }
        };

        if self.direction == Direction::East && grid[self.y][self.x + 1] != '#' {
            succ.push((Reindeer::new(self.y, self.x + 1, Direction::East), 1));
        }

        if self.direction == Direction::West && grid[self.y][self.x - 1] != '#' {
            succ.push((Reindeer::new(self.y, self.x - 1, Direction::West), 1));
        }

        if self.direction == Direction::South && grid[self.y + 1][self.x] != '#' {
            succ.push((Reindeer::new(self.y + 1, self.x, Direction::South), 1));
        }

        if self.direction == Direction::North && grid[self.y - 1][self.x] != '#' {
            succ.push((Reindeer::new(self.y - 1, self.x, Direction::North), 1));
        }

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

    let mut to_check: HashSet<(usize, usize)> = HashSet::new();
    // ToDo: integrate in grid parsing
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                continue;
            }

            
            if grid[y][x] == 'S' {
                reindeer = (y, x);
            }

            if grid[y][x] == 'E' {
                end = (y, x);
            }
            
            to_check.insert((y, x));
        }
    }

    let best_meow = astar(
        &Reindeer::new(reindeer.0, reindeer.1, Direction::East),
        |r| r.successors(&grid),
        |r| r.heuristic((end.0, end.1)),
        |r| r.finished((end.0, end.1)),
    )
    .unwrap();
    let mut sum = 0;
    for r in best_meow.0 {
        if to_check.contains(&(r.y, r.x)) {
            sum += 1;
            to_check.remove(&(r.y, r.x));
        }
    }

    let best = best_meow.1;

    while to_check.len() > 0 {
        dbg!(to_check.len());
        let y = to_check.iter().next().unwrap().0;
        let x = to_check.iter().next().unwrap().1;

        let to_solution_meow = astar(
            &Reindeer::new(reindeer.0, reindeer.1, Direction::East),
            |r| r.successors(&grid),
            |r| r.heuristic((y, x)),
            |r| r.finished((y, x)),
        );
        if to_solution_meow == None {
            to_check.remove(&(y, x));
        }

        let to_solution = to_solution_meow.unwrap();
        let to = to_solution.1;

        if to > best + 1 {
            to_check.remove(&(y, x));
        }

        let from = astar(
            to_solution.0.last().unwrap(),
            |r| r.successors(&grid),
            |r| r.heuristic((end.0, end.1)),
            |r| r.finished((end.0, end.1)),
        )
        .unwrap();

        if to + from.1 == best {
            for r in to_solution.0 {
                if to_check.contains(&(r.y, r.x)) {
                    to_check.remove(&(r.y, r.x));
                    sum += 1;
                }
            }

            for r in from.0 {
                if to_check.contains(&(r.y, r.x)) {
                    to_check.remove(&(r.y, r.x));
                    sum += 1;
                }
            }

        }

        to_check.remove(&(y, x));
    }

    sum
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
