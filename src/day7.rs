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

        for i in 0..2_usize.pow(nums.len() as u32) {
            // ToDo: schauen ob (plus bei allen ) größer als solution, dann keine solution
            if correct(&nums, i, solution) {
                sum += solution;
                break;
            }
        }
    }

    sum
}

#[inline]
pub fn correct(nums: &Vec<u64>, operators: usize, wanted_solution: u64) -> bool {
    let mut solution = nums[0];

    for (i, num) in nums[1..].into_iter().enumerate() {
        if (operators >> i) & 1 == 1 {
            solution *= num;
        } else {
            solution += num;
        }

        if solution > wanted_solution {
            return false;
        }
    }

    solution == wanted_solution
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
        if solvable(&nums, 0, solution) {
            sum += solution;
            break;
        }
    }

    sum
}

pub fn solvable(nums: &Vec<u64>, i: usize, wanted_solution: u64) -> bool {
    if nums.len()-1 == i {
        return nums[i] == wanted_solution;
    }

    if solvable(&nums, i+1, wanted_solution - nums[i]) {
        return true;
    }

    if solvable(&nums, i+1, wanted_solution / nums[i]) {
        return true;
    }

    // || operator
    let mut solution = wanted_solution - nums[0];
    let zeros = (nums[0] as f64).log10() as u32;
    let mut solution_zeros = 0;
    while solution % 10 == 0 {
        solution /= 10;
        solution_zeros += 1;
    }
    if solution_zeros != zeros + 1 {
        return false;
    }
    if solvable(&nums, i, solution) {
        return true;
    }

    false
}

pub fn correct2(nums: &Vec<u64>, mut operators: usize, wanted_solution: u64) -> bool {
    let mut solution = nums[0];

    for num in nums[1..].into_iter() {
        match operators % 3 {
            0 => solution *= num,
            1 => solution += num,
            2 => {
                solution *= 10 * ((*num as f64).log10() as u64 + 1);
                solution += num;
            }
            _ => panic!(),
        }

        if solution > wanted_solution {
            return false;
        }

        operators /= 3;
    }

    solution == wanted_solution
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

        assert!(dbg!(part2(&fs::read_to_string("input/2024/day7.txt").expect(""))) > 6417120397561);
    }
}
