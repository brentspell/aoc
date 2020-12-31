use regex::Regex;
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"^(?P<min>[\d]+?)-(?P<max>[\d]+?) (?P<char>\w): (?P<password>.*)"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day02.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(usize, usize, char, &str)> = lines.iter().map(|l| parse(&l)).collect();

    // part 1
    let mut result = 0;
    for &(min, max, ch, pw) in &inputs {
        let count = pw.matches(ch).count();
        if count >= min && count <= max {
            result += 1;
        }
    }
    println!("part 1: {}", result);

    // part 2
    let mut result = 0;
    for &(min, max, ch, pw) in &inputs {
        let chars: Vec<char> = pw.chars().collect();
        if (chars[min - 1] == ch) != (chars[max - 1] == ch) {
            result += 1;
        }
    }
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (usize, usize, char, &str) {
    let caps = REGEX.captures(line).unwrap();
    (
        caps.name("min").unwrap().as_str().parse().unwrap(),
        caps.name("max").unwrap().as_str().parse().unwrap(),
        caps.name("char").unwrap().as_str().chars().next().unwrap(),
        caps.name("password").unwrap().as_str(),
    )
}
