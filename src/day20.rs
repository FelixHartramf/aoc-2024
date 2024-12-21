use aoc_runner_derive::aoc;

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let grid_c: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut grid: Vec<Vec<isize>> = Vec::with_capacity(grid_c.len());

    let mut start = (0, 0);
    let mut end = (0, 0);

    // ToDo: integrate in grid parsing
    for y in 0..grid_c.len() {
        grid.push(Vec::with_capacity(grid_c[y].len()));
        for x in 0..grid_c[y].len() {
            match grid_c[y][x] {
                'S' => {
                    grid[y].push(1);
                    start = (y, x);
                }
                'E' => {
                    end = (y, x);
                    grid[y].push(0);
                }
                '#' => grid[y].push(-1),
                '.' => grid[y].push(0),
                _ => panic!(),
            }
        }
    }

    let mut current_pos = start;
    let mut current_index = 1;

    while current_pos != end {
        current_index += 1;
        if grid[current_pos.0 - 1][current_pos.1] == 0 {
            grid[current_pos.0 - 1][current_pos.1] = current_index;
            current_pos.0 -= 1;
        } else if grid[current_pos.0 + 1][current_pos.1] == 0 {
            grid[current_pos.0 + 1][current_pos.1] = current_index;
            current_pos.0 += 1;
        } else if grid[current_pos.0][current_pos.1 - 1] == 0 {
            grid[current_pos.0][current_pos.1 - 1] = current_index;
            current_pos.1 -= 1;
        } else if grid[current_pos.0][current_pos.1 + 1] == 0 {
            grid[current_pos.0][current_pos.1 + 1] = current_index;
            current_pos.1 += 1;
        }
    }

    let mut above_100 = 0;
    let cheats = 2;
    let number = 100 + cheats;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] != -1 {
                continue;
            }

            if grid[y][x - 1] != -1
                && grid[y][x + 1] != -1
                && grid[y][x - 1].abs_diff(grid[y][x + 1]) >= number
            {
                above_100 += 1;
                continue;
            }

            if grid[y - 1][x] != -1
                && grid[y + 1][x] != -1
                && grid[y - 1][x].abs_diff(grid[y + 1][x]) >= number
            {
                above_100 += 1;
                continue;
            }

            if grid[y][x - 1] != -1
                && grid[y + 1][x] != -1
                && grid[y][x - 1].abs_diff(grid[y + 1][x]) >= number
            {
                above_100 += 1;
                continue;
            }

            if grid[y][x + 1] != -1
                && grid[y - 1][x] != -1
                && grid[y - 1][x].abs_diff(grid[y][x + 1]) >= number
            {
                above_100 += 1;
                continue;
            }
        }
    }

    above_100
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    // ToDo: integrate in grid parsing
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                'S' => {
                    start = (y, x);
                    grid[y][x] = '.';
                }
                'E' => {
                    end = (y, x);
                    grid[y][x] = '.';
                }
                _ => {}
            }
        }
    }

    let mut current_pos = start;

    let mut path: Vec<(usize, usize)> = vec![start];
    while current_pos != end {
        if grid[current_pos.0 - 1][current_pos.1] == '.'
            && (path.len() == 1 ||path[path.len()-2] != (current_pos.0 - 1, current_pos.1))
        {
            current_pos.0 -= 1;
        } else if grid[current_pos.0 + 1][current_pos.1] == '.'
            && (path.len() == 1 ||path[path.len()-2] != (current_pos.0 + 1, current_pos.1))
        {
            current_pos.0 += 1;
        } else if grid[current_pos.0][current_pos.1 - 1] == '.'
            && (path.len() == 1 ||path[path.len()-2] != (current_pos.0, current_pos.1 - 1))
        {
            current_pos.1 -= 1;
        } else if grid[current_pos.0][current_pos.1 + 1] == '.'
            && (path.len() == 1 || path[path.len()-2] != (current_pos.0, current_pos.1 + 1))
        {
            current_pos.1 += 1;
        }
        path.push(current_pos);
    }

    let mut above_100 = 0;
    let cheats = 20;
    let number = 100;

    for (i, tile) in path[..path.len()-number].iter().enumerate() {
        for (i2, tile2) in path[i+number+1..].iter().enumerate() {
            // Skip if too many cheats would be used
            if tile.0.abs_diff(tile2.0) + tile.1.abs_diff(tile2.1) > cheats {
                continue;
            }
            
            // After some optimisations this stayed till the end
            if  tile.0.abs_diff(tile2.0) + tile.1.abs_diff(tile2.1) >= i2+2{
                continue;
            }

            
            above_100 += 1;
        }
    }

    above_100
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
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            ),
            0
        );
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day20.txt").expect("")),
            1530
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&fs::read_to_string("input/2024/day20.txt").expect("")),
            1033983
        );
    }
}
