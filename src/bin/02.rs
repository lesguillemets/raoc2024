use raoc2024::run;
use std::cmp::Ordering;

struct Report {
    dat: Vec<u32>,
}

impl Report {
    fn from_line(l: &str) -> Self {
        Report {
            dat: l.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        }
    }
    fn is_safe(&self) -> bool {
        let dir = self.dat[0].cmp(&self.dat[1]);
        for (&xi, &xj) in self.dat.iter().zip(self.dat.iter().skip(1)) {
            let diff = xi.abs_diff(xj);
            if xi.cmp(&xj) != dir || diff > 3 || diff == 0 {
                return false;
            }
        }
        true
    }
}

fn part1(s: &str) {
    let reports: Vec<Report> = s.lines().map(Report::from_line).collect();
    println!("{}", reports.iter().filter(|r| r.is_safe()).count());
}

fn part2(s: &str) {
    let safety: Vec<bool> = s
        .lines()
        .map(Report::from_line)
        .map(|r| r.is_safe())
        .collect();
    let mut count = 0;
    let mut current_streak: u32 = 0;
    for &safe in &safety {
        if safe {
            if current_streak > 1 {
                count += current_streak;
            }
            current_streak = 0;
        } else {
            current_streak += 1;
        }
    }
    let ans = safety.len() as u32 - count;
    println!("{ans}");
}

fn main() {
    run(&[part1, part2]);
}
