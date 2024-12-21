use regex::Regex;

use aoc_runner_derive::aoc;

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let re = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: (.*)",
    )
    .unwrap();

    let caps = re.captures(input).unwrap();
    let mut reg_a = caps
        .get(1)
        .map_or("", |m| m.as_str())
        .parse::<u32>()
        .unwrap();
    let mut reg_b = caps
        .get(2)
        .map_or("", |m| m.as_str())
        .parse::<u32>()
        .unwrap();
    let mut reg_c = caps
        .get(3)
        .map_or("", |m| m.as_str())
        .parse::<u32>()
        .unwrap();

    let programm = caps
        .get(4)
        .map_or("", |m| m.as_str())
        .split(",")
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut outs: Vec<u32> = vec![];
    let mut counter = 0;

    while counter < programm.len() {
        let op_code = programm[counter];
        let literal = programm[counter + 1];
        counter += 2;

        let combo_operant = match literal {
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            7 => panic!(),
            _ => literal,
        };

        match op_code {
            0 => reg_a = reg_a / 2_u32.pow(combo_operant),

            1 => reg_b = reg_b ^ literal,

            2 => reg_b = combo_operant & 0b111,
            3 => {
                if reg_a != 0 {
                    counter = literal as usize;
                    continue;
                }
            }
            4 => reg_b = reg_b ^ reg_c,

            5 => outs.push(combo_operant % 8),

            6 => reg_b = reg_a / 2_u32.pow(combo_operant),
            7 => reg_c = reg_a / 2_u32.pow(combo_operant),
            _ => panic!(),
        };
    }

    let mut s = String::new();
    for (n, i) in outs.iter().zip(0..) {
        s += &n.to_string();
        if i + 1 != outs.len() {
            s += ",";
        }
    }

    s
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: (.*)",
    )
    .unwrap();

    let caps = re.captures(input).unwrap();

    let original_reg_b = caps
        .get(2)
        .map_or("", |m| m.as_str())
        .parse::<u64>()
        .unwrap();
    let original_reg_c = caps
        .get(3)
        .map_or("", |m| m.as_str())
        .parse::<u64>()
        .unwrap();

    let programm = caps
        .get(4)
        .map_or("", |m| m.as_str())
        .split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    'a_search: for a in 0..u64::MAX {
        if a % 200__000_000 == 0{
            println!("day 17 part 2 is still running: {}", a);
        }
        
        let mut reg_a = a;
        let mut reg_b = original_reg_b;
        let mut reg_c = original_reg_c;

        let mut current_output = 0;
        let mut counter = 0;

        while counter < programm.len() {

            let op_code = programm[counter];
            let literal = programm[counter + 1];
            counter += 2;

            let combo_operant = match literal {
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                7 => panic!(),
                _ => literal,
            };

            match op_code {
                0 => reg_a = reg_a / 2_u64.pow(combo_operant as u32),

                1 => reg_b = reg_b ^ literal,

                2 => reg_b = combo_operant & 0b111,
                3 => {
                    if reg_a != 0 {
                        counter = literal as usize;
                        continue;
                    }
                }
                4 => reg_b = reg_b ^ reg_c,

                5 => {
                    if  programm[current_output] != combo_operant % 8 {
                        continue 'a_search;
                    }
                    current_output += 1;
                    
                },

                6 => reg_b = reg_a / 2_u64.pow(combo_operant as u32),
                7 => reg_c = reg_a / 2_u64.pow(combo_operant as u32),
                _ => panic!(),
            };
        }

        if current_output != programm.len() {
            continue;
        }
        
        return a;
    }

    0
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            ),
            "4,6,3,5,6,3,5,2,1,0"
        );
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day17.txt").expect("")),
            "4,1,5,3,1,5,3,5,7"
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "Register A: 729
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            ),
            117440
        );
    }
}
