use aoc_runner_derive::aoc;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut used: Vec<(usize, usize)> = vec![];
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if used.contains(&(y, x)) {
                continue;
            }

            let mut current: Vec<(usize, usize)> = vec![(y, x)];

            let mut last_length = 0;
            while last_length < current.len() {
                last_length = current.len();

                for i in 0..current.len() {
                    let x1 = current[i].1;
                    let y1 = current[i].0;
                    if x1 > 0 && !current.contains(&(y1, x1 - 1)) && grid[y1][x1 - 1] == grid[y][x]
                    {
                        current.push((y1, x1 - 1));
                    }

                    if y1 > 0 && !current.contains(&(y1 - 1, x1)) && grid[y1 - 1][x1] == grid[y][x]
                    {
                        current.push((y1 - 1, x1));
                    }

                    if x1 + 1 < grid[0].len()
                        && !current.contains(&(y1, x1 + 1))
                        && grid[y1][x1 + 1] == grid[y][x]
                    {
                        current.push((y1, x1 + 1));
                    }

                    if y1 + 1 < grid.len()
                        && !current.contains(&(y1 + 1, x1))
                        && grid[y1 + 1][x1] == grid[y][x]
                    {
                        current.push((y1 + 1, x1));
                    }
                }
            }

            sum += current.len() * get_perim(&current);
            used.append(&mut current);
        }
    }
    sum
}

pub fn get_perim(fields: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for field in fields {
        let mut neighbors = 0;

        if field.0 > 0 && fields.contains(&(field.0 - 1, field.1)) {
            neighbors += 1;
        }

        if field.1 > 0 && fields.contains(&(field.0, field.1 - 1)) {
            neighbors += 1;
        }

        if fields.contains(&(field.0 + 1, field.1)) {
            neighbors += 1;
        }

        if fields.contains(&(field.0, field.1 + 1)) {
            neighbors += 1;
        }

        sum += 4 - neighbors;
    }
    sum
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut used: Vec<(usize, usize)> = vec![];
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if used.contains(&(y, x)) {
                continue;
            }

            let mut current: Vec<(usize, usize)> = vec![(y, x)];

            let mut last_length = 0;
            while last_length < current.len() {
                last_length = current.len();

                for i in 0..current.len() {
                    let x1 = current[i].1;
                    let y1 = current[i].0;
                    if x1 > 0 && !current.contains(&(y1, x1 - 1)) && grid[y1][x1 - 1] == grid[y][x]
                    {
                        current.push((y1, x1 - 1));
                    }

                    if y1 > 0 && !current.contains(&(y1 - 1, x1)) && grid[y1 - 1][x1] == grid[y][x]
                    {
                        current.push((y1 - 1, x1));
                    }

                    if x1 + 1 < grid[0].len()
                        && !current.contains(&(y1, x1 + 1))
                        && grid[y1][x1 + 1] == grid[y][x]
                    {
                        current.push((y1, x1 + 1));
                    }

                    if y1 + 1 < grid.len()
                        && !current.contains(&(y1 + 1, x1))
                        && grid[y1 + 1][x1] == grid[y][x]
                    {
                        current.push((y1 + 1, x1));
                    }
                }
            }

            sum += current.len() * get_sides(&current);
            used.append(&mut current);
        }
    }
    sum
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Side {
    Left,
    Up,
    Right,
    Down,
}

pub fn get_sides(fields: &Vec<(usize, usize)>) -> usize {
    let mut fences: Vec<(Side, usize, usize)> = vec![];
    for field in fields {
        let x = field.1;
        let y = field.0;

        if !fields.contains(&(y, x + 1)) {
            fences.push((Side::Right, y, x));
        }

        if !fields.contains(&(y + 1, x)) {
            fences.push((Side::Down, y, x));
        }

        if x == 0 || !fields.contains(&(y, x - 1)) {
            fences.push((Side::Left, y, x));
        }

        if y == 0 || !fields.contains(&(y - 1, x)) {
            fences.push((Side::Up, y, x));
        }
    }

    let mut removed_fences: Vec<(Side, usize, usize)> = vec![];

    for i in 0..fences.len() {
        for k in 0..fences.len() {
            if i == k || fences[i].0 != fences[k].0 {
                continue;
            }

            if (fences[i].0 == Side::Up || fences[i].0 == Side::Down)
                && fences[i].1 == fences[k].1
                && fences[i].2 == fences[k].2 + 1
            {
                removed_fences.push(fences[k]);
            }

            if (fences[i].0 == Side::Left || fences[i].0 == Side::Right)
                && fences[i].2 == fences[k].2
                && fences[i].1 == fences[k].1 + 1
            {
                removed_fences.push(fences[k]);
            }
        }
    }

    fences.len() - removed_fences.len()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1930
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day12.txt").expect("")),
            1446042
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1206
        );

        assert!(part2(&fs::read_to_string("input/2024/day12.txt").expect("")) < 904658);

        assert_eq!(
            part2(&fs::read_to_string("input/2024/day12.txt").expect("")),
            902742
        );
    }
}
