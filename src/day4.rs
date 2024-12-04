use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut xmas_count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] != 'X' {
                continue;
            }

            if x+3 < lines[y].len() 
            && lines[y][x+1] == 'M'
            && lines[y][x+2] == 'A'
            && lines[y][x+3] == 'S' {
                xmas_count += 1;
            }

            if x >= 3
            && lines[y][x-1] == 'M'
            && lines[y][x-2] == 'A'
            && lines[y][x-3] == 'S' {
                xmas_count += 1;
            }

            if y+3 < lines.len() 
            && lines[y+1][x] == 'M'
            && lines[y+2][x] == 'A'
            && lines[y+3][x] == 'S' {
                xmas_count += 1;
            }

            if y >= 3
            && lines[y-1][x] == 'M'
            && lines[y-2][x] == 'A'
            && lines[y-3][x] == 'S' {
                xmas_count += 1;
            }

            if y+3 < lines.len()
            &&  x+3 < lines[y+3].len() 
            && lines[y+1][x+1] == 'M'
            && lines[y+2][x+2] == 'A'
            && lines[y+3][x+3] == 'S' {
                xmas_count += 1;
            }

            if x >= 3
            && y+3 < lines.len() 
            && lines[y+1][x-1] == 'M'
            && lines[y+2][x-2] == 'A'
            && lines[y+3][x-3] == 'S' {
                xmas_count += 1;
            }

            if x+3 < lines[y].len() 
            && y >= 3
            && lines[y-1][x+1] == 'M'
            && lines[y-2][x+2] == 'A'
            && lines[y-3][x+3] == 'S' {
                xmas_count += 1;
            }

            if x >= 3
            && y >= 3
            && lines[y-1][x-1] == 'M'
            && lines[y-2][x-2] == 'A'
            && lines[y-3][x-3] == 'S' {
                xmas_count += 1;
            }

        }
    }

    xmas_count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let row_len = input.find('\n').unwrap() +1;
    
    let char_input = input.chars().collect::<Vec<char>>();
    let mut x_mas_count = 0;

    for i in row_len+1..input.len()-row_len-1{
        
        if char_input[i] != 'A'{
            continue;
        }
        
        // It looks like only X-shape count and not +-shape
        if ((char_input[i+row_len+1] == 'S' && char_input[i-row_len-1] == 'M')
        || (char_input[i+row_len+1] == 'M' && char_input[i-row_len-1] == 'S'))
        && ((char_input[i+row_len-1] == 'S' && char_input[i-row_len+1] == 'M')
        || (char_input[i+row_len-1] == 'M' && char_input[i-row_len+1] == 'S'))
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
            part1(
                &fs::read_to_string("input/2024/day4.txt").expect("")
            ), 2646
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

        assert!(
            part2(
                &fs::read_to_string("input/2024/day4.txt").expect("")
            )< 2021
        );
    }
}
