use std::usize;

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut meow = input.split("\n\n");

    let mut rules: [Vec<usize>; 100] = vec![Vec::new(); 100].try_into().expect("static");

    for rule_line in meow.next().unwrap().lines() {
        let mut rule_nums = rule_line.split("|").map(|s| s.parse::<usize>().unwrap());
        let first = rule_nums.next().unwrap();
        let sec = rule_nums.next().unwrap();

        rules[sec].push(first);
    }

    let mut sum = 0;
    'nums_loop: for line in meow.next().unwrap().lines() {
        let nums = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for i in (0..nums.len()).rev() {
            let num = nums[i];

            for before_num in &rules[num] {
                let pos = nums.iter().position(|n| n == before_num);

                match pos {
                    None => continue,
                    Some(p) => {
                        if p > i {
                            continue 'nums_loop;
                        }
                    }
                }
            }
        }

        sum += nums[nums.len() / 2];
    }
    sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut meow = input.split("\n\n");

    let mut rules: [Vec<usize>; 100] = vec![Vec::new(); 100].try_into().expect("static");

    for rule_line in meow.next().unwrap().lines() {
        let mut rule_nums = rule_line.split("|").map(|s| s.parse::<usize>().unwrap());
        let first = rule_nums.next().unwrap();
        let sec = rule_nums.next().unwrap();

        rules[sec].push(first);
    }

    let mut sum = 0;
    for line in meow.next().unwrap().lines() {
        let mut nums = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut complete_checked = false;
        let mut correct = true;
        'complete_check_loop: while !complete_checked {
            for i in (0..nums.len()).rev() {
                let num = nums[i];

                for before_num in &rules[num] {
                    let pos = nums.iter().position(|n| n == before_num);

                    match pos {
                        None => continue,
                        Some(p) => {
                            if p > i {
                                nums.swap(i, p);
                                correct = false;
                                continue 'complete_check_loop;
                            }
                        }
                    }
                }
            }

            complete_checked = true;
        }

        if correct {
            continue;
        }

        sum += nums[nums.len() / 2];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day5.txt").expect("")),
            5509
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            123
        );

        assert_eq!(
            part2(&fs::read_to_string("input/2024/day5.txt").expect("")),
            4407
        );
    }
}
