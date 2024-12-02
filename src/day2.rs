use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut safe_reports = 0;
    'line_loop: for line in input.lines() {
        let mut num_iter = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        let first_num = num_iter.next().unwrap();
        let mut last_num = num_iter.next().unwrap();
        
        if first_num == last_num || first_num.abs_diff(last_num) > 3 {
            continue;
        }
        
        let increasing = first_num < last_num;

        for num in num_iter {
            if (!increasing && num > last_num)
            || (increasing && num < last_num )
            || num == last_num
            || num.abs_diff(last_num) > 3 
             {
                continue 'line_loop;
            }

            last_num = num;
        }

        safe_reports += 1;
    }

    safe_reports
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_reports = 0;
    'line_loop: for line in input.lines() {
        let mut error = false;
        let mut num_iter = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        let first_num = num_iter.next().unwrap();
        let mut last_num = num_iter.next().unwrap();

        if first_num == last_num || first_num.abs_diff(last_num) > 3 {
            error = true;
            last_num = num_iter.next().unwrap();
        }

        let increasing = first_num < last_num;

        for num in num_iter {
            if (!increasing && num > last_num)
            || (increasing && num < last_num )
            || num == last_num
            || num.abs_diff(last_num) > 3 
             {
                
                if error {
                    continue 'line_loop;
                }
                error = true;
                continue;
            }

            last_num = num;
        }
        
        safe_reports += 1;
        
    }

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            2
        );
        
        assert_eq!(
            part1(
                &fs::read_to_string("input/2024/day2.txt")
                    .expect("Should have been able to read the file")
            ),
            402
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            4
        );

        assert_eq!(
            part2(
                &fs::read_to_string("input/2024/day2.txt")
                    .expect("Should have been able to read the file")
            ),
            455
        );
    }

}
