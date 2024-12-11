use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut hm  = HashMap::with_capacity(3100);
    prepopulate_hm_part1(&mut hm);
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

pub fn prepopulate_hm_part1(hm: &mut HashMap<(u64, u64), u64>) {
    hm.insert((0, 15), 328);
hm.insert((0, 16), 418);
hm.insert((0, 17), 667);
hm.insert((0, 18), 1059);
hm.insert((0, 19), 1546);
hm.insert((0, 20), 2377);
hm.insert((0, 21), 3572);
hm.insert((0, 22), 5602);
hm.insert((0, 23), 8268);
hm.insert((0, 24), 12343);
hm.insert((1, 15), 418);
hm.insert((1, 16), 667);
hm.insert((1, 17), 1059);
hm.insert((1, 18), 1546);
hm.insert((1, 19), 2377);
hm.insert((1, 20), 3572);
hm.insert((1, 21), 5602);
hm.insert((1, 22), 8268);
hm.insert((1, 23), 12343);
hm.insert((1, 24), 19778);
hm.insert((2, 15), 414);
hm.insert((2, 16), 661);
hm.insert((2, 17), 977);
hm.insert((2, 18), 1501);
hm.insert((2, 19), 2270);
hm.insert((2, 20), 3381);
hm.insert((2, 21), 5463);
hm.insert((2, 22), 7921);
hm.insert((2, 23), 11819);
hm.insert((2, 24), 18712);
hm.insert((3, 15), 401);
hm.insert((3, 16), 642);
hm.insert((3, 17), 987);
hm.insert((3, 18), 1556);
hm.insert((3, 19), 2281);
hm.insert((3, 20), 3347);
hm.insert((3, 21), 5360);
hm.insert((3, 22), 7914);
hm.insert((3, 23), 12116);
hm.insert((3, 24), 18714);
hm.insert((4, 15), 390);
hm.insert((4, 16), 637);
hm.insert((4, 17), 951);
hm.insert((4, 18), 1541);
hm.insert((4, 19), 2182);
hm.insert((4, 20), 3204);
hm.insert((4, 21), 5280);
hm.insert((4, 22), 7721);
hm.insert((4, 23), 11820);
hm.insert((4, 24), 17957);
hm.insert((5, 15), 383);
hm.insert((5, 16), 597);
hm.insert((5, 17), 808);
hm.insert((5, 18), 1260);
hm.insert((5, 19), 1976);
hm.insert((5, 20), 3053);
hm.insert((5, 21), 4529);
hm.insert((5, 22), 6675);
hm.insert((5, 23), 10627);
hm.insert((5, 24), 15847);
hm.insert((6, 15), 401);
hm.insert((6, 16), 600);
hm.insert((6, 17), 871);
hm.insert((6, 18), 1431);
hm.insert((6, 19), 2033);
hm.insert((6, 20), 3193);
hm.insert((6, 21), 4917);
hm.insert((6, 22), 7052);
hm.insert((6, 23), 11371);
hm.insert((6, 24), 16815);
hm.insert((7, 15), 413);
hm.insert((7, 16), 602);
hm.insert((7, 17), 832);
hm.insert((7, 18), 1369);
hm.insert((7, 19), 2065);
hm.insert((7, 20), 3165);
hm.insert((7, 21), 4762);
hm.insert((7, 22), 6994);
hm.insert((7, 23), 11170);
hm.insert((7, 24), 16509);
hm.insert((8, 15), 393);
hm.insert((8, 16), 578);
hm.insert((8, 17), 812);
hm.insert((8, 18), 1322);
hm.insert((8, 19), 2011);
hm.insert((8, 20), 3034);
hm.insert((8, 21), 4580);
hm.insert((8, 22), 6798);
hm.insert((8, 23), 10738);
hm.insert((8, 24), 16018);
hm.insert((9, 15), 419);
hm.insert((9, 16), 586);
hm.insert((9, 17), 854);
hm.insert((9, 18), 1468);
hm.insert((9, 19), 2131);
hm.insert((9, 20), 3216);
hm.insert((9, 21), 4888);
hm.insert((9, 22), 7217);
hm.insert((9, 23), 11617);
hm.insert((9, 24), 17059);
hm.insert((10, 15), 528);
hm.insert((10, 16), 746);
hm.insert((10, 17), 1085);
hm.insert((10, 18), 1726);
hm.insert((10, 19), 2605);
hm.insert((10, 20), 3923);
hm.insert((10, 21), 5949);
hm.insert((10, 22), 9174);
hm.insert((10, 23), 13870);
hm.insert((10, 24), 20611);
hm.insert((11, 15), 656);
hm.insert((11, 16), 836);
hm.insert((11, 17), 1334);
hm.insert((11, 18), 2118);
hm.insert((11, 19), 3092);
hm.insert((11, 20), 4754);
hm.insert((11, 21), 7144);
hm.insert((11, 22), 11204);
hm.insert((11, 23), 16536);
hm.insert((11, 24), 24686);
hm.insert((12, 15), 623);
hm.insert((12, 16), 832);
hm.insert((12, 17), 1328);
hm.insert((12, 18), 2036);
hm.insert((12, 19), 3047);
hm.insert((12, 20), 4647);
hm.insert((12, 21), 6953);
hm.insert((12, 22), 11065);
hm.insert((12, 23), 16189);
hm.insert((12, 24), 24162);
hm.insert((13, 15), 622);
hm.insert((13, 16), 819);
hm.insert((13, 17), 1309);
hm.insert((13, 18), 2046);
hm.insert((13, 19), 3102);
hm.insert((13, 20), 4658);
hm.insert((13, 21), 6919);
hm.insert((13, 22), 10962);
hm.insert((13, 23), 16182);
hm.insert((13, 24), 24459);
hm.insert((14, 15), 597);
hm.insert((14, 16), 808);
hm.insert((14, 17), 1304);
hm.insert((14, 18), 2010);
hm.insert((14, 19), 3087);
hm.insert((14, 20), 4559);
hm.insert((14, 21), 6776);
hm.insert((14, 22), 10882);
hm.insert((14, 23), 15989);
hm.insert((14, 24), 24163);
hm.insert((15, 15), 551);
hm.insert((15, 16), 801);
hm.insert((15, 17), 1264);
hm.insert((15, 18), 1867);
hm.insert((15, 19), 2806);
hm.insert((15, 20), 4353);
hm.insert((15, 21), 6625);
hm.insert((15, 22), 10131);
hm.insert((15, 23), 14943);
hm.insert((15, 24), 22970);
hm.insert((16, 15), 578);
hm.insert((16, 16), 819);
hm.insert((16, 17), 1267);
hm.insert((16, 18), 1930);
hm.insert((16, 19), 2977);
hm.insert((16, 20), 4410);
hm.insert((16, 21), 6765);
hm.insert((16, 22), 10519);
hm.insert((16, 23), 15320);
hm.insert((16, 24), 23714);
hm.insert((17, 15), 570);
hm.insert((17, 16), 831);
hm.insert((17, 17), 1269);
hm.insert((17, 18), 1891);
hm.insert((17, 19), 2915);
hm.insert((17, 20), 4442);
hm.insert((17, 21), 6737);
hm.insert((17, 22), 10364);
hm.insert((17, 23), 15262);
hm.insert((17, 24), 23513);
hm.insert((18, 15), 567);
hm.insert((18, 16), 811);
hm.insert((18, 17), 1245);
hm.insert((18, 18), 1871);
hm.insert((18, 19), 2868);
hm.insert((18, 20), 4388);
hm.insert((18, 21), 6606);
hm.insert((18, 22), 10182);
hm.insert((18, 23), 15066);
hm.insert((18, 24), 23081);
hm.insert((19, 15), 590);
hm.insert((19, 16), 837);
hm.insert((19, 17), 1253);
hm.insert((19, 18), 1913);
hm.insert((19, 19), 3014);
hm.insert((19, 20), 4508);
hm.insert((19, 21), 6788);
hm.insert((19, 22), 10490);
hm.insert((19, 23), 15485);
hm.insert((19, 24), 23960);
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
