use raoc2024::run;

type XMAS = u8;
const DEBUG: bool = false;

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
    if (i as i32) + dir.0 * 3 < 0 || (j as i32) + dir.1 * 3 < 0 {
        return false;
    }
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
    if DEBUG {
        eprintln!("found at {i},{j} dir {dir:?}");
    }
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

const GRID_SIZE: usize = 3;

fn check_grid_xmas(dat: &[Vec<XMAS>], x: usize, y: usize) -> bool {
    let rightdown = (dat[y][x], dat[y + 1][x + 1], dat[y + 2][x + 2]);
    let leftdown = (dat[y][x + 2], dat[y + 1][x + 1], dat[y + 2][x]);
    (rightdown == (1, 2, 3) || rightdown == (3, 2, 1))
        && (leftdown == (1, 2, 3) || leftdown == (3, 2, 1))
}

fn part2(s: &str) {
    let dat: Vec<Vec<XMAS>> = s
        .lines()
        .map(|l| l.chars().map(from_char).collect())
        .collect();
    let mut c = 0;
    for y in 0..dat.len() + 1 - GRID_SIZE {
        for x in 0..dat[0].len() + 1 - GRID_SIZE {
            if check_grid_xmas(&dat, x, y) {
                c += 1;
            }
        }
    }
    println!("{c}")
}

fn main() {
    run(&[part1, part2]);
}
