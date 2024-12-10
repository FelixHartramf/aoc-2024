use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as u32 - 48).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut trails = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 0 {
                continue;
            }

            for xd in 0..11 {
                for yd in 0..11 {
                    if x >= xd
                        && y >= yd
                        && grid[y - yd][x - xd] == 9
                        && path(&grid, x, y, x - xd, y - yd)
                    {
                        trails += 1;
                    }

                    if x + xd < grid[0].len()
                        && y >= yd
                        && xd != 0
                        && grid[y - yd][x + xd] == 9
                        && path(&grid, x, y, x + xd, y - yd)
                    {
                        trails += 1;
                    }

                    if x >= xd
                        && y + yd < grid.len()
                        && yd != 0
                        && grid[y + yd][x - xd] == 9
                        && path(&grid, x, y, x - xd, y + yd)
                    {
                        trails += 1;
                    }

                    if x + xd < grid[0].len()
                        && y + yd < grid.len()
                        && yd != 0
                        && xd != 0
                        && grid[y + yd][x + xd] == 9
                        && path(&grid, x, y, x + xd, y + yd)
                    {
                        trails += 1;
                    }
                }
            }
        }
    }
    trails
}
pub fn path(grid: &Vec<Vec<u32>>, x0: usize, y0: usize, x1: usize, y1: usize) -> bool {
    if x0 == x1 && y0 == y1 {
        return true;
    }

    if x0 > 0 && grid[y0][x0 - 1] == grid[y0][x0] + 1 && path(grid, x0 - 1, y0, x1, y1) {
        return true;
    }

    if y0 > 0 && grid[y0 - 1][x0] == grid[y0][x0] + 1 && path(grid, x0, y0 - 1, x1, y1) {
        return true;
    }

    if x0 + 1 < grid[0].len()
        && grid[y0][x0 + 1] == grid[y0][x0] + 1
        && path(grid, x0 + 1, y0, x1, y1)
    {
        return true;
    }

    if y0 + 1 < grid.len() && grid[y0 + 1][x0] == grid[y0][x0] + 1 && path(grid, x0, y0 + 1, x1, y1)
    {
        return true;
    }

    false
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<(u32, u32)>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    (
                        c as u32 - 48,
                        match c {
                            '9' => 1,
                            _ => 0,
                        },
                    )
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();

    let width = grid[0].len();
    let heigth = grid.len();

    for i in (1..=8).rev() {
        for y in 0..width {
            for x in 0..heigth {
                if grid[y][x].0 != i {
                    continue;
                }

                let mut paths_to_nine = 0;

                if x > 0 && grid[y][x - 1].0 == i + 1 {
                    paths_to_nine += grid[y][x - 1].1;
                }
                if y > 0 && grid[y - 1][x].0 == i + 1 {
                    paths_to_nine += grid[y - 1][x].1;
                }

                if x + 1 < heigth && grid[y][x + 1].0 == i + 1 {
                    paths_to_nine += grid[y][x + 1].1;
                }
                if y + 1 < width && grid[y + 1][x].0 == i + 1 {
                    paths_to_nine += grid[y + 1][x].1;
                }

                grid[y][x].1 = paths_to_nine;
            }
        }
    }

    let mut sum = 0;
    for y in 0..heigth {
        for x in 0..width {
            if grid[y][x].0 != 0 {
                continue;
            }

            if x > 0 && grid[y][x - 1].0 == 1 {
                sum += grid[y][x - 1].1;
            }
            if y > 0 && grid[y - 1][x].0 == 1 {
                sum += grid[y - 1][x].1;
            }

            if x + 1 < heigth && grid[y][x + 1].0 == 1 {
                sum += grid[y][x + 1].1;
            }
            if y + 1 < width && grid[y + 1][x].0 == 1 {
                sum += grid[y + 1][x].1;
            }
        }
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
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            36
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day10.txt").expect("")),
            646
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            81
        );

        assert_eq!(
            part2(&fs::read_to_string("input/2024/day10.txt").expect("")),
            1494
        );
    }
}
