use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day12.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(char, i32)> = lines.iter().map(|l| parse(l)).collect();

    println!("part 1: {}", part1(&inputs));
    println!("part 2: {}", part2(&inputs));
}

fn part1(inputs: &[(char, i32)]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut ang: f32 = 0.0;
    for &(d, m) in inputs {
        match d {
            'N' => pos.1 += m,
            'S' => pos.1 -= m,
            'E' => pos.0 += m,
            'W' => pos.0 -= m,
            'L' => ang += (m as f32) / 180.0 * std::f32::consts::PI,
            'R' => ang -= (m as f32) / 180.0 * std::f32::consts::PI,
            'F' => pos = (pos.0 + m * ang.cos() as i32, pos.1 + m * ang.sin() as i32),
            _ => panic!(),
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn part2(inputs: &[(char, i32)]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut way: (i32, i32) = (10, 1);
    for &(d, m) in inputs {
        match d {
            'N' => way.1 += m,
            'S' => way.1 -= m,
            'E' => way.0 += m,
            'W' => way.0 -= m,
            'L' => way = rotate(way, m),
            'R' => way = rotate(way, -m),
            'F' => pos = (pos.0 + m * way.0, pos.1 + m * way.1),
            _ => panic!(),
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn rotate((x, y): (i32, i32), angle: i32) -> (i32, i32) {
    let r = ((x as f32).powi(2) + (y as f32).powi(2)).sqrt();
    let a = (y as f32).atan2(x as f32) + (angle as f32) / 180.0 * std::f32::consts::PI;
    ((r * a.cos()).round() as i32, (r * a.sin()).round() as i32)
}

fn parse(line: &str) -> (char, i32) {
    (line.chars().next().unwrap(), line[1..].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = ["F10", "N3", "F7", "R90", "F11"];
        let inputs: Vec<(char, i32)> = lines.iter().map(|l| parse(l)).collect();

        assert_eq!(part1(&inputs), 25);
        assert_eq!(part2(&inputs), 286);
    }
}
