use raoc2024::run_and_report_time;

use regex::Regex;

fn part1(s: &str) {
    // \d は ascii 外の数字にもマッチするみたいなので．
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let ans: u32 = re
        .captures_iter(s)
        .map(|cap| {
            let m0: u32 = cap[1].parse().unwrap();
            let m1: u32 = cap[2].parse().unwrap();
            m0 * m1
        })
        .sum();
    println!("{ans}");
}

fn part2(s: &str) {
    let re_mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let muls: Vec<(usize, u32)> = re_mul
        .captures_iter(s)
        .map(|cap| {
            let start: usize = cap.get(0).unwrap().start();
            let m0: u32 = cap[1].parse().unwrap();
            let m1: u32 = cap[2].parse().unwrap();
            (start, m0 * m1)
        })
        .collect();
    let re_do = Regex::new(r"(don't|do)\(\)").unwrap();
    let do_donts: Vec<(usize, bool)> = re_do
        .find_iter(s)
        .map(|m| {
            let flag: bool = m.as_str() == "do()";
            (m.start(), flag)
        })
        .collect();
    let mut current_flag: bool = true;
    let mut current_flag_loc: usize = 0;
    let mut total: u32 = 0;
    for (i, mul) in muls {
        // i の直前まで命令を読む
        while current_flag_loc < do_donts.len() && i > do_donts[current_flag_loc].0 {
            current_flag = do_donts[current_flag_loc].1;
            current_flag_loc += 1;
        }
        if current_flag {
            total += mul;
        }
    }
    println!("{total}");
}

fn main() {
    run_and_report_time(&[part1, part2]);
}
