use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let row_len = input.find('\n').unwrap() + 1;

    let char_input = input.chars().collect::<Vec<char>>();
    let input_len = char_input.len();

    let mut xmas_count = 0;

    for i in 0..input_len {
        if char_input[i] != 'X' {
            continue;
        }

        if i + 3 < input_len
            && char_input[i + 1] == 'M'
            && char_input[i + 2] == 'A'
            && char_input[i + 3] == 'S'
        {
            xmas_count += 1;
        }

        if i >= 3
            && char_input[i - 1] == 'M'
            && char_input[i - 2] == 'A'
            && char_input[i - 3] == 'S'
        {
            xmas_count += 1;
        }

        if i + 3 * row_len < input_len
            && char_input[i + 1 * row_len] == 'M'
            && char_input[i + 2 * row_len] == 'A'
            && char_input[i + 3 * row_len] == 'S'
        {
            xmas_count += 1;
        }

        if i >= 3 * row_len
            && char_input[i - 1 * row_len] == 'M'
            && char_input[i - 2 * row_len] == 'A'
            && char_input[i - 3 * row_len] == 'S'
        {
            xmas_count += 1;
        }

        if i + 3 * row_len + 3 < input_len
            && char_input[i + 1 * row_len + 1] == 'M'
            && char_input[i + 2 * row_len + 2] == 'A'
            && char_input[i + 3 * row_len + 3] == 'S'
        {
            xmas_count += 1;
        }

        if i + 3 * row_len - 3 < input_len
            && char_input[i + 1 * row_len - 1] == 'M'
            && char_input[i + 2 * row_len - 2] == 'A'
            && char_input[i + 3 * row_len - 3] == 'S'
        {
            xmas_count += 1;
        }

        if i + 3 >= 3 * row_len
            && char_input[i - 1 * row_len + 1] == 'M'
            && char_input[i - 2 * row_len + 2] == 'A'
            && char_input[i - 3 * row_len + 3] == 'S'
        {
            xmas_count += 1;
        }

        if i >= 3 * row_len + 3
            && char_input[i - 1 * row_len - 1] == 'M'
            && char_input[i - 2 * row_len - 2] == 'A'
            && char_input[i - 3 * row_len - 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    xmas_count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let row_len = input.find('\n').unwrap() + 1;

    let char_input = input.chars().collect::<Vec<char>>();
    let mut x_mas_count = 0;

    for i in row_len + 1..input.len() - row_len - 1 {
        if char_input[i] != 'A' {
            continue;
        }

        // It looks like only X-shape count and not +-shape
        if ((char_input[i + row_len + 1] == 'S' && char_input[i - row_len - 1] == 'M')
            || (char_input[i + row_len + 1] == 'M' && char_input[i - row_len - 1] == 'S'))
            && ((char_input[i + row_len - 1] == 'S' && char_input[i - row_len + 1] == 'M')
                || (char_input[i + row_len - 1] == 'M' && char_input[i - row_len + 1] == 'S'))
        {
            x_mas_count += 1;
        }
    }
    x_mas_count
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            ),
            4
        );

        assert_eq!(
            part1(
                "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"
            ),
            18
        );

        assert_eq!(
            part1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            18
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day4.txt").expect("")),
            2646
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "M.S
.A.
M.S"
            ),
            1
        );

        assert_eq!(
            part2(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            9
        );

        assert!(part2(&fs::read_to_string("input/2024/day4.txt").expect("")) < 2021);
    }
}
