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
    println!("part 2");
}

fn main() {
    run_and_report_time(&[part1, part2]);
}
