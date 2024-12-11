use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sum = 0;
    let mut hm  = HashMap::new();
    for num in nums {
        sum += mul_num(num, 25, &mut hm);
    }

    sum
}

pub fn process_num(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }

    let dec_len = (num as f64).log10() as u32 + 1;
    if dec_len % 2 == 0 {
        return vec![
            num / (10_u64.pow(dec_len / 2)),
            num % (10_u64.pow(dec_len / 2)),
        ];
    }

    vec![num * 2024]
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sum = 0;
    let mut hm  = HashMap::new();
    for num in nums {
        sum += mul_num(num, 75, &mut hm);
    }

    sum
}

pub fn mul_num(num: u64, steps_to_take: u64, hm: &mut HashMap<(u64, u64), u64>) -> u64{
    if steps_to_take == 0{
        return 1;
    }
    if hm.contains_key(&(num, steps_to_take)){
        return *hm.get(&(num, steps_to_take)).unwrap();
    }
    let mut sum = 0;
    for n in process_num(num){
       let v = mul_num(n, steps_to_take-1, hm); 
        hm.insert((n, steps_to_take-1), v);
       sum += v;
    }
    

    sum
}

#[cfg(test)]
mod tests {

    use super::{part1, part2, process_num};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(process_num(125), vec![253000]);
        assert_eq!(process_num(17), vec![1, 7]);
        assert_eq!(process_num(253000), vec![253, 0]);
        assert_eq!(part1("125 17"), 55312);

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
