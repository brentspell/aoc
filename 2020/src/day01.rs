use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day01.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let values: Vec<i32> = lines.iter().map(|l| l.parse().unwrap()).collect();

    // part 1
    let mut result = 0;
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] + values[j] == 2020 {
                result = values[i] * values[j];
            }
        }
    }
    println!("part 1: {}", result);

    // part 2
    let mut result = 0;
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            for k in (j + 1)..values.len() {
                if values[i] + values[j] + values[k] == 2020 {
                    result = values[i] * values[j] * values[k];
                }
            }
        }
    }
    println!("part 2: {}", result);
}
