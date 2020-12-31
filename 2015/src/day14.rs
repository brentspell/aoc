use regex::Regex;
use std::io::prelude::*;
use std::{cmp, fs, io};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"^(?P<name>[\w]+?) can fly (?P<speed>[\d]+?) km/s for (?P<duration>[\d]+?) seconds, but then must rest for (?P<recovery>[\d]+?) seconds.$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day14.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(&str, i32, i32, i32)> = lines.iter().map(|l| parse(&l)).collect();

    // part 1
    let mut result = 0;
    for (_name, speed, duration, rest) in inputs.iter() {
        let mut flying = true;
        let mut run = 0;
        let mut distance = 0;
        for _ in 0..2503 {
            if flying {
                distance += speed;
            }
            run += 1;
            if flying && run == *duration {
                flying = false;
                run = 0;
            } else if !flying && run == *rest {
                flying = true;
                run = 0;
            }
        }
        result = cmp::max(result, distance);
    }
    println!("part 1: {}", result);

    // part 2
    let mut flying = vec![true; inputs.len()];
    let mut run = vec![0; inputs.len()];
    let mut distance = vec![0; inputs.len()];
    let mut points = vec![0; inputs.len()];
    for _ in 0..2503 {
        for (i, (_name, speed, duration, rest)) in inputs.iter().enumerate() {
            if flying[i] {
                distance[i] += speed;
            }
            run[i] += 1;
            if flying[i] && run[i] == *duration {
                flying[i] = false;
                run[i] = 0;
            } else if !flying[i] && run[i] == *rest {
                flying[i] = true;
                run[i] = 0;
            }
        }
        let max = *distance.iter().max().unwrap();
        for i in 0..inputs.len() {
            if distance[i] == max {
                points[i] += 1;
            }
        }
    }
    let result = points.iter().max().unwrap();
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, i32, i32, i32) {
    let caps = REGEX.captures(line).unwrap();
    (
        caps.name("name").unwrap().as_str(),
        caps.name("speed").unwrap().as_str().parse::<i32>().unwrap(),
        caps.name("duration")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
        caps.name("recovery")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
    )
}
