use std::iter::zip;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left_nums: Vec<u32> = vec![];
    let mut right_nums: Vec<u32> = vec![];
    for line in input.lines() {
        let num_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_nums.push(num_str[0].parse::<u32>().unwrap());
        right_nums.push(num_str[1].parse::<u32>().unwrap());
    }

    left_nums.sort();
    right_nums.sort();

    zip(left_nums, right_nums).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left_nums: Vec<u32> = vec![];
    let mut right_nums: Vec<u32> = vec![];
    for line in input.lines() {
        let num_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_nums.push(num_str[0].parse::<u32>().unwrap());
        right_nums.push(num_str[1].parse::<u32>().unwrap());
    }

    left_nums
        .iter()
        .map(|l| right_nums.iter().filter(|&r| r == l).count() as u32 * l)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }

    #[test]
    fn real1() {
        assert_eq!(
            part1(
                &fs::read_to_string("input/2024/day1.txt")
                    .expect("Should have been able to read the file")
            ),
            1320851
        );
    }

    #[test]
    fn sample1_part2() {
        assert_eq!(
            part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        );
    }

    #[test]
    fn real2() {
        assert_eq!(
            part2(
                &fs::read_to_string("input/2024/day1.txt")
                    .expect("Should have been able to read the file")
            ),
            26859182
        );
    }
}
