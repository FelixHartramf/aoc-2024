use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut tokens = 0;
    for game in input.split("\n\n") {
        for (_, [x_a_str, y_a_str, x_b_str, y_b_str, x_target_str, y_target_str]) in
            re.captures_iter(game).map(|c| c.extract())
        {
            tokens += get_num_tokens(
                x_a_str.parse().unwrap(),
                y_a_str.parse().unwrap(),
                x_b_str.parse().unwrap(),
                y_b_str.parse().unwrap(),
                x_target_str.parse().unwrap(),
                y_target_str.parse().unwrap(),
            );
        }
    }

    tokens
}

pub fn get_num_tokens(x_a: i64, y_a: i64, x_b: i64, y_b: i64, x_t: i64, y_t: i64) -> i64 {

    // It's just
    // a*x_a + b * x_b = x_t
    // a*y_a + b * y_b = y_t
    // reordered to a and b
    let b = (x_t * y_a - x_a * y_t) / (y_a * x_b - y_b * x_a);
    let a = (y_t - b * y_b) / y_a;

    if a * x_a + b * x_b == x_t && a * y_a + b * y_b == y_t {
        return 3 * a + b;
    }

    0
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut tokens = 0;
    for game in input.split("\n\n") {
        for (_, [x_a_str, y_a_str, x_b_str, y_b_str, x_target_str, y_target_str]) in
            re.captures_iter(game).map(|c| c.extract())
        {
            tokens += get_num_tokens(
                x_a_str.parse().unwrap(),
                y_a_str.parse().unwrap(),
                x_b_str.parse().unwrap(),
                y_b_str.parse().unwrap(),
                x_target_str.parse::<i64>().unwrap() + 10000000000000,
                y_target_str.parse::<i64>().unwrap() + 10000000000000,
            );
        }
    }

    tokens
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            ),
            480
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day13.txt").expect("")),
            29187
        );
    }

    #[test]
    fn part2_test() {
        assert!(
            part2(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            ) > part1(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            )
        );

        assert!(
            part2(&fs::read_to_string("input/2024/day13.txt").expect(""))>
            part1(&fs::read_to_string("input/2024/day13.txt").expect(""))
        );

        assert_eq!(
            part2(&fs::read_to_string("input/2024/day13.txt").expect("")),
            99968222587852
        );
    }
}
