use raoc2024::run;
use std::convert::TryFrom;

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&self) -> Self {
        match *self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Dir::*;
        match c {
            '^' => Ok(Up),
            '>' => Ok(Right),
            'v' => Ok(Down),
            'V' => Ok(Down),
            '<' => Ok(Left),
            _ => Err("unmatch"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Point {
    Obstacle,
    Visited,
    Unvisited,
}
impl Point {
    fn from_char(c: &char) -> Self {
        match c {
            '#' => Point::Obstacle,
            '.' => Point::Unvisited,
            _ => Point::Visited,
        }
    }
}

type Field = Vec<Vec<Point>>;
type Loc = (usize, usize);

trait Fi {
    fn at(&self, l: &Loc) -> &Point;
    fn from_str(s: &str) -> Self;
}

impl Fi for Field {
    fn at(&self, &(x, y): &Loc) -> &Point {
        &self[y][x]
    }
    fn from_str(s: &str) -> Self {
        s.lines()
            .map(|l| l.chars().map(|c| Point::from_char(&c)).collect())
            .collect()
    }
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Guard {
    fn loc(&self) -> Loc {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct World {
    field: Field,
    guard: Guard,
}

impl World {
    fn from_str(s: &str) -> Self {
        let mut field = Field::from_str(s);
        let mut d: Dir = Dir::Up;
        let (mut x, mut y): (usize, usize) = (0, 0);
        for (j, l) in s.lines().enumerate() {
            for (i, c) in l.chars().enumerate() {
                if let Ok(dir) = Dir::try_from(c) {
                    (x, y) = (i, j);
                    d = dir;
                }
            }
        }
        World {
            field: field,
            guard: Guard { x: x, y: y, dir: d },
        }
    }
    fn step(&mut self) -> Option<Loc> {
        let (mut newx, mut newy) = self.guard.loc();
        match self.guard.dir {
            Dir::Up => {
                newy = self.guard.y.checked_sub(1)?;
            }
            Dir::Right => {
                newx = self.guard.x + 1;
                if newx >= self.field[0].len() {
                    // out of bounds
                    return None;
                }
            }
            Dir::Down => {
                newy = self.guard.y + 1;
                if newy >= self.field.len() {
                    // out of bounds
                    return None;
                }
            }
            Dir::Left => {
                newx = self.guard.x.checked_sub(1)?;
            }
        }
        // Ok, start moving
        if self.field.at(&(newx, newy)) == &Point::Obstacle {
            self.guard.dir = self.guard.dir.turn();
        } else {
            self.field[newy][newx] = Point::Visited;
            self.guard.x = newx;
            self.guard.y = newy;
        }
        Some(self.guard.loc())
    }
}

fn part1(s: &str) {
    let mut w = World::from_str(s);
    while let Some(l) = w.step() {
        println!("{l:?}");
    }
    let ans: usize = w
        .field
        .iter()
        .map(|l| l.iter().filter(|&c| *c == Point::Visited).count())
        .sum();

    println!("part 1, {ans:?}");
}

fn part2(s: &str) {
    println!("part 2");
}

fn main() {
    run(&[part1, part2]);
}
