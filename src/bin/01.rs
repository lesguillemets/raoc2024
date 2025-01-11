use raoc2024::run;

fn to_list(s: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first: Vec<u32> = vec![];
    let mut second: Vec<u32> = vec![];
    for line in s.split("\n").filter(|l| !l.is_empty()) {
        let cs: Vec<u32> = line.split("   ").map(|n| n.parse().unwrap()).collect();
        first.push(cs[0]);
        second.push(cs[1]);
    }
    (first, second)
}

fn part1(s: &str) {
    let (mut ns0, mut ns1) = to_list(s);
    ns0.sort();
    ns1.sort();
    let ans: u32 = ns0
        .iter()
        .zip(ns1.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum();
    println!("{ans}");
}

fn part2(s: &str) {
    println!("part 2");
}

fn main() {
    run(&[part1, part2]);
}
