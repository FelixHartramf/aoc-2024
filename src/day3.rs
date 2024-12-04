use aoc_runner_derive::aoc;

use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;

    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        result += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    result
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();

    let mut result = 0;
    let mut does = true;

    for (_, [s]) in re.captures_iter(input).map(|c| c.extract()) {
       if s == "do()" {
            does = true;
            continue;
       }

       if !does{
        continue;
       }

       if s == "don't()" {
        does = false;
        continue;
       }
       let i = s[4..s.len()-1].split(",").collect::<Vec<&str>>();
       
       result += i[0].parse::<u32>().unwrap() * i[1].parse::<u32>().unwrap();
       
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)))"), 161
        );
        
        assert_eq!(
            part1(
                &fs::read_to_string("input/2024/day3.txt")
                    .expect("Should have been able to read the file")
            ), 173419328
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );

        assert_eq!(
            part2(
                &fs::read_to_string("input/2024/day3.txt")
                    .expect("Should have been able to read the file")
            ), 90669332
        );
    }

}
