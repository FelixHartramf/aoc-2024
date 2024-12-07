use std::usize;

use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (solution_str, numms_str) = l.split_once(": ").unwrap();
        let solution = solution_str.parse::<u64>().unwrap();
        let nums: Vec<u64> = numms_str
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        if solvable(&nums, 0, solution, 0) {
            sum += solution;
        }
    }

    sum
}

pub fn solvable(nums: &Vec<u64>, i: usize, wanted_solution: u64, current: u64) -> bool {

    if current > wanted_solution {
        return false;
    }
    
    if nums.len() == i {
        return current == wanted_solution;
    }

    if solvable(&nums, i + 1, wanted_solution, current + nums[i]) {
        return true;
    }

    if solvable(&nums, i + 1, wanted_solution, current * nums[i]) {
        return true;
    }

    false
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (solution_str, numms_str) = l.split_once(": ").unwrap();
        let solution = solution_str.parse::<u64>().unwrap();
        let nums: Vec<u64> = numms_str
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        // ToDo: schauen ob (plus bei allen ) größer als solution, dann keine solution
        if solvable2(&nums, 0, solution, 0) {
            sum += solution;
        }
    }

    sum
}

pub fn solvable2(nums: &Vec<u64>, i: usize, wanted_solution: u64, current: u64) -> bool {
    if current > wanted_solution {
        return false;
    }

    if nums.len() == i {
        return current == wanted_solution;
    }

    if solvable2(&nums, i + 1, wanted_solution, current + nums[i]) {
        return true;
    }

    if solvable2(&nums, i + 1, wanted_solution, current * nums[i]) {
        return true;
    }

    if solvable2(
        &nums,
        i + 1,
        wanted_solution,
        current *  (10_u64.pow((nums[i] as f64).log10() as u32 + 1)) + nums[i],
    ) {
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
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            3749
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day7.txt").expect("")),
            6392012777720
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            11387
        );

        assert_eq!(part2(&fs::read_to_string("input/2024/day7.txt").expect("")), 61561126043536);
    }
}
