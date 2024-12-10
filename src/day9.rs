use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let v = input.as_bytes();
    let mut lp = 0;
    let mut l_appearnce = v[0] as u8 - 48;
    let mut rp = v.len() - 2;
    let mut r_appearnce = v[rp] as u8 - 48;

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
                    r_appearnce = v[rp] as u8 - 48;
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
    let v = input.as_bytes();

    let mut files: Vec<(u64, u8, bool)> = vec![]; // file_id, size, file?

    for (value, i) in v[..v.len() - 1].iter().zip(0..) {
        files.push((i / 2, *value as u8 - 48, i % 2 == 0));
    }

    'bigloop: for i in (0..files.len()).rev() {
        if !files[i].2 {
            continue;
        }

        for k in 0..i {
            if files[k].2 {
                continue;
            }

            if files[k].1 < files[i].1 {
                continue;
            }
            if files[k].1 == files[i].1 {
                files.swap(i, k);
            }

            if files[k].1 > files[i].1 {
                files[k].1 -= files[i].1;
                files.insert(k, files[i]);
                files[i + 1].2 = false;
            }
            continue 'bigloop;
        }
    }

    let mut sum = 0;
    let mut i = 0;
    for file in files.iter_mut() {
        if file.2 {
            while file.1 > 0 {
                sum += file.0 * i;
                file.1 -= 1;
                i += 1;
            }
        } else {
            i += file.1 as u64;
        }
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
                "2333133121414131402
"
            ),
            1928
        );

        assert_eq!(
            part1(&fs::read_to_string("input/2024/day9.txt").expect("")),
            6330095022244
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "2333133121414131402
"
            ),
            2858
        );

        assert!(part2(&fs::read_to_string("input/2024/day9.txt").expect("")) > 6358495490494);
        assert!(part2(&fs::read_to_string("input/2024/day9.txt").expect("")) < 63584954904941);
        assert_eq!(part2(
            &fs::read_to_string("input/2024/day9.txt").expect("")
       ),
6359491814941);
    }
}
