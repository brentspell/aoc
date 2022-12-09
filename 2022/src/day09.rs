use std::cmp::max;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Move {
    d: Vec2,
    n: u32,
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day09.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Move> {
    data.lines()
        .map(|l| {
            let (d, n) = l.split_once(' ').expect("space");
            Move {
                d: match d {
                    "L" => Vec2::new(-1, 0),
                    "R" => Vec2::new(1, 0),
                    "D" => Vec2::new(0, -1),
                    "U" => Vec2::new(0, 1),
                    _ => panic!("invalid-dir"),
                },
                n: n.parse().expect("number"),
            }
        })
        .collect()
}

fn part1(moves: &[Move]) -> usize {
    let mut h = Vec2::ORIGIN;
    let mut t = Vec2::ORIGIN;
    let mut v = HashSet::from([t]);
    for m in moves {
        for _ in 0..m.n {
            h += m.d;
            let d = h - t;
            if d.infnorm() > 1 {
                t += d.signum();
                v.insert(t);
            }
        }
    }
    v.len()
}

fn part2(moves: &[Move]) -> usize {
    let mut p = vec![Vec2::ORIGIN; 10];
    let mut v = HashSet::from([p[p.len() - 1]]);
    for m in moves {
        for _ in 0..m.n {
            p[0] += m.d;
            for i in 1..p.len() {
                let d = p[i - 1] - p[i];
                if d.infnorm() > 1 {
                    p[i] += d.signum();
                    if i == p.len() - 1 {
                        v.insert(p[i]);
                    }
                }
            }
        }
    }
    v.len()
}

impl Vec2 {
    const ORIGIN: Self = Self { x: 0, y: 0 };

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn infnorm(&self) -> i32 {
        max(self.x.abs(), self.y.abs())
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = "\
        R 4\n\
        U 4\n\
        L 3\n\
        D 1\n\
        R 4\n\
        D 1\n\
        L 5\n\
        R 2\n\
    ";

    const DATA2: &str = "\
        R 5\n\
        U 8\n\
        L 8\n\
        D 3\n\
        R 17\n\
        D 10\n\
        L 25\n\
        U 20\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA1)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA1)), 1);
        assert_eq!(part2(&parse(DATA2)), 36);
    }
}
