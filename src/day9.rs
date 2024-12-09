use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let v = input.as_bytes();
    let mut lp = 0;
    let mut l_appearnce = v[0] as u8 - 48;
    let mut rp = v.len() - 2;
    let mut r_appearnce = v[rp] as u8  - 48;

    let mut sum: u64 = 0;
    let mut i = 0;
    while lp < rp {
        if lp % 2 == 0 {
            while l_appearnce != 0 {
                l_appearnce -= 1;
                sum += i * lp as u64 / 2;
                i += 1;
            }
        } else {
            loop {
                if l_appearnce == 0 {
                    break;
                }

                while r_appearnce == 0 {
                    rp -= 2;
                    r_appearnce = v[rp] as u8  - 48;
                }
                r_appearnce -= 1;
                l_appearnce -= 1;
                sum += i * rp as u64 / 2;
                i += 1;
            }
        }
        lp += 1;
        l_appearnce = v[lp] - 48;
    }
    while r_appearnce != 0 {
        r_appearnce -= 1;
        sum += i * rp as u64 / 2;
        i += 1;
    }

    sum
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
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
                "2333133121414131402
"
            ),
            1928
        );

        assert_eq!(part1(&fs::read_to_string("input/2024/day9.txt").expect("")), 6330095022244);
    }

    #[test]
    fn part2_test() {}
}
