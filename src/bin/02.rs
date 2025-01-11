use raoc2024::run_and_report_time;
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
    fn is_safe_tor(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        for i in 0..self.dat.len() {
            let skipped: Vec<u32> = self
                .dat
                .iter()
                .take(i)
                .chain(self.dat.iter().skip(i + 1))
                .copied()
                .collect();
            if (Report { dat: skipped }).is_safe() {
                return true;
            }
        }
        false
    }
}

fn part1(s: &str) {
    let reports: Vec<Report> = s.lines().map(Report::from_line).collect();
    println!("{}", reports.iter().filter(|r| r.is_safe()).count());
}

fn part2(s: &str) {
    let reports: Vec<Report> = s.lines().map(Report::from_line).collect();
    println!("{}", reports.iter().filter(|r| r.is_safe_tor()).count());
}

fn main() {
    run_and_report_time(&[part1, part2]);
}
