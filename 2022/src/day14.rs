use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day14.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> HashSet<Point> {
    data.lines()
        .flat_map(|line| {
            // parse the point coordinates
            let points: Vec<Point> = line
                .split(" -> ")
                .map(|point| {
                    let (sx, sy) = point.split_once(',').expect("point-string");
                    Point {
                        x: sx.parse().expect("coord-string"),
                        y: sy.parse().expect("coord-string"),
                    }
                })
                .collect();

            // expand the point ranges into a set of points
            points
                .iter()
                .zip(points.iter().skip(1))
                .flat_map(|(p1, p2)| {
                    let (lx, rx) = (min(p1.x, p2.x), max(p1.x, p2.x));
                    let (ly, ry) = (min(p1.y, p2.y), max(p1.y, p2.y));
                    (lx..=rx).flat_map(move |x| (ly..=ry).map(move |y| Point { x, y }))
                })
                .collect::<HashSet<_>>()
        })
        .collect()
}

fn part1(rocks: &HashSet<Point>) -> u32 {
    let mut settled = rocks.clone();
    let my = rocks.iter().map(|p| p.y).max().expect("some-rocks");

    // continue dropping new grains of sand until they fall off the bottom
    loop {
        // drop the current grain until it settles or falls off the bottom
        let mut p = Point { x: 500, y: 0 };
        loop {
            // attempt to find a lower point for the sand
            // . first try straight below
            // . next, diagonally to the left
            // . finally diagonally to the right
            let mut q = Point { x: p.x, y: p.y + 1 };
            if settled.contains(&q) {
                q.x -= 1;
                if settled.contains(&q) {
                    q.x += 2;
                    if settled.contains(&q) {
                        // no lower point was found, settle the grain
                        settled.insert(p);
                        break;
                    }
                }
            }
            p = q;

            // if we fall off the bottom, we are done
            if p.y > my {
                return (settled.len() - rocks.len()) as u32;
            }
        }
    }
}

fn part2(rocks: &HashSet<Point>) -> u32 {
    let mut settled = rocks.clone();
    let my = rocks.iter().map(|p| p.y).max().expect("some-rocks");

    // continue dropping new grains of sand until they settle on the bottom
    loop {
        // drop the current grain until it settles
        let mut p = Point { x: 500, y: 0 };
        loop {
            // attempt to find a lower point for the sand
            // . first try straight below
            // . next, diagonally to the left
            // . finally diagonally to the right
            let mut q = Point { x: p.x, y: p.y + 1 };
            if settled.contains(&q) {
                q.x -= 1;
                if settled.contains(&q) {
                    q.x += 2;
                    if settled.contains(&q) {
                        // no lower point was found, settle the grain
                        settled.insert(p);
                        break;
                    }
                }
            }
            p = q;

            // if we reach the bottom, settle the sand
            if p.y > my {
                settled.insert(p);
                break;
            }
        }

        // if the grain didn't move at all, the source is blocked, so we are done
        if p.y == 0 {
            return (settled.len() - rocks.len()) as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        498,4 -> 498,6 -> 496,6\n\
        503,4 -> 502,4 -> 502,9 -> 494,9\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 93);
    }
}
