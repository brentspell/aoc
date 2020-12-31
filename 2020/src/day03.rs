use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day03.txt").unwrap();
    let lines: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    // part 1
    let mut result = 0;
    let mut x = 0;
    for line in &lines {
        if line[x] == '#' {
            result += 1;
        }
        x = (x + 3) % line.len();
    }
    println!("part 1: {}", result);

    // part 2
    let mut result = 1_i64;
    for &(dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0;
        let mut y = 0;
        let mut t = 0;
        while y < lines.len() {
            if lines[y][x] == '#' {
                t += 1;
            }
            x = (x + dx) % lines[y].len();
            y += dy;
        }
        result *= t;
    }
    println!("part 2: {}", result);
}
