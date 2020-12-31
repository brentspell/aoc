use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io};

type Point2 = (i32, i32);
type Point3 = (i32, i32, i32);
type Point4 = (i32, i32, i32, i32);

pub fn solve() {
    let file = fs::File::open("data/day17.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs = parse(&lines);

    println!("part 1: {}", part1(&inputs));
    println!("part 2: {}", part2(&inputs));
}

fn part1(inputs: &HashSet<Point2>) -> usize {
    let mut cubes: HashSet<Point3> = inputs.iter().map(|&(i, j)| (i, j, 0)).collect();
    for _ in 0..6 {
        let old = cubes.clone();
        let mut todo: Vec<Point3> = old.iter().copied().collect();
        let mut done: HashSet<Point3> = old.iter().copied().collect();
        while !todo.is_empty() {
            let p = todo.pop().unwrap();
            let active = old.contains(&p);
            let mut neighbors = 0;
            for u in -1..=1 {
                for v in -1..=1 {
                    for w in -1..=1 {
                        let q = (p.0 + u, p.1 + v, p.2 + w);
                        if q != p {
                            neighbors += old.contains(&q) as u32;
                            if active && !done.contains(&q) {
                                todo.push(q);
                                done.insert(q);
                            }
                        }
                    }
                }
            }
            if active && neighbors != 2 && neighbors != 3 {
                cubes.remove(&p);
            }
            if !active && neighbors == 3 {
                cubes.insert(p);
            }
        }
    }

    cubes.len()
}

fn part2(inputs: &HashSet<Point2>) -> usize {
    let mut cubes: HashSet<Point4> = inputs.iter().map(|&(i, j)| (i, j, 0, 0)).collect();
    for _ in 0..6 {
        let old = cubes.clone();
        let mut todo: Vec<Point4> = old.iter().copied().collect();
        let mut done: HashSet<Point4> = old.iter().copied().collect();
        while !todo.is_empty() {
            let p = todo.pop().unwrap();
            let active = old.contains(&p);
            let mut neighbors = 0;
            for u in -1..=1 {
                for v in -1..=1 {
                    for w in -1..=1 {
                        for x in -1..=1 {
                            let q = (p.0 + u, p.1 + v, p.2 + w, p.3 + x);
                            if q != p {
                                neighbors += old.contains(&q) as u32;
                                if active && !done.contains(&q) {
                                    todo.push(q);
                                    done.insert(q);
                                }
                            }
                        }
                    }
                }
            }
            if active && neighbors != 2 && neighbors != 3 {
                cubes.remove(&p);
            }
            if !active && neighbors == 3 {
                cubes.insert(p);
            }
        }
    }

    cubes.len()
}

fn parse(lines: &[String]) -> HashSet<Point2> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = vec![
            String::from(".#."),
            String::from("..#"),
            String::from("###"),
        ];
        let inputs = parse(&lines);

        assert_eq!(part1(&inputs), 112);
        assert_eq!(part2(&inputs), 848);
    }
}
