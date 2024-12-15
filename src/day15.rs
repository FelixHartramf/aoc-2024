use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let (grid_str, instructions) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut robot = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot = (y as isize, x as isize);
                grid[y][x] = '.';
            }
        }
    }

    for instruction in instructions.chars() {
        let movement: (isize, isize) = match instruction {
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => continue,
        };

        // check for free space to move
        let mut num_moves = 1;
        while grid[(robot.0 + num_moves * movement.0) as usize]
            [(robot.1 + num_moves * movement.1) as usize]
            == 'O'
        {
            num_moves += 1;
        }

        if grid[(robot.0 + num_moves * movement.0) as usize]
            [(robot.1 + num_moves * movement.1) as usize]
            == '#'
        {
            continue;
        }

        grid[(robot.0) as usize][(robot.1) as usize] = '.';

        if num_moves >= 2 {
            for i in 2..=num_moves {
                grid[(robot.0 + i * movement.0) as usize][(robot.1 + i * movement.1) as usize] =
                    'O';
            }
        }

        robot.0 = robot.0 + movement.0;
        robot.1 = robot.1 + movement.1;

        grid[robot.0 as usize][robot.1 as usize] = '.';
    }

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    sum
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let (grid_str, instructions) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = vec![Vec::with_capacity(100)];

// ToDo: Move this to the grid parsing
    let mut robot = (0, 0);
    let mut x = 0;
    let mut y = 0;
    for c in grid_str.chars() {

        let l = grid.len() - 1;
        match c {
            '#' => {
                grid[l].push('#');
                grid[l].push('#');
            }
            'O' => {
                grid[l].push('[');
                grid[l].push(']');
            }
            '.' => {
                grid[l].push('.');
                grid[l].push('.');
            }
            '@' => {
                robot = (y,x);
                grid[l].push('.');
                grid[l].push('.');
            }
            '\n' => {
                y += 1;
                x = -2;
                grid.push(Vec::with_capacity(100));
            }
            _ => panic!(),
        }
        x+= 2;
    }

    
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot = (y as isize, x as isize);
                grid[y][x] = '.';
            }
        }
    }

    for instruction in instructions.chars() {
        let movement: (isize, isize) = match instruction {
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => continue,
        };

        if grid[(robot.0 + movement.0) as usize][(robot.1 + movement.1) as usize] == '.' {
            robot.0 = robot.0 + movement.0;
            robot.1 = robot.1 + movement.1;
            continue;
        }

        // check for free space to move
        if !can_move(&mut grid, robot, movement) {
            continue;
        }
        do_move(&mut grid, (robot.0, robot.1), movement);

        robot.0 = robot.0 + movement.0;
        robot.1 = robot.1 + movement.1;

        grid[robot.0 as usize][robot.1 as usize] = '.';
    }

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                sum += 100 * y + x;
            }
        }
    }
    sum
}
pub fn do_move(grid: &mut Vec<Vec<char>>, current: (isize, isize), movement: (isize, isize)) {
    if movement.0 != 0
        && (grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] == '['
            || grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] == ']')
        && grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize]
            != grid[current.0 as usize][current.1 as usize]
    {
        if grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] == '[' {
            do_move(
                grid,
                (current.0 + movement.0, current.1 + movement.1 + 1),
                movement,
            );
        } else {
            do_move(
                grid,
                (current.0 + movement.0, current.1 + movement.1 - 1),
                movement,
            );
        }
    }

    if grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] != '.' {
        do_move(
            grid,
            (current.0 + movement.0, current.1 + movement.1),
            movement,
        );
    }

    grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] =
        grid[current.0 as usize][current.1 as usize];
    grid[current.0 as usize][current.1 as usize] = '.';
}

pub fn can_move(
    grid: &mut Vec<Vec<char>>,
    current: (isize, isize),
    movement: (isize, isize),
) -> bool {
    if grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] == '.' {
        return true;
    }
    if grid[(current.0 + movement.0) as usize][(current.1 + movement.1) as usize] == '#' {
        return false;
    }

    if movement.1 != 0 {
        if can_move(grid, (current.0, current.1 + movement.1), movement) {
            return true;
        }
        return false;
    }

    if movement.0 != 0 {
        if !can_move(grid, (current.0 + movement.0, current.1), movement) {
            return false;
        }
        if grid[(current.0 + movement.0) as usize][current.1 as usize]
            != grid[current.0 as usize][current.1 as usize]
        {
            if grid[(current.0 + movement.0) as usize][(current.1) as usize] == '[' {
                return can_move(grid, (current.0 + movement.0, current.1 + 1), movement);
            } else {
                return can_move(grid, (current.0 + movement.0, current.1 - 1), movement);
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            10092
        );
        assert!(part1(&fs::read_to_string("input/2024/day15.txt").expect("")) > 0);
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day15.txt").expect("")),
            1487337
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            ),
            105 + 207 + 306
        );

        // From : https://github.com/Fadi88/AoC/blob/master/2024/day15/test_corner.txt
        assert_eq!(
            part2(
                "#######
#.....#
#.....#
#.@O..#
#..#O.#
#...O.#
#..O..#
#.....#
#######

>><vvv>v>^^^"
            ),
            1430
        );

        assert_eq!(
            part2(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            9021
        );
        assert_eq!(
            part2(&fs::read_to_string("input/2024/day15.txt").expect("")),
            1521952
        );
    }
}
