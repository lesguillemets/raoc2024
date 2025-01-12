use raoc2024::run;

type XMAS = u8;
const DEBUG: bool = true;

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];
fn from_char(c: char) -> XMAS {
    match c {
        'X' => 0,
        'M' => 1,
        'A' => 2,
        'S' => 3,
        _ => panic!("unexpected char {}", c),
    }
}

fn count_Xmas_at(dat: &[Vec<XMAS>], &(i, j): &(usize, usize)) -> u32 {
    if dat[i][j] != 0 {
        return 0;
    }
    let mut count = 0;
    for dir in &DIRS {
        if check_dir(dat, &(i, j), dir) {
            count += 1;
        }
    }
    count
}

fn add_dir(i: usize, v: i32) -> usize {
    ((i as i32) + v).max(0) as usize
}

fn check_dir(dat: &[Vec<XMAS>], &(i, j): &(usize, usize), &dir: &(i32, i32)) -> bool {
    for dist in 0..=3 {
        if let Some(&n) = dat
            .get(add_dir(i, dir.0 * dist))
            .and_then(|l| l.get(add_dir(j, dir.1 * dist)))
        {
            if n as i32 != dist {
                return false;
            } else if DEBUG {
                let ni = add_dir(i, dir.0 * dist);
                let nj = add_dir(j, dir.1 * dist);
                eprintln!("oK: ({i},{j}): {dist} for {dir:?}, at {n}@({ni},{nj})");
            }
        } else {
            return false;
        }
    }
    eprintln!("found at {i},{j} dir {dir:?}");
    true
}

fn part1(s: &str) {
    let dat: Vec<Vec<XMAS>> = s
        .lines()
        .map(|l| l.chars().map(from_char).collect())
        .collect();
    let mut c = 0;
    for i in 0..dat.len() {
        for j in 0..dat[0].len() {
            c += count_Xmas_at(&dat, &(i, j));
        }
    }
    println!("{c}");
}

fn part2(s: &str) {
    println!("part 2");
}

fn main() {
    run(&[part1, part2]);
}
