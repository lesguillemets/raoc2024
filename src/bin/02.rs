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
    fn is_safe_tor(&self) -> bool {
        let mut unsafe_count = 0;
        for (&xi, &xj) in self.dat.iter().zip(self.dat.iter().skip(1)) {
            let diff = xi.abs_diff(xj);
            if diff > 3 || diff == 0 {
                unsafe_count += 1;
                if unsafe_count >= 2 {
                    return false;
                }
            }
        }
        let (asc, desc): (Vec<_>, Vec<_>) = self
            .dat
            .iter()
            .zip(self.dat.iter().skip(1))
            .partition(|(xi, xj)| xi < xj);
        asc.len() <= 1 || desc.len() <= 1
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
    run(&[part1, part2]);
}
