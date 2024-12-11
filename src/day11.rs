use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut hm  = HashMap::with_capacity(3100);
    input
        .split_ascii_whitespace()
        .map(|s| mul_num(s.parse::<u64>().unwrap(), 25, &mut hm))
        .sum()
}

pub fn mul_num(num: u64, steps_to_take: u64, hm: &mut HashMap<(u64, u64), u64>) -> u64{
    if steps_to_take == 0{
        return 1;
    }
    if hm.contains_key(&(num, steps_to_take)){
        return *hm.get(&(num, steps_to_take)).unwrap();
    }
    let sum = match process_num(num) {
        Mul::One(p) => mul_num(p, steps_to_take-1, hm),
        Mul::Two(p1, p2) => mul_num(p1, steps_to_take-1, hm) + mul_num(p2, steps_to_take-1, hm),
    };
    
    hm.insert((num, steps_to_take), sum);

    sum
}

#[derive(Debug)]
pub enum Mul {
    One(u64),
    Two(u64,u64)
}
pub fn process_num(num: u64) -> Mul{
    if num == 0 {
        return Mul::One(1);
    }

    let dec_len = (num as f64).log10() as u32 + 1;
    if dec_len % 2 == 0 {
        return Mul::Two(
            num / (10_u64.pow(dec_len / 2)),
            num % (10_u64.pow(dec_len / 2)),
    );
    }

    Mul::One(num * 2024)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut hm  = HashMap::with_capacity(125000);
    input
        .split_ascii_whitespace()
        .map(|s| mul_num(s.parse::<u64>().unwrap(), 75, &mut hm))
        .sum()
}



#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&fs::read_to_string("input/2024/day11.txt").expect("")),
            202019
        );
    }

    #[test]
    fn part2_test() {
        assert!(part2(&fs::read_to_string("input/2024/day11.txt").expect("")) > part1(&fs::read_to_string("input/2024/day11.txt").expect("")));
        assert_eq!(part2(&fs::read_to_string("input/2024/day11.txt").expect("")), 239321955280205);
    }
}
