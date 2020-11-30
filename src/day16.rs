use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day16.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<HashMap<String, i32>> = lines.iter().map(|l| parse(l)).collect();

    let constraints: HashMap<&str, i32> = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();

    // part 1
    let result = inputs
        .iter()
        .enumerate()
        .find(|(_i, aunt)| aunt.iter().all(|(k, &v)| v == constraints[k.as_str()]))
        .unwrap()
        .0
        + 1;
    println!("part 1: {}", result);

    // part 2
    let result = inputs
        .iter()
        .enumerate()
        .find(|(_i, aunt)| {
            aunt.iter().all(|(k, &v)| {
                if k == "cats" || k == "trees" {
                    v > constraints[k.as_str()]
                } else if k == "pomeranians" || k == "goldfish" {
                    v < constraints[k.as_str()]
                } else {
                    v == constraints[k.as_str()]
                }
            })
        })
        .unwrap()
        .0
        + 1;
    println!("part 2: {}", result);
}

fn parse(line: &str) -> HashMap<String, i32> {
    let line = line.splitn(2, ": ").collect::<Vec<&str>>()[1];
    line.split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|p| {
            let s: Vec<&str> = p.split(": ").collect();
            (s[0].to_string(), s[1].parse().unwrap())
        })
        .collect()
}
