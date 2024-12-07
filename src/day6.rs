use std::usize;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<(bool /*steppable */, u32)>> = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for (y_i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x_i, c) in line.chars().enumerate() {
            match c {
                '.' => row.push((true, 0)),
                '#' => row.push((false, 0)),
                '^' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 1
                }
                '>' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 2
                }
                'v' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 4
                }
                '<' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 8
                }
                _ => panic!(),
            }
        }

        grid.push(row);
    }

    loop {
        if grid[y][x].1 & dir != 0 {
            break;
        }

        grid[y][x].1 |= dir;

        match dir {
            1 => {
                if y == 0 {
                    break; // Off map walking
                }
                if !grid[y - 1][x].0 {
                    dir = 2;
                } else {
                    y -= 1;
                }
            }
            2 => {
                if x + 1 == grid[y].len() {
                    break;
                }

                if !grid[y][x + 1].0 {
                    dir = 4;
                } else {
                    x += 1;
                }
            }
            4 => {
                if y == grid.len() - 1 {
                    break;
                }
                if !grid[y + 1][x].0 {
                    dir = 8;
                } else {
                    y += 1;
                }
            }
            8 => {
                if x == 0 {
                    break;
                }

                if !grid[y][x - 1].0 {
                    dir = 1;
                } else {
                    x -= 1;
                }
            }
            _ => panic!(),
        }
    }

    let mut stepped_tiles = 0;
    for row in grid {
        for tile in row {
            if tile.1 != 0 {
                stepped_tiles += 1;
            }
        }
    }
    stepped_tiles
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<(bool /*steppable */, u32)>> = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for (y_i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x_i, c) in line.chars().enumerate() {
            match c {
                '.' => row.push((true, 0)),
                '#' => row.push((false, 0)),
                '^' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 1
                }
                '>' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 2
                }
                'v' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 4
                }
                '<' => {
                    row.push((true, 0));
                    x = x_i;
                    y = y_i;
                    dir = 8
                }
                _ => panic!(),
            }
        }

        grid.push(row);
    }

    let mut obs = 0;

    for y_o in 0..grid.len() {
        for x_o in 0..grid[y_o].len() {
            if !grid[y_o][x_o].0 {continue;}

            grid[y_o][x_o].0 = false;

            if loops(grid.clone(), x, y, dir){
                obs += 1;
            }

            grid[y_o][x_o].0 = true;

        }
    }

    obs
}

pub fn loops (mut grid: Vec<Vec<(bool, u32)>>, x_: usize, y_:usize, dir_: u32) -> bool {
    let mut dir = dir_;
    let mut x = x_;
    let mut y = y_;
    loop {
        if grid[y][x].1 & dir != 0 {
            return true;
        }

        grid[y][x].1 |= dir;

        match dir {
            1 => {
                if y == 0 {
                    return false;
                }
                if !grid[y - 1][x].0 {
                    dir = 2;
                } else {
                    y -= 1;
                }
            }
            2 => {
                if x + 1 == grid[y].len() {
                    return false;
                }

                if !grid[y][x + 1].0 {
                    dir = 4;
                } else {
                    x += 1;
                }
            }
            4 => {
                if y == grid.len() - 1 {
                    return false;
                }
                if !grid[y + 1][x].0 {
                    dir = 8;
                } else {
                    y += 1;
                }
            }
            8 => {
                if x == 0 {
                    return false;
                }

                if !grid[y][x - 1].0 {
                    dir = 1;
                } else {
                    x -= 1;
                }
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            41
        );

        assert_eq!(part1(&fs::read_to_string("input/2024/day6.txt").expect("")), 5162);
    }

    #[test]
    fn part2_test() {assert_eq!(
        part2(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        ),
        6
    );

    assert_eq!(part2(&fs::read_to_string("input/2024/day6.txt").expect("")), 1909);}
}
