use std::{collections::HashMap, iter::zip};

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left_nums: Vec<u32> = vec![];
    let mut right_nums: Vec<u32> = vec![];
    for line in input.lines() {
        let mut num_str_iter = line.split_ascii_whitespace();
        left_nums.push(num_str_iter.next().unwrap().parse::<u32>().unwrap());
        right_nums.push(num_str_iter.next().unwrap().parse::<u32>().unwrap());
    }

    left_nums.sort();
    right_nums.sort();

    zip(left_nums, right_nums).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left_nums: Vec<u32> = vec![];

    // I tried using a let mut nums: HashMap<u32, (u32, u32)> but it was nearly 30% slower
    let mut right_nums: HashMap<u32, u32> = HashMap::new();
    
    for line in input.lines() {
        let mut num_str_iter = line.split_ascii_whitespace();
        left_nums.push(num_str_iter.next().unwrap().parse::<u32>().unwrap());        
        
        let right_num = num_str_iter.next().unwrap().parse::<u32>().unwrap();
        right_nums.entry(right_num).and_modify(|counter| *counter += 1).or_insert(1);
    }

    left_nums
        .iter()
        .map(|l| right_nums.get(l).unwrap_or(&0) * l)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
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
        
        assert_eq!(
            part1(
                &fs::read_to_string("input/2024/day1.txt")
                    .expect("Should have been able to read the file")
            ),
            1320851
        );
    }

    #[test]
    fn part2_test() {
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
        
        assert_eq!(
            part2(
                &fs::read_to_string("input/2024/day1.txt")
                    .expect("Should have been able to read the file")
            ),
            26859182
        );
    }
}
