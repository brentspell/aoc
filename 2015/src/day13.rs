use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"^(?P<from>[\w]+?) would (?P<op>(gain|lose)) (?P<number>[\d]+?) happiness units by sitting next to (?P<to>[\w]+?).$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day13.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(&str, &str, i32)> = lines.iter().map(|l| parse(&l)).collect();
    let people: Vec<&str> = inputs
        .iter()
        .map(|&(f, _t, _d)| f)
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let costs: HashMap<(&str, &str), i32> = inputs.iter().map(|&(f, t, d)| ((f, t), d)).collect();

    // part 1
    let result = optimize(&people, &costs);
    println!("part 1: {}", result);

    // part 2
    let people: Vec<&str> = people.into_iter().chain(vec!["Brent"]).collect();
    let result = optimize(&people, &costs);
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, &str, i32) {
    let caps = REGEX.captures(line).unwrap();
    let sign = if caps.name("op").unwrap().as_str() == "gain" {
        1
    } else {
        -1
    };
    (
        caps.name("from").unwrap().as_str(),
        caps.name("to").unwrap().as_str(),
        sign * caps
            .name("number")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
    )
}

fn optimize(people: &[&str], costs: &HashMap<(&str, &str), i32>) -> i32 {
    permutohedron::Heap::new(&mut people.to_vec())
        .map(|p| {
            p.iter()
                .zip(p.iter().skip(1).chain(&p[..1]))
                .map(|(a, b)| costs.get(&(a, b)).unwrap_or(&0) + costs.get(&(b, a)).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap()
}
