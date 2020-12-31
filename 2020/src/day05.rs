use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day05.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // part 1
    let result = lines.iter().map(|l| seat_id(&l)).max().unwrap();
    println!("part 1: {}", result);

    // part 2
    let set: HashSet<u32> = lines.iter().map(|l| seat_id(&l)).collect();
    let result = (1..1023)
        .find(|i| !set.contains(&i) && set.contains(&(i - 1)) && set.contains(&(i + 1)))
        .unwrap();
    println!("part 2: {}", result);
}

fn seat_id(seat: &str) -> u32 {
    let mut row = 0;
    for (i, c) in seat[..7].chars().enumerate() {
        if c == 'B' {
            row |= 1 << (6 - i);
        }
    }
    let mut col = 0;
    for (i, c) in seat[7..].chars().enumerate() {
        if c == 'R' {
            col |= 1 << (2 - i);
        }
    }
    row * 8 + col
}
