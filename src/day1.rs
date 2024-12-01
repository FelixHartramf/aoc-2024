use std::iter::zip;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut left_nums: Vec<u64> = vec![];
    let mut right_nums: Vec<u64> = vec![];
    for line in input.lines(){
        let num_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_nums.push(num_str[0].parse::<u64>().unwrap());
        right_nums.push(num_str[1].parse::<u64>().unwrap());
    }

    left_nums.sort();
    right_nums.sort();

    let mut diff = 0;
    for (l, r) in zip(left_nums, right_nums) {
        diff += l.abs_diff(r);
    }

    diff
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let mut left_nums: Vec<u64> = vec![];
    let mut right_nums: Vec<u64> = vec![];
    for line in input.lines(){
        let num_str = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left_nums.push(num_str[0].parse::<u64>().unwrap());
        right_nums.push(num_str[1].parse::<u64>().unwrap());
    }

    let mut similarity_score = 0;
    for num in left_nums {
        let appearnce_count = right_nums.iter().filter(|&n| *n == num).count();

        similarity_score += appearnce_count as u64 * num;
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(
"3   4
4   3
2   5
1   3
3   9
3   3"
), 11);
    }


    #[test]
    fn sample1_part2() {
        assert_eq!(part2(
"3   4
4   3
2   5
1   3
3   9
3   3"
), 31);
    }
}